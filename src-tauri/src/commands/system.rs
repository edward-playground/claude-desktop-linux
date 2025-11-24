//! System-related commands

use serde::Serialize;
use tauri::State;

use crate::error::AppError;
use crate::state::AppState;

/// Application information
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub repository: String,
    pub license: String,
    pub is_unofficial: bool,
}

/// Data paths information
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPaths {
    pub config_dir: String,
    pub data_dir: String,
    pub cache_dir: String,
    pub database_path: String,
    pub log_path: String,
}

/// Get application information
#[tauri::command]
pub async fn get_app_info() -> Result<AppInfo, AppError> {
    Ok(AppInfo {
        name: "Claude for Linux".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: "Unofficial Community Desktop Client for Claude on Linux".to_string(),
        repository: "https://github.com/edward-playground/claude-desktop-linux".to_string(),
        license: "MIT".to_string(),
        is_unofficial: true,
    })
}

/// Get data paths
#[tauri::command]
pub async fn get_data_paths(state: State<'_, AppState>) -> Result<DataPaths, AppError> {
    let paths = state.config.paths();

    Ok(DataPaths {
        config_dir: paths.config_dir.display().to_string(),
        data_dir: paths.data_dir.display().to_string(),
        cache_dir: paths.cache_dir.display().to_string(),
        database_path: paths.database_path().display().to_string(),
        log_path: paths.log_path().display().to_string(),
    })
}

/// Clear all application data
#[tauri::command]
pub async fn clear_all_data(state: State<'_, AppState>) -> Result<(), AppError> {
    // Clear database
    {
        let db = state.database.lock().await;
        db.clear_all_data()?;
    }

    // Clear keyring
    {
        let keyring = state.keyring.lock().await;
        keyring.clear_all()?;
    }

    tracing::info!("All application data cleared");
    Ok(())
}
