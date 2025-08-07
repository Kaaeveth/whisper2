use core::str;
use std::{
    ops::Deref, path::{Path, PathBuf}, process::{Child, Command}, sync::Arc, time::Duration
};
use time::UtcDateTime;
use tokio::{runtime::Handle, sync::RwLock};
use log::{info, error};

use crate::{
    backend::{
        chat::ChatMessage,
        llm::{
            Backend, Capability, Model, ModelInfo, PromptResponse, RuntimeInfo, SharedBackend,
            SharedBackendImpl, SharedModel, WeakBackend,
        },
        reader::ndjson_reader::NdJsonReader,
    },
    errors::{self, Error},
};
use async_trait::async_trait;
use reqwest::{Client, IntoUrl, Method, RequestBuilder, Response};
use serde::Deserialize;
use url::Url;

pub(crate) static OLLAMA_NAME: &'static str = "Ollama";

pub(crate) fn not_ollama() -> errors::Error {
    errors::internal("Backend is not Ollama")
}

pub struct OllamaBackend {
    http_client: Client,
    api_url: Url,
    models: Vec<SharedModel>,
    self_ref: WeakBackend<OllamaBackend>,
    ollama_proc: Option<Child>,
    models_path: Option<PathBuf>
}

#[derive(Clone)]
pub struct SharedOllamaBackend(pub SharedBackendImpl<OllamaBackend>);

impl SharedOllamaBackend {
    pub fn new(mut api_url: Url, models_path: Option<PathBuf>) -> SharedBackend {
        OllamaBackend::prepare_api_url(&mut api_url);

        Arc::new_cyclic(|me| {
            RwLock::new(OllamaBackend {
                http_client: Client::new(),
                api_url: api_url,
                models: Vec::new(),
                self_ref: me.clone(),
                ollama_proc: None,
                models_path: models_path
            })
        })
    }
}

impl OllamaBackend {
    async fn call_backend(
        &self,
        url: &str,
        method: Method,
        req_builder: impl FnOnce(RequestBuilder) -> RequestBuilder + 'static,
    ) -> reqwest::Result<reqwest::Response> {
        let url = self.api_url.join(url).unwrap();
        let mut builder = self.http_client.request(method, url);
        builder = req_builder(builder);

        self.http_client.execute(builder.build()?).await
    }

    async fn call_backend_default(&self, url: &str) -> reqwest::Result<reqwest::Response> {
        self.call_backend(url, Method::GET, |r| r).await
    }

    pub fn set_api_url(&mut self, url: impl IntoUrl) -> Result<(), errors::Error> {
        let mut url = url.into_url()?;
        OllamaBackend::prepare_api_url(&mut url);

        self.api_url = url;
        Ok(())
    }

    pub fn get_models_path(&self) -> Option<&Path> {
        self.models_path.as_ref().and_then(|p| Some(p.as_path()))
    }

    /// Sets the path where Ollama searches for models.
    /// This method will attempt restart Ollama since
    /// the path cannot be after start.
    pub async fn set_models_path(&mut self, path: &Path) -> Result<(), errors::Error> {
        self.models_path = Some(path.to_path_buf());
        self.shutdown().await?;
        self.boot().await?;
        Ok(())
    }

    /// Append trailing slash (/) if not already there
    pub fn prepare_api_url(url: &mut Url) {
        if !url.as_str().ends_with("/") {
            url.set_path(&format!("{}/", url.path()));
        }
    }

    pub fn get_api_url(&self) -> &Url {
        &self.api_url
    }
}

impl Deref for SharedOllamaBackend {
    type Target = SharedBackendImpl<OllamaBackend>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize, Debug)]
struct ModelResInner {
    name: String,
    model: String,
    size: u64,
}

#[derive(Deserialize, Debug)]
struct ModelResponse<T> {
    models: Vec<T>,
}

#[derive(Deserialize)]
struct ModelDetail {
    capabilities: Vec<Capability>,
}

#[async_trait]
impl Backend for OllamaBackend {
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
            let detail_res: ModelDetail = self
                .call_backend("show", Method::POST, move |req| {
                    req.json(&serde_json::json!({"model": model_name}))
                })
                .await?
                .json()
                .await?;

            // A backend doesn't support embedding directly.
            if detail_res.capabilities.contains(&Capability::Embedding) {
                continue;
            }

            models.push(Arc::new(RwLock::new(OllamaModel {
                info: ModelInfo {
                    name: m.name,
                    id: m.model,
                    size: m.size,
                    capabilities: detail_res.capabilities,
                },
                backend: self.self_ref.clone(),
                runtime_info: RwLock::new(None),
            })));
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
        let res = self
            .call_backend("version", Method::HEAD, |req| {
                req.header("Cache", "no-store")
                    .timeout(Duration::from_secs(2))
            })
            .await;

