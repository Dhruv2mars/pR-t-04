use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use std::io::Cursor;

use std::path::PathBuf;
use rubato::{Resampler, SincFixedIn, SincInterpolationType, SincInterpolationParameters, WindowFunction};
use std::fs;

pub fn process_audio_blob(audio_data: Vec<u8>) -> Result<PathBuf, String> {
    // Create a persistent temporary file for the processed audio
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join(format!("whisper_audio_{}.wav", uuid::Uuid::new_v4()));
    
    // First, save the raw audio data to a temporary file for debugging
    let raw_temp_path = temp_dir.join(format!("raw_audio_{}.bin", uuid::Uuid::new_v4()));
    
    println!("Processing audio blob: {} bytes", audio_data.len());
    
    if audio_data.is_empty() {
        return Err("Audio data is empty".to_string());
    }
    
    fs::write(&raw_temp_path, &audio_data)
        .map_err(|e| format!("Failed to write raw audio data: {}", e))?;
    
    println!("Raw audio data saved to: {}", raw_temp_path.display());
    
    // Try to read the audio data as WAV first
    let cursor = Cursor::new(&audio_data);
    let wav_result = WavReader::new(cursor);
    
    let (samples, sample_rate, channels) = match wav_result {
        Ok(mut reader) => {
            // It's a valid WAV file
            let spec = reader.spec();
            let sample_rate = spec.sample_rate;
            let channels = spec.channels;
            
            println!("Valid WAV file detected: {}Hz, {} channels, {:?} format", 
                     sample_rate, channels, spec.sample_format);
            
            let samples: Result<Vec<f32>, _> = match spec.sample_format {
                SampleFormat::Float => {
                    reader.samples::<f32>().collect()
                },
                SampleFormat::Int => {
                    reader.samples::<i16>()
                        .map(|s| s.map(|sample| sample as f32 / 32768.0))
                        .collect()
                }
            };
            
            let samples = samples.map_err(|e| format!("Failed to read samples: {}", e))?;
            println!("Read {} samples from WAV file", samples.len());
            (samples, sample_rate, channels)
        },
        Err(e) => {
            // Not a WAV file, try to decode using ffmpeg or assume it's WebM/OGG
            println!("Not a valid WAV file ({}), trying ffmpeg decode", e);
            decode_with_ffmpeg(&raw_temp_path)?
        }
    };
    
    // Convert to mono if stereo
    let mut samples = if channels == 2 {
        println!("Converting stereo to mono");
        convert_stereo_to_mono(samples)
    } else if channels == 1 {
        println!("Audio is already mono");
        samples
    } else {
        return Err(format!("Unsupported channel count: {}", channels));
    };
    
    println!("After channel conversion: {} samples", samples.len());
    
    // Check if samples are all zero (silent audio)
    let non_zero_samples = samples.iter().filter(|&&s| s.abs() > 0.001).count();
    println!("Non-zero samples: {} out of {}", non_zero_samples, samples.len());
    
    if samples.len() < 1600 { // Less than 0.1 seconds at 16kHz
        return Err("Audio is too short (less than 0.1 seconds)".to_string());
    }
    
    if non_zero_samples == 0 {
        return Err("Audio appears to be silent (all samples are zero)".to_string());
    }
    
    if non_zero_samples < samples.len() / 100 { // Less than 1% non-zero samples
        return Err("Audio appears to be mostly silent".to_string());
    }
    
    // Resample to 16kHz if needed
    if sample_rate != 16000 {
        println!("Resampling from {}Hz to 16000Hz", sample_rate);
        samples = resample_audio(samples, sample_rate, 16000)?;
        println!("After resampling: {} samples", samples.len());
    }
    
    // Write the processed audio to temp file
    let output_spec = WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    
    let mut writer = WavWriter::create(&temp_path, output_spec)
        .map_err(|e| format!("Failed to create WAV writer: {}", e))?;
    
    for sample in samples {
        let sample_i16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
        writer.write_sample(sample_i16)
            .map_err(|e| format!("Failed to write sample: {}", e))?;
    }
    
    writer.finalize()
        .map_err(|e| format!("Failed to finalize WAV file: {}", e))?;
    
    // Return the path to the temporary file
    Ok(temp_path)
}

fn decode_with_ffmpeg(input_path: &std::path::Path) -> Result<(Vec<f32>, u32, u16), String> {
    use std::process::Command;
    
    // Create a persistent temporary WAV file for ffmpeg output
    let temp_dir = std::env::temp_dir();
    let temp_wav_path = temp_dir.join(format!("ffmpeg_output_{}.wav", uuid::Uuid::new_v4()));
    
    // Use ffmpeg to convert the input to WAV
    let output = Command::new("ffmpeg")
        .args(&[
            "-i", input_path.to_str().unwrap(),
            "-ar", "48000",  // Sample rate
            "-ac", "1",      // Mono
            "-acodec", "pcm_s16le", // PCM 16-bit little endian
            "-f", "wav",     // Output format
            "-y",            // Overwrite output file
            "-loglevel", "error", // Reduce ffmpeg output
            temp_wav_path.to_str().unwrap()
        ])
        .output()
        .map_err(|e| format!("Failed to run ffmpeg: {}. Make sure ffmpeg is installed.", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ffmpeg failed: {}", stderr));
    }
    
    // Now read the converted WAV file
    let mut reader = WavReader::open(&temp_wav_path)
        .map_err(|e| format!("Failed to read converted WAV: {}", e))?;
    
    let spec = reader.spec();
    let sample_rate = spec.sample_rate;
    let channels = spec.channels;
    
    let samples: Result<Vec<f32>, _> = match spec.sample_format {
        SampleFormat::Float => {
            reader.samples::<f32>().collect()
        },
        SampleFormat::Int => {
            reader.samples::<i16>()
                .map(|s| s.map(|sample| sample as f32 / 32768.0))
                .collect()
        }
    };
    
    let samples = samples.map_err(|e| format!("Failed to read converted samples: {}", e))?;
    Ok((samples, sample_rate, channels))
}

fn convert_stereo_to_mono(stereo_samples: Vec<f32>) -> Vec<f32> {
    stereo_samples
        .chunks_exact(2)
        .map(|chunk| (chunk[0] + chunk[1]) / 2.0)
        .collect()
}

fn resample_audio(samples: Vec<f32>, input_rate: u32, output_rate: u32) -> Result<Vec<f32>, String> {
    if input_rate == output_rate {
        return Ok(samples);
    }
    
    let params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Linear,
        oversampling_factor: 256,
        window: WindowFunction::BlackmanHarris2,
    };
    
    let mut resampler = SincFixedIn::<f32>::new(
        output_rate as f64 / input_rate as f64,
        2.0,
        params,
        samples.len(),
        1,
    ).map_err(|e| format!("Failed to create resampler: {}", e))?;
    
    let input_frames = vec![samples];
    let output_frames = resampler.process(&input_frames, None)
        .map_err(|e| format!("Failed to resample: {}", e))?;
    
    Ok(output_frames[0].clone())
}