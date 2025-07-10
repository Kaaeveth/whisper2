use std::any::Any;
use std::sync::{Arc, Weak};
use time::UtcDateTime;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Receiver;
use std::boxed::Box;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::backend::chat::{ChatMessage, ChatResponse, parse_utc_datetime, serialize_utc_datetime};
use crate::errors::Error;

pub type SharedModel = Arc<RwLock<Box<dyn Model>>>;
pub type SharedBackend = Arc<RwLock<Box<dyn Backend>>>;
pub type WeakBackend = Weak<RwLock<Box<dyn Backend>>>;

pub type SharedBackendImpl<T> = Arc<RwLock<Box<T>>>;

/// A backend hosting large language models, which
/// manages multiple [Model]s.
#[async_trait]
pub trait Backend: Send + Sync + Any {
    /// Name of the backend.
    /// Should be a unique identifier and is used
    /// in tauri commands for fetching the backend.
    fn name(&self) -> &str;

    /// The currently loaded models.
    /// Needs to be updated manually using [update_models].
    fn models(&self) -> &[SharedModel];

    /// Updates the available models.
    /// These can be retrieved using [models].
    async fn update_models(&mut self) -> Result<(), Error>;

    /// Returns runtime information regarding currently
    /// loaded models. A model may be available but not loaded.
    async fn get_running_models(&self) -> Result<Vec<RuntimeInfo>, Error>;

    /// Whether the backend is running.
    /// Other methods of this trait may fail if the
    /// backend is not running.
    async fn running(&self) -> bool;

    /// Starts the backend.
    async fn boot(&self) -> Result<(), Error>;

    /// Stops the backend.
    async fn shutdown(&self) -> Result<(), Error>;
}

impl dyn Backend {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Capabilities of a model.
/// This is not a complete list and we currently
/// don't support all of those functionalities.
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    Completion,
    Vision,
    Tools,
    Thinking
}

/// A token (or chunk) of a chat completion.
/// If [PromptEvent::Stop], then [ChatResponse.done]
/// should also be true.
/// [PromptEvent::Stop] is mainly used for cancellation
/// of ongoing chat completions.
/// TODO: Add stop reason or error variant
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
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

/// Information about a loaded model.
/// See [ModelInfo] for general information.
#[derive(Clone, Deserialize, Serialize)]
pub struct RuntimeInfo {
    pub size_vram: i64,
    #[serde(deserialize_with = "parse_utc_datetime", serialize_with = "serialize_utc_datetime")]
    pub expires_at: UtcDateTime,
    pub name: String
}

pub trait PromptResponse: Send + Sync {
    /// Sink for receiving generated tokens.
    /// If None, then the Receiver was already taken.
    fn get_prompts(&mut self) -> Option<Receiver<PromptEvent>>;

    /// Stops an ongoing chat completion.
    /// Has no effect if already completed.
    fn abort(&self);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ModelInfo {
    /// Human readable name of the model
    /// and also identifier.
    pub name: String,
    /// Unique ID of the model.
    /// Is usually the same as [name]
    pub id: String,
    /// Disk size of the model in bytes.
    pub size: u64,
    /// Additional functionality of the model.
    pub capabilities: Vec<Capability>
}

/// A large language model (LLM).
#[async_trait]
pub trait Model: Send + Sync {
    fn info(&self) -> &ModelInfo;

    /// The corresponding backend
    /// of the model.
    #[allow(dead_code)]
    fn backend(&self) -> SharedBackend;

    /// Whether the model is loaded
    async fn loaded(&self) -> Result<bool, Error>;

    /// Size in bytes of the model in RAM or VRAM
    async fn get_loaded_size(&self) -> Result<i64, Error>;
    async fn get_runtime_info(&self) -> Result<Option<RuntimeInfo>, Error>;

    /// Loads the model.
    /// Makes the model ready for use.
    /// Note that upon calling [prompt], the model
    /// will be loaded if necessary.
    async fn load(&mut self) -> Result<(), Error>;

    /// Unloads a model.
    async fn unload(&mut self) -> Result<(), Error>;

    /// Starts a streaming chat completion.
    /// This will [load] the model if not already
    /// done so.
    /// An ongoing completion may be stopped using
    /// [PromptResponse.abort]
    async fn prompt(
        &self,
        content: ChatMessage,
        history: &[ChatMessage],
        think: Option<bool>
    ) -> Result<Box<dyn PromptResponse>, Error>;
}
