use std::{ops::Deref, sync::{atomic::AtomicBool, Arc, RwLock}, time::{Duration, SystemTime}};
use tokio::sync::{Mutex, OnceCell};

use async_trait::async_trait;
use serde::Deserialize;
use tauri::ipc::{Channel, InvokeResponseBody};
use reqwest::{Client, IntoUrl, Method, RequestBuilder, Response};
use url::Url;
use crate::{backend::{chat::ChatMessage, llm::{Backend, Capability, Model, PromptEvent, RuntimeInfo, SharedBackend, SharedBackendImpl, SharedModel, WeakBackend}}, commands::process_commands::{execute, terminate}, errors::{self, Error}};

pub struct OllamaBackendInner {
    http_client: Client,
    api_url: Url,
    models: Vec<SharedModel>,
    self_ref: OnceCell<WeakBackend>
}

#[derive(Clone)]
pub struct OllamaBackend(pub SharedBackendImpl<OllamaBackendInner>);

impl OllamaBackend {
    pub fn new() -> SharedBackend {
        let backend: Box<dyn Backend> = Box::new(
            OllamaBackendInner {
                http_client: Client::new(),
                api_url: Url::parse("http://localhost:11434/api").unwrap(),
                models: Vec::new(),
                self_ref: OnceCell::new()
            }
        );

        let shared_backend = Arc::new(Mutex::new(backend));
        {
            let weak_backend = Arc::downgrade(&shared_backend);
            let guard = shared_backend.blocking_lock();
            let ollama = guard.as_any().downcast_ref::<OllamaBackendInner>().unwrap();
            let _ = ollama.self_ref.set(weak_backend); // Cannot fail
        }

        return shared_backend;
    }
}

impl OllamaBackendInner {
    async fn call_backend(
        &self,
        url: impl IntoUrl,
        method: Method,
        req_builder: impl FnOnce(RequestBuilder) -> RequestBuilder + 'static
    ) -> reqwest::Result<reqwest::Response>
    {
        let url = self.api_url.join(url.into_url()?.as_str()).unwrap();
        let mut builder = self.http_client.request(
            method,
            url
        );
        builder = req_builder(builder);

        self.http_client.execute(builder.build()?).await
    }

    async fn call_backend_default<>(&self, url: impl IntoUrl) -> reqwest::Result<reqwest::Response>
    {
        self.call_backend(url, Method::GET, |r| {r}).await
    }
}

impl Deref for OllamaBackend {
    type Target = SharedBackendImpl<OllamaBackendInner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize)]
struct ModelRes {
    name: String,
    id: String,
    size: i32
}

#[derive(Deserialize)]
struct ModelDetail {
    capabilities: Vec<Capability>
}

#[async_trait]
impl Backend for OllamaBackendInner {
    fn name(&self) -> &str {
        "Ollama"
    }

    fn models(&self) -> &[SharedModel] {
        &self.models
    }

    async fn update_models(&mut self) -> Result<(), errors::Error> {
        let res = self.call_backend_default("/tags").await?;
        let model_json: Vec<ModelRes> = res.json().await?;
        let mut models: Vec<SharedModel> = Vec::with_capacity(model_json.len());

        for m in model_json {
            let model_name = m.name.clone();
            let detail_res: ModelDetail = self.call_backend(
                "/show",
                Method::POST,
                move |req| {
                    req.json(&serde_json::json!({"model": model_name}))
                }
            ).await?.json().await?;

            models.push(Arc::new(Mutex::new(Box::new(
                OllamaModel {
                    name: m.name,
                    id: m.id,
                    size: m.size,
                    capabilities: detail_res.capabilities,
                    backend: self.self_ref.get().ok_or(Error::Unknown)?.upgrade().ok_or(Error::Unknown)?,
                    runtime_info: RwLock::new(None)
                }
            ))));
        }
        
        self.models = models;
        Ok(())
    }

    async fn get_running_models(&self) -> Result<Vec<RuntimeInfo>, errors::Error> {
        let res = self.call_backend_default("/ps").await?;
        let models: Vec<RuntimeInfo> = res.json().await?;
        Ok(models)
    }

