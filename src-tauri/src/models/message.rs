//! Message model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Role of a message sender
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

impl std::fmt::Display for MessageRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageRole::User => write!(f, "user"),
            MessageRole::Assistant => write!(f, "assistant"),
            MessageRole::System => write!(f, "system"),
        }
    }
}

impl std::str::FromStr for MessageRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user" => Ok(MessageRole::User),
            "assistant" => Ok(MessageRole::Assistant),
            "system" => Ok(MessageRole::System),
            _ => Err(format!("Invalid message role: {}", s)),
        }
    }
}

/// A single message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// Unique identifier
    pub id: String,

    /// ID of the conversation this message belongs to
    pub conversation_id: String,

    /// Role of the message sender
    pub role: MessageRole,

    /// Message content
    pub content: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Token count (if available)
    pub token_count: Option<i32>,

    /// Model used to generate this message (for assistant messages)
    pub model: Option<String>,

    /// Stop reason (for assistant messages)
    pub stop_reason: Option<String>,

    /// Whether this message is still being streamed
    #[serde(default)]
    pub is_streaming: bool,

    /// Error message if generation failed
    pub error: Option<String>,
}

/// Request to create a new message
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageRequest {
    pub conversation_id: String,
    pub role: MessageRole,
    pub content: String,
    pub model: Option<String>,
}

/// Response from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub id: String,
    pub content: String,
    pub model: String,
    pub stop_reason: Option<String>,
    pub usage: Option<ApiUsage>,
}

/// Token usage from API response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiUsage {
    pub input_tokens: i32,
    pub output_tokens: i32,
}

/// Streaming chunk from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamChunk {
    pub chunk_type: String,
    pub delta: Option<String>,
    pub stop_reason: Option<String>,
    pub error: Option<String>,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            conversation_id: String::new(),
            role: MessageRole::User,
            content: String::new(),
            created_at: Utc::now(),
            token_count: None,
            model: None,
            stop_reason: None,
            is_streaming: false,
            error: None,
        }
    }
}
