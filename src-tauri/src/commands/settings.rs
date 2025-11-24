//! Settings management commands

use tauri::State;

use crate::error::AppError;
use crate::models::AppSettings;
use crate::state::AppState;

/// Get all settings
#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, AppError> {
    let db = state.database.lock().await;
    db.get_settings()
}

/// Update all settings
#[tauri::command]
pub async fn update_settings(
    settings: AppSettings,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let db = state.database.lock().await;
    db.update_settings(&settings)?;
    tracing::info!("Settings updated");
    Ok(())
}

/// Get a single setting
#[tauri::command]
pub async fn get_setting(
    key: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, AppError> {
    let db = state.database.lock().await;
    db.get_setting(&key)
}

/// Set a single setting
#[tauri::command]
pub async fn set_setting(
    key: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let db = state.database.lock().await;
    db.set_setting(&key, &value)?;
    tracing::info!("Setting {} updated", key);
    Ok(())
}
