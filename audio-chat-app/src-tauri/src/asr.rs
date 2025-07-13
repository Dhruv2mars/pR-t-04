use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};
use tauri::Manager;

pub async fn transcribe_audio(audio_path: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    // Try multiple possible model locations
    let possible_paths = vec![
        // Development path (src-tauri/models)
        std::env::current_dir()
            .unwrap_or_default()
            .join("src-tauri")
            .join("models")
            .join("ggml-base.en.bin"),
        // Alternative development path (models in current dir)
        std::env::current_dir()
            .unwrap_or_default()
            .join("models")
            .join("ggml-base.en.bin"),
        // Production path (app data dir)
        app_handle.path().app_data_dir()
            .unwrap_or_default()
            .join("models")
            .join("ggml-base.en.bin"),
        // Resource path for bundled app
        app_handle.path().resource_dir()
            .unwrap_or_default()
            .join("models")
            .join("ggml-base.en.bin"),
    ];
    
    let model_path = possible_paths
        .iter()
        .find(|path| path.exists())
        .ok_or_else(|| {
            let paths_str = possible_paths
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join("\n  - ");
            format!(
                "Whisper model not found. Searched in:\n  - {}\n\nPlease ensure ggml-base.en.bin is in one of these locations.",
                paths_str
            )
        })?;
    
    println!("Using Whisper model at: {}", model_path.display());
    
    // Load audio file and convert to required format
    let audio_data = load_audio_file(&audio_path)?;
    
    // Initialize Whisper context
    let ctx = WhisperContext::new_with_params(
        model_path.to_str().unwrap(),
        WhisperContextParameters::default()
    ).map_err(|e| format!("Failed to load Whisper model from {}: {}", model_path.display(), e))?;
    
    // Set up parameters for transcription
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some("en"));
    params.set_translate(false);
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    
    // Run inference
    let mut state = ctx.create_state().map_err(|e| format!("Failed to create Whisper state: {}", e))?;
    state.full(params, &audio_data)
        .map_err(|e| format!("Failed to run Whisper inference: {}", e))?;
    
    // Extract transcription
    let num_segments = state.full_n_segments()
        .map_err(|e| format!("Failed to get segment count: {}", e))?;
    
    println!("Whisper found {} segments", num_segments);
    
    let mut transcription = String::new();
    for i in 0..num_segments {
        let segment = state.full_get_segment_text(i)
            .map_err(|e| format!("Failed to get segment text: {}", e))?;
        println!("Segment {}: '{}'", i, segment);
        transcription.push_str(&segment);
    }
    
    let final_transcription = transcription.trim().to_string();
    println!("Final transcription: '{}'", final_transcription);
    
    if final_transcription.is_empty() {
        return Err("Transcription is empty - audio may be too short or silent".to_string());
    }
    
    Ok(final_transcription)
}

fn load_audio_file(path: &str) -> Result<Vec<f32>, String> {
    let mut reader = hound::WavReader::open(path)
        .map_err(|e| format!("Failed to open audio file: {}", e))?;
    
    let spec = reader.spec();
    
    // Whisper expects 16kHz mono audio
    if spec.sample_rate != 16000 {
        return Err("Audio must be 16kHz sample rate".to_string());
    }
    
    if spec.channels != 1 {
        return Err("Audio must be mono (1 channel)".to_string());
    }
    
    let samples: Result<Vec<f32>, _> = match spec.sample_format {
        hound::SampleFormat::Float => {
            reader.samples::<f32>().collect()
        },
        hound::SampleFormat::Int => {
            reader.samples::<i16>()
                .map(|s| s.map(|sample| sample as f32 / 32768.0))
                .collect()
        }
    };
    
    samples.map_err(|e| format!("Failed to read audio samples: {}", e))
}