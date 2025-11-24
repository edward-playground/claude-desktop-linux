//! Settings model

use serde::{Deserialize, Serialize};

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    /// Selected model
    pub model: String,

    /// Theme (light, dark, system)
    pub theme: String,

    /// Language code (en, zh-TW)
    pub language: String,

    /// Whether to stream responses
    pub streaming_enabled: bool,

    /// Proxy URL (optional)
    pub proxy_url: Option<String>,

    /// API timeout in seconds
    pub api_timeout: u32,

    /// Maximum tokens for responses
    pub max_tokens: u32,

    /// Temperature for generation
    pub temperature: f32,

    /// Whether first run wizard has been completed
    pub first_run_completed: bool,

    /// Privacy mode - don't persist conversations
    pub privacy_mode: bool,

    /// Send anonymous usage analytics (opt-in)
    pub analytics_enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            model: "claude-sonnet-4-20250514".to_string(),
            theme: "system".to_string(),
            language: "en".to_string(),
            streaming_enabled: true,
            proxy_url: None,
            api_timeout: 120,
            max_tokens: 4096,
            temperature: 1.0,
            first_run_completed: false,
            privacy_mode: false,
            analytics_enabled: false,
        }
    }
}

/// Available AI models
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub context_window: u32,
    pub max_output: u32,
}

impl AvailableModel {
    pub fn get_available_models() -> Vec<Self> {
        vec![
            AvailableModel {
                id: "claude-sonnet-4-20250514".to_string(),
                name: "Claude Sonnet 4".to_string(),
                description: "Best balance of speed and intelligence".to_string(),
                context_window: 200000,
                max_output: 64000,
            },
            AvailableModel {
                id: "claude-opus-4-5-20251101".to_string(),
                name: "Claude Opus 4.5".to_string(),
                description: "Most intelligent model for complex tasks".to_string(),
                context_window: 200000,
                max_output: 64000,
            },
            AvailableModel {
                id: "claude-haiku-4-5-20251001".to_string(),
                name: "Claude Haiku 4.5".to_string(),
                description: "Fastest model for simple tasks".to_string(),
                context_window: 200000,
                max_output: 64000,
            },
            AvailableModel {
                id: "claude-3-5-sonnet-20241022".to_string(),
                name: "Claude 3.5 Sonnet".to_string(),
                description: "Previous generation balanced model".to_string(),
                context_window: 200000,
                max_output: 8192,
            },
        ]
    }
}
