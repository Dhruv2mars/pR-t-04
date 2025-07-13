use reqwest;
use serde_json::{json, Value};

const OLLAMA_URL: &str = "http://localhost:11434";

pub async fn check_ollama() -> bool {
    match reqwest::get(&format!("{}/api/tags", OLLAMA_URL)).await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

pub async fn send_prompt(prompt: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    let payload = json!({
        "model": "gemma3n:latest",
        "prompt": prompt,
        "stream": false
    });

    let response = client
        .post(&format!("{}/api/generate", OLLAMA_URL))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to send request to Ollama: {}. Make sure Ollama is running with 'ollama run gemma3n:latest'", e))?;

    if !response.status().is_success() {
        return Err(format!("Ollama API returned error: {}. Make sure 'ollama run gemma3n:latest' is running.", response.status()));
    }

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    // Parse the JSON response
    let json_response: Value = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

    // Extract the response text
    let response_content = json_response["response"]
        .as_str()
        .ok_or("No response field in Ollama response")?;

    Ok(response_content.to_string())
}