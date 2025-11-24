//! Claude for Linux - Unofficial Community Desktop Client
//!
//! This library contains the core functionality for the Tauri backend.

mod commands;
mod error;
mod models;
mod services;
mod state;

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::services::{config::ConfigService, database::DatabaseService, keyring::KeyringService};
use crate::state::AppState;

/// Initialize logging with file output and console output
fn init_logging() {
    let config = ConfigService::new();
    let log_dir = config.paths().cache_dir.clone();

    // Create cache directory if it doesn't exist
    std::fs::create_dir_all(&log_dir).ok();

    let file_appender = tracing_appender::rolling::daily(&log_dir, "claude-for-linux.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,claude_for_linux_lib=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();
}

/// Run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logging();

    tracing::info!("Starting Claude for Linux v{}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tracing::info!("Initializing application state...");

            // Initialize services
            let config_service = ConfigService::new();

            // Ensure directories exist
            config_service.ensure_directories()?;

            // Initialize keyring service
            let keyring_service = KeyringService::new().map_err(|e| {
                tracing::warn!("Failed to initialize keyring: {}", e);
                e
            })?;

            // Initialize database service
            let db_path = config_service.paths().database_path();
            let database_service = DatabaseService::new(&db_path).map_err(|e| {
                tracing::error!("Failed to initialize database: {}", e);
                e
            })?;

            // Run migrations
            database_service.run_migrations()?;

            // Create app state
            let app_state = AppState {
                config: Arc::new(config_service),
                keyring: Arc::new(Mutex::new(keyring_service)),
                database: Arc::new(Mutex::new(database_service)),
            };

            // Store state in app
            app.manage(app_state);

            tracing::info!("Application state initialized successfully");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Keyring commands
            commands::keyring::get_api_key,
            commands::keyring::set_api_key,
            commands::keyring::delete_api_key,
            commands::keyring::has_api_key,
            // Conversation commands
            commands::conversations::get_conversations,
            commands::conversations::get_conversation,
            commands::conversations::create_conversation,
            commands::conversations::update_conversation,
            commands::conversations::delete_conversation,
            // Message commands
            commands::messages::get_messages,
            commands::messages::create_message,
            commands::messages::delete_message,
            // Settings commands
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::settings::get_setting,
            commands::settings::set_setting,
            // API commands
            commands::api::validate_api_key,
            commands::api::get_available_models,
            commands::api::send_message,
            // System commands
            commands::system::get_app_info,
            commands::system::get_data_paths,
            commands::system::clear_all_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
