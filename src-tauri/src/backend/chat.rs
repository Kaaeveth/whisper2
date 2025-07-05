use std::sync::Arc;

use uuid::Uuid;
use time::Date;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool
}

#[derive(Serialize, Clone, Deserialize)]
pub struct ChatMessageInner {
    role: Role,
    content: String,
    images: Option<Vec<String>>,
    thoughts: Option<String>
}

#[derive(Serialize, Clone, Deserialize)]
pub struct ChatMessage {
    #[serde(flatten)]
    inner: Arc<ChatMessageInner>   
}

#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    done: bool,
    message: ChatMessage
}

#[derive(Serialize)]
pub struct Chat {
    uuid: Uuid,
    title: String,
    history: Vec<ChatMessage>,
    create_at: Date
}
