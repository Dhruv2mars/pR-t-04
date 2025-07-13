// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod asr;
mod tts;
mod ollama;
mod microphone;
mod audio_processing;

use std::sync::Mutex;
use std::collections::HashMap;

// State to manage conversations
#[derive(Default)]
struct AppState {
    _conversations: Mutex<HashMap<String, Vec<db::Message>>>,
}

#[tauri::command]
async fn transcribe_audio(audio_path: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    asr::transcribe_audio(audio_path, app_handle).await
}

#[tauri::command]
async fn synthesize_speech(text: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    tts::synthesize_speech(text, app_handle).await
}

#[tauri::command]
async fn send_prompt(prompt: String) -> Result<String, String> {
    ollama::send_prompt(prompt).await
}

#[tauri::command]
async fn check_ollama() -> bool {
    ollama::check_ollama().await
}

#[tauri::command]
async fn create_conversation(app_handle: tauri::AppHandle) -> Result<String, String> {
    db::create_conversation(app_handle).await
}

#[tauri::command]
async fn save_message(
    conversation_id: String,
    role: String,
    content: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    db::save_message(conversation_id, role, content, app_handle).await
}

#[tauri::command]
async fn get_conversations(app_handle: tauri::AppHandle) -> Result<Vec<db::Conversation>, String> {
    db::get_conversations(app_handle).await
}

#[tauri::command]
async fn get_messages(conversation_id: String, app_handle: tauri::AppHandle) -> Result<Vec<db::Message>, String> {
    db::get_messages(conversation_id, app_handle).await
}

#[tauri::command]
async fn process_audio_blob(audio_data: Vec<u8>, app_handle: tauri::AppHandle) -> Result<String, String> {
    // Process the audio blob and convert to proper format
    let temp_path = audio_processing::process_audio_blob(audio_data)?;
    
    // Transcribe the processed audio
    let transcription = asr::transcribe_audio(temp_path.to_string_lossy().to_string(), app_handle).await?;
    
    // Clean up the temporary file
    if let Err(e) = std::fs::remove_file(&temp_path) {
        eprintln!("Warning: Failed to clean up temp file {}: {}", temp_path.display(), e);
    }
    
    Ok(transcription)
}

#[tauri::command]
async fn read_audio_file(path: String) -> Result<Vec<u8>, String> {
    println!("Reading audio file: {}", path);
    let data = std::fs::read(&path)
        .map_err(|e| format!("Failed to read audio file {}: {}", path, e))?;
    println!("Read {} bytes from audio file", data.len());
    Ok(data)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .setup(|app| {
            // Initialize database
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = db::init_database(app_handle).await {
                    eprintln!("Failed to initialize database: {}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            transcribe_audio,
            synthesize_speech,
            send_prompt,
            check_ollama,
            create_conversation,
            save_message,
            get_conversations,
            get_messages,
            process_audio_blob,
            read_audio_file,
            microphone::request_microphone_permission,
            microphone::check_microphone_permission
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}