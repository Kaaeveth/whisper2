use uuid::Uuid;
use time::Date;
use serde::Serialize;

#[derive(Serialize)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool
}

#[derive(Serialize)]
pub struct ChatMessage {
    role: Role,
    content: String,
    images: Option<Vec<String>>,
    thoughts: Option<String>
}

#[derive(Serialize)]
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
