//! Database service for conversation and message storage
//!
//! Uses SQLite for local storage with support for future SQLCipher encryption.

use std::path::Path;

use chrono::{DateTime, TimeZone, Utc};
use rusqlite::{params, Connection, OptionalExtension};

use crate::error::{AppError, Result};
use crate::models::{
    AppSettings, Conversation, CreateConversationRequest, CreateMessageRequest, Message,
    MessageRole, UpdateConversationRequest,
};

/// Database service for local storage
pub struct DatabaseService {
    conn: Connection,
}

impl DatabaseService {
    /// Create a new database service
    pub fn new(db_path: &Path) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(db_path)?;

        // Enable foreign keys
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        // Set WAL mode for better concurrency
        conn.execute_batch("PRAGMA journal_mode = WAL;")?;

        Ok(Self { conn })
    }

    /// Run database migrations
    pub fn run_migrations(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            -- Conversations table
            CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY NOT NULL,
                title TEXT,
                model TEXT NOT NULL DEFAULT 'claude-sonnet-4-20250514',
                system_prompt TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                pinned INTEGER NOT NULL DEFAULT 0,
                tags TEXT DEFAULT '[]'
            );

            -- Messages table
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY NOT NULL,
                conversation_id TEXT NOT NULL,
                role TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
                content TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                token_count INTEGER,
                model TEXT,
                stop_reason TEXT,
                error TEXT,
                FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
            );

            -- Settings table
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY NOT NULL,
                value TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            );

            -- Indexes
            CREATE INDEX IF NOT EXISTS idx_messages_conversation_id ON messages(conversation_id);
            CREATE INDEX IF NOT EXISTS idx_messages_created_at ON messages(created_at);
            CREATE INDEX IF NOT EXISTS idx_conversations_updated_at ON conversations(updated_at);
            CREATE INDEX IF NOT EXISTS idx_conversations_pinned ON conversations(pinned);
            "#,
        )?;

        tracing::info!("Database migrations completed");
        Ok(())
    }

    // ==================== Conversation Methods ====================

    /// Get all conversations ordered by updated_at descending
    pub fn get_conversations(&self) -> Result<Vec<Conversation>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT c.id, c.title, c.model, c.system_prompt, c.created_at, c.updated_at,
                   c.pinned, c.tags,
                   (SELECT COUNT(*) FROM messages WHERE conversation_id = c.id) as message_count
            FROM conversations c
            ORDER BY c.pinned DESC, c.updated_at DESC
            "#,
        )?;

        let conversations = stmt
            .query_map([], |row| {
                Ok(Conversation {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    model: row.get(2)?,
                    system_prompt: row.get(3)?,
                    created_at: timestamp_to_datetime(row.get(4)?),
                    updated_at: timestamp_to_datetime(row.get(5)?),
                    pinned: row.get::<_, i32>(6)? != 0,
                    tags: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
                    message_count: row.get(8)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(conversations)
    }

    /// Get a single conversation by ID
    pub fn get_conversation(&self, id: &str) -> Result<Option<Conversation>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT c.id, c.title, c.model, c.system_prompt, c.created_at, c.updated_at,
                   c.pinned, c.tags,
                   (SELECT COUNT(*) FROM messages WHERE conversation_id = c.id) as message_count
            FROM conversations c
            WHERE c.id = ?1
            "#,
        )?;

        let conversation = stmt
            .query_row([id], |row| {
                Ok(Conversation {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    model: row.get(2)?,
                    system_prompt: row.get(3)?,
                    created_at: timestamp_to_datetime(row.get(4)?),
                    updated_at: timestamp_to_datetime(row.get(5)?),
                    pinned: row.get::<_, i32>(6)? != 0,
                    tags: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
                    message_count: row.get(8)?,
                })
            })
            .optional()?;

        Ok(conversation)
    }

    /// Create a new conversation
    pub fn create_conversation(&self, request: CreateConversationRequest) -> Result<Conversation> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let timestamp = now.timestamp();
        let model = request
            .model
            .unwrap_or_else(|| "claude-sonnet-4-20250514".to_string());

        self.conn.execute(
            r#"
            INSERT INTO conversations (id, title, model, system_prompt, created_at, updated_at, pinned, tags)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0, '[]')
            "#,
            params![id, request.title, model, request.system_prompt, timestamp, timestamp],
        )?;

        Ok(Conversation {
            id,
            title: request.title,
            model,
            system_prompt: request.system_prompt,
            created_at: now,
            updated_at: now,
            pinned: false,
            tags: Vec::new(),
            message_count: 0,
        })
    }

    /// Update a conversation
    pub fn update_conversation(
        &self,
        id: &str,
        request: UpdateConversationRequest,
    ) -> Result<Conversation> {
        let existing = self
            .get_conversation(id)?
            .ok_or_else(|| AppError::NotFound(format!("Conversation {} not found", id)))?;

        let now = Utc::now();
        let timestamp = now.timestamp();

        let title = request.title.or(existing.title);
        let model = request.model.unwrap_or(existing.model);
        let system_prompt = request.system_prompt.or(existing.system_prompt);
        let pinned = request.pinned.unwrap_or(existing.pinned);
        let tags = request.tags.unwrap_or(existing.tags);
        let tags_json = serde_json::to_string(&tags)?;

        self.conn.execute(
            r#"
            UPDATE conversations
            SET title = ?1, model = ?2, system_prompt = ?3, pinned = ?4, tags = ?5, updated_at = ?6
            WHERE id = ?7
            "#,
            params![
                title,
                model,
                system_prompt,
                pinned as i32,
                tags_json,
                timestamp,
                id
            ],
        )?;

        Ok(Conversation {
            id: id.to_string(),
            title,
            model,
            system_prompt,
            created_at: existing.created_at,
            updated_at: now,
            pinned,
            tags,
            message_count: existing.message_count,
        })
    }

    /// Delete a conversation and all its messages
    pub fn delete_conversation(&self, id: &str) -> Result<()> {
        let affected = self
            .conn
            .execute("DELETE FROM conversations WHERE id = ?1", [id])?;

        if affected == 0 {
            return Err(AppError::NotFound(format!(
                "Conversation {} not found",
                id
            )));
        }

        tracing::info!("Deleted conversation {}", id);
        Ok(())
    }

    // ==================== Message Methods ====================

    /// Get all messages for a conversation
    pub fn get_messages(&self, conversation_id: &str) -> Result<Vec<Message>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT id, conversation_id, role, content, created_at, token_count, model, stop_reason, error
            FROM messages
            WHERE conversation_id = ?1
            ORDER BY created_at ASC
            "#,
        )?;

        let messages = stmt
            .query_map([conversation_id], |row| {
                let role_str: String = row.get(2)?;
                Ok(Message {
                    id: row.get(0)?,
                    conversation_id: row.get(1)?,
                    role: role_str.parse().unwrap_or(MessageRole::User),
                    content: row.get(3)?,
                    created_at: timestamp_to_datetime(row.get(4)?),
                    token_count: row.get(5)?,
                    model: row.get(6)?,
                    stop_reason: row.get(7)?,
                    is_streaming: false,
                    error: row.get(8)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(messages)
    }

    /// Create a new message
    pub fn create_message(&self, request: CreateMessageRequest) -> Result<Message> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let timestamp = now.timestamp();

        self.conn.execute(
            r#"
            INSERT INTO messages (id, conversation_id, role, content, created_at, model)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                id,
                request.conversation_id,
                request.role.to_string(),
                request.content,
                timestamp,
                request.model
            ],
        )?;

        // Update conversation's updated_at
        self.conn.execute(
            "UPDATE conversations SET updated_at = ?1 WHERE id = ?2",
            params![timestamp, request.conversation_id],
        )?;

        Ok(Message {
            id,
            conversation_id: request.conversation_id,
            role: request.role,
            content: request.content,
            created_at: now,
            token_count: None,
            model: request.model,
            stop_reason: None,
            is_streaming: false,
            error: None,
        })
    }

    /// Update a message (typically to add token count or stop reason)
    pub fn update_message(
        &self,
        id: &str,
        content: Option<&str>,
        token_count: Option<i32>,
        stop_reason: Option<&str>,
        error: Option<&str>,
    ) -> Result<()> {
        self.conn.execute(
            r#"
            UPDATE messages
            SET content = COALESCE(?1, content),
                token_count = COALESCE(?2, token_count),
                stop_reason = COALESCE(?3, stop_reason),
                error = COALESCE(?4, error)
            WHERE id = ?5
            "#,
            params![content, token_count, stop_reason, error, id],
        )?;

        Ok(())
    }

    /// Delete a message
    pub fn delete_message(&self, id: &str) -> Result<()> {
        let affected = self
            .conn
            .execute("DELETE FROM messages WHERE id = ?1", [id])?;

        if affected == 0 {
            return Err(AppError::NotFound(format!("Message {} not found", id)));
        }

        Ok(())
    }

    // ==================== Settings Methods ====================

    /// Get all settings
    pub fn get_settings(&self) -> Result<AppSettings> {
        let mut stmt = self
            .conn
            .prepare("SELECT key, value FROM settings")?;

        let mut settings = AppSettings::default();

        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        for row in rows {
            let (key, value) = row?;
            match key.as_str() {
                "model" => settings.model = value,
                "theme" => settings.theme = value,
                "language" => settings.language = value,
                "streaming_enabled" => settings.streaming_enabled = value == "true",
                "proxy_url" => settings.proxy_url = if value.is_empty() { None } else { Some(value) },
                "api_timeout" => settings.api_timeout = value.parse().unwrap_or(120),
                "max_tokens" => settings.max_tokens = value.parse().unwrap_or(4096),
                "temperature" => settings.temperature = value.parse().unwrap_or(1.0),
                "first_run_completed" => settings.first_run_completed = value == "true",
                "privacy_mode" => settings.privacy_mode = value == "true",
                "analytics_enabled" => settings.analytics_enabled = value == "true",
                _ => {}
            }
        }

        Ok(settings)
    }

    /// Update all settings
    pub fn update_settings(&self, settings: &AppSettings) -> Result<()> {
        let timestamp = Utc::now().timestamp();

        let settings_map = vec![
            ("model", settings.model.clone()),
            ("theme", settings.theme.clone()),
            ("language", settings.language.clone()),
            ("streaming_enabled", settings.streaming_enabled.to_string()),
            ("proxy_url", settings.proxy_url.clone().unwrap_or_default()),
            ("api_timeout", settings.api_timeout.to_string()),
            ("max_tokens", settings.max_tokens.to_string()),
            ("temperature", settings.temperature.to_string()),
            ("first_run_completed", settings.first_run_completed.to_string()),
            ("privacy_mode", settings.privacy_mode.to_string()),
            ("analytics_enabled", settings.analytics_enabled.to_string()),
        ];

        for (key, value) in settings_map {
            self.conn.execute(
                r#"
                INSERT INTO settings (key, value, updated_at)
                VALUES (?1, ?2, ?3)
                ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = ?3
                "#,
                params![key, value, timestamp],
            )?;
        }

        Ok(())
    }

    /// Get a single setting
    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let value = self
            .conn
            .query_row(
                "SELECT value FROM settings WHERE key = ?1",
                [key],
                |row| row.get(0),
            )
            .optional()?;

        Ok(value)
    }

    /// Set a single setting
    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        let timestamp = Utc::now().timestamp();

        self.conn.execute(
            r#"
            INSERT INTO settings (key, value, updated_at)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = ?3
            "#,
            params![key, value, timestamp],
        )?;

        Ok(())
    }

    /// Clear all data (for privacy mode or reset)
    pub fn clear_all_data(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            DELETE FROM messages;
            DELETE FROM conversations;
            DELETE FROM settings;
            VACUUM;
            "#,
        )?;

        tracing::info!("All database data cleared");
        Ok(())
    }
}

