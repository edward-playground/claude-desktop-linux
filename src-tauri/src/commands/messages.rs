//! Message management commands

use tauri::State;

use crate::error::AppError;
use crate::models::{CreateMessageRequest, Message};
use crate::state::AppState;

/// Get all messages for a conversation
#[tauri::command]
pub async fn get_messages(
    conversation_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Message>, AppError> {
    let db = state.database.lock().await;
    db.get_messages(&conversation_id)
}

/// Create a new message
#[tauri::command]
pub async fn create_message(
    request: CreateMessageRequest,
    state: State<'_, AppState>,
) -> Result<Message, AppError> {
    // Check if privacy mode is enabled
    let settings = {
        let db = state.database.lock().await;
        db.get_settings()?
    };

    if settings.privacy_mode {
        // In privacy mode, return a message without persisting it
        return Ok(Message {
            id: uuid::Uuid::new_v4().to_string(),
            conversation_id: request.conversation_id,
            role: request.role,
            content: request.content,
            created_at: chrono::Utc::now(),
            token_count: None,
            model: request.model,
            stop_reason: None,
            is_streaming: false,
            error: None,
        });
    }

    let db = state.database.lock().await;
    let message = db.create_message(request)?;
    Ok(message)
}

/// Delete a message
#[tauri::command]
pub async fn delete_message(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    let db = state.database.lock().await;
    db.delete_message(&id)?;
    tracing::info!("Deleted message: {}", id);
    Ok(())
}
