use tauri::Manager;

pub async fn synthesize_speech(text: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    // For now, use macOS built-in TTS as a fallback while MeloTTS is being set up
    // This ensures the pipeline works end-to-end
    
    // Use app data directory instead of temp directory for better access control
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    // Ensure the directory exists
    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    
    let output_path = app_data_dir.join(format!("tts_output_{}.wav", uuid::Uuid::new_v4()));
    
    // Use macOS `say` command to generate speech
    let output = std::process::Command::new("say")
        .arg("-o")
        .arg(&output_path)
        .arg("--file-format=WAVE")
        .arg("--data-format=LEI16")
        .arg(&text)
        .output()
        .map_err(|e| format!("Failed to execute say command: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("TTS failed: {}", stderr));
    }
    
    // Check if output file was created
    if !output_path.exists() {
        return Err("TTS did not generate output file".to_string());
    }
    
    println!("TTS file generated successfully at: {}", output_path.display());
    println!("File size: {} bytes", std::fs::metadata(&output_path).map(|m| m.len()).unwrap_or(0));
    
    Ok(output_path.to_string_lossy().to_string())
}