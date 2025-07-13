use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;
use std::time::Duration;

#[tauri::command]
pub async fn request_microphone_permission() -> Result<bool, String> {
    // This function will attempt to access the microphone, which will trigger
    // the macOS permission prompt and make the app appear in Privacy Settings
    
    tokio::task::spawn_blocking(|| {
        let host = cpal::default_host();
        
        // Try to get the default input device
        match host.default_input_device() {
            Some(device) => {
                println!("Found microphone device: {}", device.name().unwrap_or("Unknown".to_string()));
                
                // Try to get the default input config
                match device.default_input_config() {
                    Ok(config) => {
                        println!("Microphone config: {:?}", config);
                        
                        // Try to build a stream (this will trigger the permission request)
                        let (tx, _rx) = mpsc::channel();
                        
                        let stream_result = device.build_input_stream(
                            &config.into(),
                            move |_data: &[f32], _: &cpal::InputCallbackInfo| {
                                // We don't need to do anything with the data
                                // Just having this stream triggers the permission request
                                let _ = tx.send(());
                            },
                            move |err| {
                                eprintln!("An error occurred on the input audio stream: {}", err);
                            },
                            None,
                        );
                        
                        match stream_result {
                            Ok(stream) => {
                                // Start the stream briefly to trigger permission
                                if let Err(e) = stream.play() {
                                    eprintln!("Failed to start stream: {}", e);
                                    return Ok(false);
                                }
                                
                                // Wait a moment for the permission prompt
                                std::thread::sleep(Duration::from_millis(100));
                                
                                // Stop the stream
                                drop(stream);
                                
                                println!("Microphone access requested successfully");
                                Ok(true)
                            }
                            Err(e) => {
                                eprintln!("Failed to build input stream: {}", e);
                                Err(format!("Failed to access microphone: {}", e))
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get default input config: {}", e);
                        Err(format!("Failed to get microphone config: {}", e))
                    }
                }
            }
            None => {
                eprintln!("No input device found");
                Err("No microphone device found".to_string())
            }
        }
    }).await
    .map_err(|e| format!("Task failed: {}", e))?
}


#[tauri::command]
pub async fn check_microphone_permission() -> Result<bool, String> {
    // Try to access microphone without triggering permission prompt
    tokio::task::spawn_blocking(|| {
        let host = cpal::default_host();
        
        match host.default_input_device() {
            Some(device) => {
                match device.default_input_config() {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
            None => Ok(false),
        }
    }).await
    .map_err(|e| format!("Task failed: {}", e))?
}