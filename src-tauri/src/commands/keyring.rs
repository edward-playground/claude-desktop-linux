//! Keyring commands for secure credential management

use tauri::State;

use crate::error::AppError;
use crate::state::AppState;

/// Get the stored API key
#[tauri::command]
pub async fn get_api_key(state: State<'_, AppState>) -> Result<String, AppError> {
    let keyring = state.keyring.lock().await;
    keyring.get_api_key()
}

/// Store an API key
#[tauri::command]
pub async fn set_api_key(api_key: String, state: State<'_, AppState>) -> Result<(), AppError> {
    if api_key.is_empty() {
        return Err(AppError::InvalidInput("API key cannot be empty".to_string()));
    }

    // Basic validation - Anthropic API keys typically start with "sk-ant-"
    if !api_key.starts_with("sk-ant-") && !api_key.starts_with("sk-") {
        tracing::warn!("API key doesn't match expected format, but storing anyway");
    }

    let keyring = state.keyring.lock().await;
    keyring.set_api_key(&api_key)
}

/// Delete the stored API key
#[tauri::command]
pub async fn delete_api_key(state: State<'_, AppState>) -> Result<(), AppError> {
    let keyring = state.keyring.lock().await;
    keyring.delete_api_key()
}

/// Check if an API key is stored
#[tauri::command]
pub async fn has_api_key(state: State<'_, AppState>) -> Result<bool, AppError> {
    let keyring = state.keyring.lock().await;
    keyring.has_api_key()
}
