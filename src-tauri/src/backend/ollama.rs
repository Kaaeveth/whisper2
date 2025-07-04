use std::{ops::Deref, sync::Arc, time::Duration};
use tokio::sync::Mutex;

use async_trait::async_trait;
use serde::Deserialize;
use tauri::ipc::Channel;
use reqwest::{Client, IntoUrl, Method, RequestBuilder};
use url::Url;
use crate::{backend::{chat::ChatMessage, llm::{Backend, Capability, Model, PromptEvent, SharedBackend, SharedBackendImpl, SharedModel}}, commands::process_commands::{execute, terminate}, errors};


pub struct OllamaBackendInner {
    http_client: Client,
    api_url: Url,
    models: Vec<SharedModel>,
    create_clone: Box<dyn Fn() -> Option<SharedBackend> + Send + Sync>
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
                create_clone: Box::new(|| None)
            }
        );

        let shared_backend = Arc::new(Mutex::new(backend));
        let b2 = shared_backend.clone();
        shared_backend.blocking_lock().set_clone_fn(Box::new(move || {
            Some(b2.clone())
        }));

        return shared_backend;
    }
}

impl OllamaBackendInner {
    async fn call_backend(
        &self,
        url: impl IntoUrl,
        method: Option<Method>,
        req_builder: impl FnOnce(RequestBuilder) -> RequestBuilder + 'static
    ) -> reqwest::Result<reqwest::Response>
    {
        let url = self.api_url.join(url.into_url()?.as_str()).unwrap();
        let mut builder = self.http_client.request(
            method.unwrap_or(Method::GET),
            url
        );
        builder = req_builder(builder);

        self.http_client.execute(builder.build()?).await
    }

    async fn call_backend_default<>(&self, url: impl IntoUrl) -> reqwest::Result<reqwest::Response>
    {
        self.call_backend(url, None, |r| {r}).await
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

    fn set_clone_fn(&mut self, clone: Box<dyn Fn() -> Option<SharedBackend> + Send + Sync>) {
        self.create_clone = clone;
    }

    async fn update_models(&mut self) -> Result<(), errors::Error> {
        let res = self.call_backend_default("/tags").await?;
        let model_json: Vec<ModelRes> = res.json().await?;
        let mut models: Vec<SharedModel> = Vec::with_capacity(model_json.len());

        for m in model_json {
            let model_name = m.name.clone();
            let detail_res: ModelDetail = self.call_backend(
                "/show",
                Some(Method::POST),
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
                    backend: (self.create_clone)().unwrap()
                }
            ))));
        }
        
        self.models = models;
        Ok(())
    }

    async fn running(&self) -> bool {
        let res = self.call_backend(
            "",
            Some(Method::HEAD),
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
    backend: SharedBackend
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
        todo!();
    }
    async fn get_loaded_size(&self) -> i32 {
        todo!();
    }

    async fn load(&mut self) {
        todo!();
    }
    async fn unload(&mut self) {
        todo!();
    }

    fn prompt(
        &self,
        content: &ChatMessage,
        history: &[ChatMessage],
        think: Option<bool>
    ) -> Channel<PromptEvent> {
        todo!();
    }
}