    async fn running(&self) -> bool {
        let res = self.call_backend(
            "",
            Method::HEAD,
            |req| {
                req.header("Cachec", "no-store")
                   .timeout(Duration::from_secs(2))
            }
        ).await;

        match res {
            Ok(res) => res.status().is_success(),
            Err(_) => false
        }
    }

    async fn boot(&self) -> Result<(), errors::Error> {
        execute("ollama app", vec!["serve"]).await.map(|_| ())
    }

    async fn shutdown(&self) -> Result<(), errors::Error> {
        terminate("ollama app").await.map(|_| ())
    }
}

pub struct OllamaModel {
    name: String,
    id: String,
    size: i32,
    capabilities: Vec<Capability>,
    backend: SharedBackend,
    runtime_info: RwLock<Option<RuntimeInfo>>
}

#[async_trait]
impl Model for OllamaModel {
    fn name(&self) -> &str {
        &self.name
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn size(&self) -> i32 {
        self.size
    }
    fn capabilities(&self) -> &[Capability] {
        &self.capabilities
    }
    fn backend(&self) -> SharedBackend {
        self.backend.clone()
    }

    async fn loaded(&self) -> bool {
        self.runtime_info.read().unwrap().is_some()
    }

    async fn get_loaded_size(&self) -> i32 {
        self.runtime_info.read().unwrap().as_ref()
            .and_then(|info| Some(info.size_vram))
            .unwrap_or(-1)
    }

    async fn get_runtime_info(&self) -> Result<Option<RuntimeInfo>, errors::Error> {
        let current_time = SystemTime::now();

        if let Some(rt) = &*self.runtime_info.read().unwrap() {
            if rt.expires_at > current_time {
                return Ok(Some(rt.clone()));
            }
        }

        let backend = self.backend.lock().await;
        let mut runtime_infos: Vec<RuntimeInfo> = backend.get_running_models().await?;
        let mut runtime_info = self.runtime_info.write().unwrap();

        let runtime_info_idx = runtime_infos.iter().position(|r| r.model_name == self.name);
        if let Some(runtime_info_idx) = runtime_info_idx {
            let rt = runtime_infos.swap_remove(runtime_info_idx);
            *runtime_info = Some(rt.clone());
            Ok(Some(rt.clone()))
        } else {
            Ok(None)
        }
    }

    async fn load(&mut self) {
        // Ollama models cannot be loaded manually 
    }

    async fn unload(&mut self) {
        // Ollama models cannot be unloaded manually
    }

    async fn prompt(
        &self,
        content: &ChatMessage,
        history: &[ChatMessage],
        think: Option<bool>
    ) -> Result<Channel<PromptEvent>, Error>
    {
        let mut res: Response;
        {
            let model_name = self.id.clone();
            let mut messages: Vec<ChatMessage> = Vec::with_capacity(history.len()+1);
            history.iter().for_each(|msg| messages.push(msg.clone()));
            messages.push(content.clone());

            let backend = self.backend.blocking_lock();
            let ollama = backend
                .as_any()
                .downcast_ref::<OllamaBackendInner>()
                .ok_or(Error::Internal("Could not get Ollama backend".into()))?;

            res = ollama.call_backend(
                "/chat",
                Method::POST,
                move |req| {
                    req.json(&serde_json::json!({
                        "model": model_name,
                        "keep_alive": "10m",
                        "think": think.unwrap_or(false),
                        "messages": messages
                    }))
                }
            ).await?;
        }

        let abort = Arc::new(AtomicBool::new(false));
        let send_abort = abort.clone();
        let on_message = move |event: InvokeResponseBody| {
            let msg = event.deserialize::<PromptEvent>()?;
            if let PromptEvent::Stop = msg {
                send_abort.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            Ok(())
        };

        let channel: Channel<PromptEvent> = Channel::new(on_message);
        let producer_channel = channel.clone();

        tokio::spawn(async move {
            while let Some(chunk) = res.chunk().await.unwrap() {
                if abort.load(std::sync::atomic::Ordering::Relaxed) {
                    let _ = producer_channel.send(PromptEvent::Stop);
                    break;
                }


            }
            let _ = producer_channel.send(PromptEvent::Stop);
        });
        
        Ok(channel)
    }
}