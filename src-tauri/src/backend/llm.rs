use std::sync::Arc;
use tokio::sync::Mutex;
use std::boxed::Box;
use tauri::ipc::Channel;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::backend::chat::{ChatMessage, ChatResponse};
use crate::errors;

pub type SharedModel = Arc<Mutex<Box<dyn Model>>>;
pub type SharedBackend = Arc<Mutex<Box<dyn Backend>>>;

pub type SharedBackendImpl<T> = Arc<Mutex<Box<T>>>;

#[async_trait]
pub trait Backend: Send + Sync {
    fn name(&self) -> &str;

    async fn update_models(&mut self) -> Result<(), errors::Error>;
    async fn running(&self) -> bool;

    async fn boot(&self) -> Result<(), errors::Error>;
    async fn shutdown(&self) -> Result<(), errors::Error>;

    fn set_clone_fn(&mut self, clone: Box<dyn Fn() -> Option<SharedBackend> + Send + Sync>);
}

#[derive(Serialize, Deserialize)]
pub enum Capability {
    Completion,
    Vision,
    Tools
}

#[derive(Serialize)]
pub enum PromptEvent {
    Message(ChatResponse),
    Abort
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

    async fn load(&mut self);
    async fn unload(&mut self);

    fn prompt(
        &self,
        content: &ChatMessage,
        history: &[ChatMessage],
        think: Option<bool>
    ) -> Channel<PromptEvent>;
}
