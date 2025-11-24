//! Application state management

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::services::{config::ConfigService, database::DatabaseService, keyring::KeyringService};

/// Global application state shared across Tauri commands
pub struct AppState {
    /// Configuration service
    pub config: Arc<ConfigService>,

    /// Keyring service for secure credential storage
    pub keyring: Arc<Mutex<KeyringService>>,

    /// Database service for conversation and message storage
    pub database: Arc<Mutex<DatabaseService>>,
}
