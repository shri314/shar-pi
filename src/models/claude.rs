use anyhow::{Context, Result};
use std::env;

/// Makes a request to the AI API with the given input text
/// and returns the response text
pub fn call_claude(input: &str) -> Result<String> {
    // Get API key from environment variable
    let api_key = env::var("AI_API_KEY")
        .context("AI_API_KEY environment variable not set")?;
    
    // Get API URL from environment variable and ensure it ends with the correct endpoint
    let base_url = env::var("AI_API_URL")
        .context("AI_API_URL environment variable not set")?;
    
    // Ensure the URL ends with a trailing slash
    let base_url = if base_url.ends_with('/') {
        base_url
    } else {
        format!("{}/", base_url)
    };
    
    // Append the chat/completions endpoint if not already included
    let api_url = if base_url.ends_with("chat/completions/") {
        base_url
    } else {
        format!("{}chat/completions", base_url)
    };
    
    // Get model from environment variable
    let model = env::var("AI_MODEL")
        .context("AI_MODEL environment variable not set")?;
    
    // Construct the JSON request body for OpenAI-compatible API
    let request_body = format!(
        r#"{{
            "model": "{model}",
            "messages": [
                {{
                    "role": "user",
                    "content": "{content}"
                }}
            ],
            "max_tokens": 1000
        }}"#,
        model = model,
        content = input.replace("\"", "\\\"").replace("\n", "\\n")
    );

    println!("Sending request to: {}", api_url);
    
    // Show debug info
    println!("Request details:");
    println!("  URL: {}", api_url);
    println!("  Model: {}", model);
    println!("  Request body: {}", request_body);
    
    // Make the API request (using OpenAI-compatible format)
    let response = match ureq::post(&api_url)
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {}", api_key))
        .send_string(&request_body) {
            Ok(res) => res,
            Err(ureq::Error::Status(code, res)) => {
                // Handle HTTP error response
                let error_body = res.into_string()
                    .unwrap_or_else(|_| "Could not read error response".to_string());
                return Err(anyhow::anyhow!(
                    "API request failed with status {}: {}", 
                    code, error_body
                ));
            },
            Err(err) => {
                // Handle network/transport errors
                return Err(anyhow::anyhow!(
                    "Network error while making API request: {}", err
                ));
            }
        };

    // Parse the response body
    let response_text = response.into_string()
        .context("Failed to read response body")?;

    // For now, return the raw response
    // In future iterations, we could parse this to extract just the assistant's message
    Ok(response_text)
}
