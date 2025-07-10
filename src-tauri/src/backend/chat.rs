use std::sync::Arc;

use uuid::Uuid;
use time::{format_description::well_known::Rfc3339, UtcDateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::errors;

#[derive(Serialize, Clone, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ChatMessageInner {
    role: Role,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thoughts: Option<String>
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ChatMessage {
    #[serde(flatten)]
    inner: Arc<ChatMessageInner>   
}

#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    done: bool,
    message: ChatMessage
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub uuid: Uuid,
    pub title: String,
    pub history: Vec<ChatMessage>,
    #[serde(deserialize_with = "parse_utc_datetime", serialize_with = "serialize_utc_datetime")]
    pub created_at: UtcDateTime
}

pub fn parse_utc_datetime<'de, D>(deserializer: D) -> Result<UtcDateTime, D::Error>
where D: Deserializer<'de>
{
    let utc_str = String::deserialize(deserializer)?;
    UtcDateTime::parse(&utc_str, &Rfc3339).map_err(serde::de::Error::custom)
}

pub fn serialize_utc_datetime<S>(time: &UtcDateTime, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer
{
    let timestamp = timestamp_to_string(&time).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&timestamp)
}

pub fn timestamp_to_string(timestamp: &UtcDateTime) -> Result<String, errors::Error>
{
    timestamp.format(&Rfc3339).map_err(|e| errors::internal(e.to_string()))
}
