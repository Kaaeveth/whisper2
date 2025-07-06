use std::any::Any;
use std::sync::{Arc, Weak};
use std::time::SystemTime;
use tokio::sync::RwLock;
use tokio::sync::mpsc::{Sender, Receiver};
use std::boxed::Box;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::backend::chat::{ChatMessage, ChatResponse};
use crate::errors::Error;

pub type SharedModel = Arc<RwLock<Box<dyn Model>>>;
pub type SharedBackend = Arc<RwLock<Box<dyn Backend>>>;
pub type WeakBackend = Weak<RwLock<Box<dyn Backend>>>;

pub type SharedBackendImpl<T> = Arc<RwLock<Box<T>>>;

#[async_trait]
pub trait Backend: Send + Sync + Any {
    fn name(&self) -> &str;
    fn models(&self) -> &[SharedModel];

    async fn update_models(&mut self) -> Result<(), Error>;
    async fn get_running_models(&self) -> Result<Vec<RuntimeInfo>, Error>;
    async fn running(&self) -> bool;

    async fn boot(&self) -> Result<(), Error>;
    async fn shutdown(&self) -> Result<(), Error>;
}

impl dyn Backend {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Capability {
    Completion,
    Vision,
    Tools
}

#[derive(Serialize, Deserialize)]
pub enum PromptEvent {
    Message(ChatResponse),
    Stop
}

impl PromptEvent {
    pub fn is_stop(&self) -> bool {
        if let Self::Stop = &self {
            return true;
        }
        false
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct RuntimeInfo {
    pub size_vram: i32,
    pub expires_at: SystemTime,
    pub model_name: String
}

pub trait PromptResponse: Send {
    fn get_prompts(&mut self) -> Receiver<PromptEvent>;
    fn get_control(&self) -> Sender<PromptEvent>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub id: String,
    pub size: i32,
    pub capabilities: Vec<Capability>
}

#[async_trait]
pub trait Model: Send + Sync {
    fn info(&self) -> &ModelInfo;
    #[allow(dead_code)]
    fn backend(&self) -> SharedBackend;

    async fn loaded(&self) -> Result<bool, Error>;
    async fn get_loaded_size(&self) -> Result<i32, Error>;
    async fn get_runtime_info(&self) -> Result<Option<RuntimeInfo>, Error>;

    async fn load(&mut self) -> Result<(), Error>;
    async fn unload(&mut self) -> Result<(), Error>;

    async fn prompt(
        &self,
        content: &ChatMessage,
        history: &[ChatMessage],
        think: Option<bool>
    ) -> Result<Box<dyn PromptResponse>, Error>;
}
