use core::str;
use std::{ops::Deref, sync::Arc, time::Duration};
use time::UtcDateTime;
use tokio::sync::{OnceCell, RwLock};

use async_trait::async_trait;
use serde::Deserialize;
use reqwest::{Client, IntoUrl, Method, RequestBuilder, Response};
use url::Url;
use crate::{backend::{chat::ChatMessage, llm::{Backend, Capability, Model, ModelInfo, PromptResponse, RuntimeInfo, SharedBackend, SharedBackendImpl, SharedModel, WeakBackend}, reader::ollama_reader::{OllamaPromptData, OllamaPromptReader}}, commands::process_commands::{execute, terminate}, errors::{self, Error}};

pub(crate) static OLLAMA_NAME: &'static str = "Ollama";

pub struct OllamaBackendInner {
    http_client: Client,
    api_url: Url,
    models: Vec<SharedModel>,
    self_ref: OnceCell<WeakBackend>
}

#[derive(Clone)]
pub struct OllamaBackend(pub SharedBackendImpl<OllamaBackendInner>);

impl OllamaBackend {
    pub fn new(mut api_url: Url) -> SharedBackend {
        OllamaBackendInner::prepare_api_url(&mut api_url);
        let backend: Box<dyn Backend> = Box::new(
            OllamaBackendInner {
                http_client: Client::new(),
                api_url: api_url,
                models: Vec::new(),
                self_ref: OnceCell::new()
            }
        );

        let shared_backend = Arc::new(RwLock::new(backend));
        {
            let weak_backend = Arc::downgrade(&shared_backend);
            let guard = shared_backend.blocking_write();
            let ollama = guard.as_any().downcast_ref::<OllamaBackendInner>().unwrap();
            let _ = ollama.self_ref.set(weak_backend); // Cannot fail
        }

        return shared_backend;
    }
}

impl OllamaBackendInner {
    async fn call_backend(
        &self,
        url: &str,
        method: Method,
        req_builder: impl FnOnce(RequestBuilder) -> RequestBuilder + 'static
    ) -> reqwest::Result<reqwest::Response>
    {
        let url = self.api_url.join(url).unwrap();
        let mut builder = self.http_client.request(
            method,
            url
        );
        builder = req_builder(builder);

        self.http_client.execute(builder.build()?).await
    }

    async fn call_backend_default<>(&self, url: &str) -> reqwest::Result<reqwest::Response>
    {
        self.call_backend(url, Method::GET, |r| {r}).await
    }

    pub fn set_api_url(&mut self, url: impl IntoUrl) -> Result<(), errors::Error>
    {
        let mut url = url.into_url()?;
        OllamaBackendInner::prepare_api_url(&mut url);

        self.api_url = url;
        Ok(())
    }

    /// Append trailing slash (/) if not already there
    pub fn prepare_api_url(url: &mut Url) {
        if !url.as_str().ends_with("/") {
            url.set_path(&format!("{}/", url.path()));
        }
    }

    pub fn get_api_url(&self) -> &Url
    {
        &self.api_url
    }
}

impl Deref for OllamaBackend {
    type Target = SharedBackendImpl<OllamaBackendInner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize, Debug)]
struct ModelResInner {
    name: String,
    model: String,
    size: u64
}

#[derive(Deserialize, Debug)]
struct ModelResponse<T> {
    models: Vec<T>
}

#[derive(Deserialize)]
struct ModelDetail {
    capabilities: Vec<Capability>
}

#[async_trait]
impl Backend for OllamaBackendInner {
    fn name(&self) -> &str {
        OLLAMA_NAME
    }

    fn models(&self) -> &[SharedModel] {
        &self.models
    }

    async fn update_models(&mut self) -> Result<(), errors::Error> {
        let res = self.call_backend_default("tags").await?;
        let model_json: ModelResponse<ModelResInner> = res.json().await?;
        let mut models: Vec<SharedModel> = Vec::with_capacity(model_json.models.len());

        for m in model_json.models {
            let model_name = m.name.clone();
            let detail_res: ModelDetail = self.call_backend(
                "show",
                Method::POST,
                move |req| {
                    req.json(&serde_json::json!({"model": model_name}))
                }
            ).await?.json().await?;

            models.push(Arc::new(RwLock::new(Box::new(
                OllamaModel {
                    info: ModelInfo {
                        name: m.name,
                        id: m.model,
                        size: m.size,
                        capabilities: detail_res.capabilities,
                    },
                    backend: self.self_ref.get().ok_or(Error::Unknown)?.upgrade().ok_or(Error::Unknown)?,
                    runtime_info: RwLock::new(None)
                }
            ))));
        }
        
        self.models = models;
        Ok(())
    }