        match res {
            Ok(res) => res.status().is_success(),
            Err(_) => false,
        }
    }

    async fn boot(&mut self) -> Result<(), errors::Error> {
        if self.running().await {
            info!("Boot - Ollama is already running");
            return Ok(());
        }
        // Ollama may not be responding and must be killed first
        self.shutdown().await?;

        info!("Booting Ollama");
        let mut proc = Command::new("ollama");
        proc.arg("serve");
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            proc.creation_flags(0x08000000); // Hide cmd window on Windows
        }

        if let Some(path) = &self.models_path {
            info!("OLLAMA_MODELS directory: {:?}", &path);
            if !path.exists() {
                return Err(Error::BackendBoot {
                    reason: format!("Models directory '{:?}' does not exist", &path),
                    backend: self.name().to_owned()
                });
            }
            proc.env("OLLAMA_MODELS", path.to_str().ok_or(errors::internal("Invalid Ollama models path"))?);
        }

        self.ollama_proc = Some(
            proc
                .spawn()
                .map_err(|e| Error::BackendBoot {
                    reason: format!("{:?}", e),
                    backend: self.name().to_owned(),
                })?,
        );

        const TRIES: u32 = 3;
        // Poll for Ollama boot
        for i in 0..TRIES {
            if self.running().await {
                info!("Ollama booted after {} tries", i+1);
                return Ok(());
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
        Err(Error::BackendBoot {
            backend: self.name().to_owned(),
            reason: format!("Ollama did not start after {TRIES} tries"),
        })
    }

    async fn shutdown(&mut self) -> Result<(), errors::Error> {
        self.models.clear();
        if let Some(mut proc) = self.ollama_proc.take() {
            info!("Shutting down Ollama");
            if let Err(e) = proc.kill() {
                return Err(Error::BackendBoot {
                    reason: format!("Failed to kill Ollama: {:?}", e),
                    backend: self.name().to_owned(),
                });
            }
            info!("Ollama shutdown");
        }
        Ok(())
    }
}

impl Drop for OllamaBackend {
    fn drop(&mut self) {
        Handle::current().block_on(async {
            if let Err(e) = self.shutdown().await {
                error!("{:?}", e);
            }
        });
    }
}

pub struct OllamaModel {
    info: ModelInfo,
    backend: WeakBackend<OllamaBackend>,
    runtime_info: RwLock<Option<RuntimeInfo>>,
}

impl OllamaModel {
    /// Gets a strong reference to the backend.
    /// Fails if the backend has already been dropped.
    fn access_backend(&self) -> Result<Arc<RwLock<OllamaBackend>>, Error> {
        self.backend
            .upgrade()
            .ok_or(errors::internal("Backend is already disposed"))
    }

    async fn load_model(&self, unload: bool) -> Result<(), Error> {
        let strong_backend = self.access_backend()?;
        let backend = strong_backend.read().await;

        let model_name = self.info.name.clone();
        let res = backend
            .call_backend("generate", Method::POST, move |req| {
                req.json(&serde_json::json!({
                    "model": model_name,
                    "keep_alive": if unload {Some(0)} else {None}
                }))
            })
            .await?;
        let _ = res.error_for_status()?;
        Ok(())
    }
}

#[async_trait]
impl Model for OllamaModel {
    fn info(&self) -> &ModelInfo {
        &self.info
    }

    fn backend(&self) -> Option<SharedBackend> {
        self.backend
            .upgrade()
            .map(|strong| strong as Arc<RwLock<dyn Backend>>)
    }

    async fn loaded(&self) -> Result<bool, Error> {
        let _ = self.get_runtime_info().await?;
        Ok(self.runtime_info.read().await.is_some())
    }

    async fn get_loaded_size(&self) -> Result<i64, Error> {
        let _ = self.get_runtime_info().await?;
        Ok(self
            .runtime_info
            .read()
            .await
            .as_ref()
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

        let strong_backend = self.access_backend()?;
        let backend = strong_backend.read().await;
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
        self.load_model(false).await
    }

    async fn unload(&mut self) -> Result<(), Error> {
        self.load_model(true).await
    }

    async fn prompt(
        &self,
        content: ChatMessage,
        history: &[ChatMessage],
        think: Option<bool>,
    ) -> Result<Box<dyn PromptResponse>, Error> {
        let res: Response = {
            let model_name = self.info.id.clone();
            let mut messages: Vec<ChatMessage> = Vec::with_capacity(history.len() + 1);
            history.iter().for_each(|msg| messages.push(msg.clone()));
            messages.push(content);

            let strong_backend = self.access_backend()?;
            let backend = strong_backend.read().await;

            backend
                .call_backend("chat", Method::POST, move |req| {
                    req.json(&serde_json::json!({
                        "model": model_name,
                        "keep_alive": "10m",
                        "think": think.unwrap_or(false),
                        "messages": messages
                    }))
                })
                .await?
        };

        let reader = NdJsonReader::new();
        reader.start_reading_response(res);

        Ok(Box::new(reader))
    }
}