/// Convert Unix timestamp to DateTime<Utc>
fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    Utc.timestamp_opt(timestamp, 0)
        .single()
        .unwrap_or_else(Utc::now)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_db() -> (DatabaseService, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = DatabaseService::new(&db_path).unwrap();
        db.run_migrations().unwrap();
        (db, temp_dir)
    }

    #[test]
    fn test_create_conversation() {
        let (db, _temp_dir) = create_test_db();

        let request = CreateConversationRequest {
            title: Some("Test Conversation".to_string()),
            model: None,
            system_prompt: None,
        };

        let conv = db.create_conversation(request).unwrap();
        assert_eq!(conv.title, Some("Test Conversation".to_string()));
        assert!(!conv.id.is_empty());
    }

    #[test]
    fn test_create_and_get_messages() {
        let (db, _temp_dir) = create_test_db();

        let conv = db
            .create_conversation(CreateConversationRequest {
                title: Some("Test".to_string()),
                model: None,
                system_prompt: None,
            })
            .unwrap();

        let request = CreateMessageRequest {
            conversation_id: conv.id.clone(),
            role: MessageRole::User,
            content: "Hello!".to_string(),
            model: None,
        };

        let msg = db.create_message(request).unwrap();
        assert_eq!(msg.content, "Hello!");
        assert_eq!(msg.role, MessageRole::User);

        let messages = db.get_messages(&conv.id).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "Hello!");
    }
}
