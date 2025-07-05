use std::any::Any;
use std::sync::{Arc, Weak};
use std::time::SystemTime;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Sender, Receiver};
use std::boxed::Box;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::backend::chat::{ChatMessage, ChatResponse};
use crate::errors::Error;

// NOTE: Replace Mutex with RwLock?
pub type SharedModel = Arc<Mutex<Box<dyn Model>>>;
pub type SharedBackend = Arc<Mutex<Box<dyn Backend>>>;
pub type WeakBackend = Weak<Mutex<Box<dyn Backend>>>;

pub type SharedBackendImpl<T> = Arc<Mutex<Box<T>>>;

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

#[derive(Serialize, Deserialize)]
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

#[derive(Clone, Deserialize)]
pub struct RuntimeInfo {
    pub size_vram: i32,
    pub expires_at: SystemTime,
    pub model_name: String
}

pub trait PromptResponse {
    fn get_prompts(&self) -> &Receiver<PromptEvent>;
    fn get_control(&mut self) -> Sender<PromptEvent>;
}

#[async_trait]
pub trait Model: Send + Sync {
    fn name(&self) -> &str;
    fn id(&self) -> &str;
    fn size(&self) -> i32;
    fn capabilities(&self) -> &[Capability];
    fn backend(&self) -> SharedBackend;

    async fn loaded(&self) -> bool;
    async fn get_loaded_size(&self) -> i32;
    async fn get_runtime_info(&self) -> Result<Option<RuntimeInfo>, Error>;

    async fn load(&mut self);
    async fn unload(&mut self);

    async fn prompt(
        &self,
        content: &ChatMessage,
        history: &[ChatMessage],
        think: Option<bool>
    ) -> Result<Box<dyn PromptResponse>, Error>;
}
