//! API-related commands for communicating with Claude

use tauri::{AppHandle, Emitter, State};

use crate::error::AppError;
use crate::models::{AvailableModel, Message};
use crate::services::api_client::ApiClient;
use crate::state::AppState;

/// Validate an API key
#[tauri::command]
pub async fn validate_api_key(api_key: String) -> Result<bool, AppError> {
    if api_key.is_empty() {
        return Err(AppError::InvalidInput("API key cannot be empty".to_string()));
    }

    ApiClient::validate_api_key(&api_key).await
}

/// Get available models
#[tauri::command]
pub async fn get_available_models() -> Result<Vec<AvailableModel>, AppError> {
    Ok(AvailableModel::get_available_models())
}

/// Send a message and get a response (with optional streaming)
#[tauri::command]
pub async fn send_message(
    app: AppHandle,
    conversation_id: String,
    messages: Vec<Message>,
    model: String,
    system_prompt: Option<String>,
    stream: bool,
    state: State<'_, AppState>,
) -> Result<Option<Message>, AppError> {
    // Get API key from keyring
    let api_key = {
        let keyring = state.keyring.lock().await;
        keyring.get_api_key()?
    };

    // Get settings for timeout and max tokens
    let settings = {
        let db = state.database.lock().await;
        db.get_settings()?
    };

    // Create API client
    let client = if let Some(proxy_url) = &settings.proxy_url {
        ApiClient::new_with_proxy(api_key, settings.api_timeout as u64, proxy_url)?
    } else {
        ApiClient::new(api_key, settings.api_timeout as u64)?
    };

    if stream && settings.streaming_enabled {
        // Streaming response
        let mut receiver = client
            .send_message_streaming(
                &model,
                &messages,
                system_prompt.as_deref(),
                Some(settings.max_tokens),
                Some(settings.temperature),
            )
            .await?;

        // Spawn a task to forward stream chunks to the frontend
        let event_name = format!("stream-{}", conversation_id);
        tokio::spawn(async move {
            while let Some(chunk) = receiver.recv().await {
                if let Err(e) = app.emit(&event_name, &chunk) {
                    tracing::error!("Failed to emit stream chunk: {}", e);
                    break;
                }

                // Stop if we received an error or stop message
                if chunk.chunk_type == "error" || chunk.chunk_type == "message_stop" {
                    break;
                }
            }
        });

        // Return None for streaming - the frontend will assemble the message from events
        Ok(None)
    } else {
        // Non-streaming response
        let response = client
            .send_message(
                &model,
                &messages,
                system_prompt.as_deref(),
                Some(settings.max_tokens),
                Some(settings.temperature),
            )
            .await?;

        // Create message from response
        let message = Message {
            id: response.id,
            conversation_id,
            role: crate::models::MessageRole::Assistant,
            content: response.content,
            created_at: chrono::Utc::now(),
            token_count: response.usage.map(|u| u.output_tokens),
            model: Some(response.model),
            stop_reason: response.stop_reason,
            is_streaming: false,
            error: None,
        };

        Ok(Some(message))
    }
}

/// Cancel an ongoing stream
#[tauri::command]
pub async fn cancel_stream(_conversation_id: String) -> Result<(), AppError> {
    // The frontend can stop listening to events
    // The tokio task will naturally terminate when the receiver is dropped
    Ok(())
}