    async fn get_running_models(&self) -> Result<Vec<RuntimeInfo>, errors::Error> {
        let res = self.call_backend_default("ps").await?;
        let models: ModelResponse<RuntimeInfo> = res.json().await?;
        Ok(models.models.into_iter().map(|m| m).collect())
    }

    async fn running(&self) -> bool {
        let res = self.call_backend(
            "version",
            Method::HEAD,
            |req| {
                req.header("Cache", "no-store")
                   .timeout(Duration::from_secs(2))
            }
        ).await;

        match res {
            Ok(res) => res.status().is_success(),
            Err(_) => false
        }
    }

    async fn boot(&self) -> Result<(), errors::Error> {
        let res = execute("ollama app", vec![]).await?;
        if !res.status.success() {
            return Err(Error::BackendBoot {
                backend: self.name().to_owned(),
                reason: str::from_utf8(&res.stderr).unwrap_or("Unknown").to_owned()
            });
        }

        const TRIES: u32 = 3;
        // Poll for Ollama boot
        for _ in 0..TRIES {
            if self.running().await {
                return Ok(())
            }
            tokio::time::sleep(Duration::from_secs(2000)).await;
        }
        Err(Error::BackendBoot {
            backend: self.name().to_owned(),
            reason: format!("Ollama did not start after {TRIES} tries")
        })
    }

    async fn shutdown(&self) -> Result<(), errors::Error> {
        terminate("ollama app").await.map(|_| ())
    }
}

pub struct OllamaModel {
    info: ModelInfo,
    backend: SharedBackend,
    runtime_info: RwLock<Option<RuntimeInfo>>
}

#[async_trait]
impl Model for OllamaModel {
    fn info(&self) -> &ModelInfo {
        &self.info
    }
    fn backend(&self) -> SharedBackend {
        self.backend.clone()
    }

    async fn loaded(&self) -> Result<bool, Error> {
        let _ = self.get_runtime_info().await?;
        Ok(self.runtime_info.read().await.is_some())
    }

    async fn get_loaded_size(&self) -> Result<i64, Error> {
        let _ = self.get_runtime_info().await?;
        Ok(self.runtime_info.read().await.as_ref()
            .and_then(|info| Some(info.size_vram))
            .unwrap_or(-1))
    }

    async fn get_runtime_info(&self) -> Result<Option<RuntimeInfo>, errors::Error> {
        let current_time = UtcDateTime::now();

        if let Some(rt) = &*self.runtime_info.read().await {
            if rt.expires_at > current_time {
                return Ok(Some(rt.clone()));
            }
        }

        let backend = self.backend.read().await;
        let mut runtime_infos: Vec<RuntimeInfo> = backend.get_running_models().await?;
        let mut runtime_info = self.runtime_info.write().await;

        let runtime_info_idx = runtime_infos.iter().position(|r| r.name == self.info.name);
        if let Some(runtime_info_idx) = runtime_info_idx {
            let rt = runtime_infos.swap_remove(runtime_info_idx);
            *runtime_info = Some(rt.clone());
            Ok(Some(rt))
        } else {
            Ok(None)
        }
    }

    async fn load(&mut self) -> Result<(), Error> {
        // Ollama models cannot be loaded manually 
        Ok(())
    }

    async fn unload(&mut self) -> Result<(), Error> {
        // Ollama models cannot be unloaded manually
        Ok(())
    }

    async fn prompt(
        &self,
        content: ChatMessage,
        history: &[ChatMessage],
        think: Option<bool>
    ) -> Result<Box<dyn PromptResponse>, Error>
    {
        let mut res: Response;
        {
            let model_name = self.info.id.clone();
            let mut messages: Vec<ChatMessage> = Vec::with_capacity(history.len()+1);
            history.iter().for_each(|msg| messages.push(msg.clone()));
            messages.push(content);

            let backend = self.backend.read().await;
            let ollama = backend
                .as_any()
                .downcast_ref::<OllamaBackendInner>()
                .ok_or(Error::Internal("Could not get Ollama backend".into()))?;

            res = ollama.call_backend(
                "chat",
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

        let reader = OllamaPromptReader::new();
        let sink = reader.data_intake();

        tokio::spawn(async move {
            while let Ok(chunk) = res.chunk().await {
                if let Some(chunk) = chunk {
                    if let Err(_) = sink.send(OllamaPromptData::Data(chunk)).await {
                        break;
                    }
                } else {
                    // We don't remove the data from the stream using "chunk".
                    // So if we don't have any more data, we need to break manually
                    break;
                }
            }
            let _ = sink.send(OllamaPromptData::End).await;
        });
        
        Ok(Box::new(reader))
    }
}
