use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::Manager;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub role: String, // "user" or "assistant"
    pub content: String,
    pub timestamp: String,
}

fn get_db_path(app_handle: tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    
    Ok(app_data_dir.join("conversations.db"))
}

pub async fn init_database(app_handle: tauri::AppHandle) -> Result<(), String> {
    let db_path = get_db_path(app_handle)?;
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS conversations (
            id TEXT PRIMARY KEY,
            created_at TEXT NOT NULL
        )",
        [],
    ).map_err(|e| format!("Failed to create conversations table: {}", e))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            conversation_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            FOREIGN KEY (conversation_id) REFERENCES conversations(id)
        )",
        [],
    ).map_err(|e| format!("Failed to create messages table: {}", e))?;

    Ok(())
}

pub async fn create_conversation(app_handle: tauri::AppHandle) -> Result<String, String> {
    let db_path = get_db_path(app_handle)?;
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    let conversation_id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO conversations (id, created_at) VALUES (?1, ?2)",
        [&conversation_id, &created_at],
    ).map_err(|e| format!("Failed to create conversation: {}", e))?;

    Ok(conversation_id)
}

pub async fn save_message(
    conversation_id: String,
    role: String,
    content: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let db_path = get_db_path(app_handle)?;
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    let message_id = Uuid::new_v4().to_string();
    let timestamp = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO messages (id, conversation_id, role, content, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
        [&message_id, &conversation_id, &role, &content, &timestamp],
    ).map_err(|e| format!("Failed to save message: {}", e))?;

    Ok(())
}

pub async fn get_conversations(app_handle: tauri::AppHandle) -> Result<Vec<Conversation>, String> {
    let db_path = get_db_path(app_handle)?;
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    let mut stmt = conn
        .prepare("SELECT id, created_at FROM conversations ORDER BY created_at DESC")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let conversation_iter = stmt
        .query_map([], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                created_at: row.get(1)?,
            })
        })
        .map_err(|e| format!("Failed to query conversations: {}", e))?;

    let mut conversations = Vec::new();
    for conversation in conversation_iter {
        conversations.push(conversation.map_err(|e| format!("Failed to parse conversation: {}", e))?);
    }

    Ok(conversations)
}

pub async fn get_messages(conversation_id: String, app_handle: tauri::AppHandle) -> Result<Vec<Message>, String> {
    let db_path = get_db_path(app_handle)?;
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    let mut stmt = conn
        .prepare("SELECT id, conversation_id, role, content, timestamp FROM messages WHERE conversation_id = ?1 ORDER BY timestamp ASC")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let message_iter = stmt
        .query_map([&conversation_id], |row| {
            Ok(Message {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                timestamp: row.get(4)?,
            })
        })
        .map_err(|e| format!("Failed to query messages: {}", e))?;

    let mut messages = Vec::new();
    for message in message_iter {
        messages.push(message.map_err(|e| format!("Failed to parse message: {}", e))?);
    }

    Ok(messages)
}