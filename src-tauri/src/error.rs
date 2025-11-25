//! Error types for the application

use serde::Serialize;
use thiserror::Error;

/// Main error type for the application
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Keyring error: {0}")]
    Keyring(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<keyring::Error> for AppError {
    fn from(err: keyring::Error) -> Self {
        AppError::Keyring(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::Network("Request timed out".to_string())
        } else if err.is_connect() {
            AppError::Network("Failed to connect to server".to_string())
        } else {
            AppError::Network(err.to_string())
        }
    }
}

/// Serializable error for frontend
#[derive(Debug, Serialize)]
pub struct SerializableError {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

impl From<AppError> for SerializableError {
    fn from(err: AppError) -> Self {
        let (code, message, details) = match &err {
            AppError::Keyring(msg) => ("KEYRING_ERROR", "Keyring error", Some(msg.clone())),
            AppError::Database(msg) => ("DATABASE_ERROR", "Database error", Some(msg.clone())),
            AppError::Api(msg) => ("API_ERROR", "API error", Some(msg.clone())),
            AppError::Config(msg) => ("CONFIG_ERROR", "Configuration error", Some(msg.clone())),
            AppError::Io(e) => ("IO_ERROR", "I/O error", Some(e.to_string())),
            AppError::Serialization(e) => (
                "SERIALIZATION_ERROR",
                "Serialization error",
                Some(e.to_string()),
            ),
            AppError::NotFound(msg) => ("NOT_FOUND", "Not found", Some(msg.clone())),
            AppError::InvalidInput(msg) => ("INVALID_INPUT", "Invalid input", Some(msg.clone())),
            AppError::Network(msg) => ("NETWORK_ERROR", "Network error", Some(msg.clone())),
            AppError::Authentication(msg) => {
                ("AUTH_ERROR", "Authentication error", Some(msg.clone()))
            }
            AppError::RateLimited(msg) => ("RATE_LIMITED", "Rate limited", Some(msg.clone())),
        };

        SerializableError {
            code: code.to_string(),
            message: message.to_string(),
            details,
        }
    }
}

// Implement Serialize for AppError to work with Tauri commands
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serializable: SerializableError = self.clone().into();
        serializable.serialize(serializer)
    }
}

impl Clone for AppError {
    fn clone(&self) -> Self {
        match self {
            AppError::Keyring(s) => AppError::Keyring(s.clone()),
            AppError::Database(s) => AppError::Database(s.clone()),
            AppError::Api(s) => AppError::Api(s.clone()),
            AppError::Config(s) => AppError::Config(s.clone()),
            AppError::Io(e) => AppError::Io(std::io::Error::new(e.kind(), e.to_string())),
            AppError::Serialization(_) => {
                AppError::Serialization(serde_json::from_str::<()>("invalid").unwrap_err())
            }
            AppError::NotFound(s) => AppError::NotFound(s.clone()),
            AppError::InvalidInput(s) => AppError::InvalidInput(s.clone()),
            AppError::Network(s) => AppError::Network(s.clone()),
            AppError::Authentication(s) => AppError::Authentication(s.clone()),
            AppError::RateLimited(s) => AppError::RateLimited(s.clone()),
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
