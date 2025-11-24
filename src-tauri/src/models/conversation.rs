//! Conversation model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A conversation (thread) containing multiple messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    /// Unique identifier
    pub id: String,

    /// Optional title for the conversation
    pub title: Option<String>,

    /// Model used for this conversation
    pub model: String,

    /// Optional system prompt
    pub system_prompt: Option<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,

    /// Whether this conversation is pinned
    pub pinned: bool,

    /// Optional tags for organization
    pub tags: Vec<String>,

    /// Message count (for display, not stored)
    #[serde(default)]
    pub message_count: i32,
}

/// Request to create a new conversation
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversationRequest {
    pub title: Option<String>,
    pub model: Option<String>,
    pub system_prompt: Option<String>,
}

/// Request to update a conversation
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConversationRequest {
    pub title: Option<String>,
    pub model: Option<String>,
    pub system_prompt: Option<String>,
    pub pinned: Option<bool>,
    pub tags: Option<Vec<String>>,
}

impl Default for Conversation {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: None,
            model: "claude-sonnet-4-20250514".to_string(),
            system_prompt: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            pinned: false,
            tags: Vec::new(),
            message_count: 0,
        }
    }
}
