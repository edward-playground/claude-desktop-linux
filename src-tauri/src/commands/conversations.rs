//! Conversation management commands

use tauri::State;

use crate::error::AppError;
use crate::models::{Conversation, CreateConversationRequest, UpdateConversationRequest};
use crate::state::AppState;

/// Get all conversations
#[tauri::command]
pub async fn get_conversations(state: State<'_, AppState>) -> Result<Vec<Conversation>, AppError> {
    let db = state.database.lock().await;
    db.get_conversations()
}

/// Get a single conversation by ID
#[tauri::command]
pub async fn get_conversation(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<Conversation>, AppError> {
    let db = state.database.lock().await;
    db.get_conversation(&id)
}

/// Create a new conversation
#[tauri::command]
pub async fn create_conversation(
    request: CreateConversationRequest,
    state: State<'_, AppState>,
) -> Result<Conversation, AppError> {
    let db = state.database.lock().await;
    let conversation = db.create_conversation(request)?;
    tracing::info!("Created conversation: {}", conversation.id);
    Ok(conversation)
}

/// Update a conversation
#[tauri::command]
pub async fn update_conversation(
    id: String,
    request: UpdateConversationRequest,
    state: State<'_, AppState>,
) -> Result<Conversation, AppError> {
    let db = state.database.lock().await;
    let conversation = db.update_conversation(&id, request)?;
    tracing::info!("Updated conversation: {}", id);
    Ok(conversation)
}

/// Delete a conversation
#[tauri::command]
pub async fn delete_conversation(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    let db = state.database.lock().await;
    db.delete_conversation(&id)?;
    tracing::info!("Deleted conversation: {}", id);
    Ok(())
}
