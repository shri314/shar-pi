// Copyright (c) 2025 SharPi Contributors
// MIT License

use crate::config;
use anyhow::{Context, Result};

pub fn call_openai(input: &str, client_name: Option<&str>) -> Result<String> {
    let config = config::load_config()?;
    let client_config = config.get_client_config(client_name)?;
    
    let base_url = if client_config.api_url.ends_with('/') {
        client_config.api_url.clone()
    } else {
        format!("{}/", client_config.api_url)
    };
    
    let api_url = if base_url.ends_with("chat/completions/") {
        base_url
    } else {
        format!("{}chat/completions", base_url)
    };
    
    let request_body = format!(
        r#"{{
            "model": "{model}",
            "messages": [
                {{
                    "role": "user",
                    "content": "{content}"
                }}
            ],
            "max_tokens": {max_tokens},
            "temperature": {temperature}
        }}"#,
        model = client_config.model,
        content = input.replace("\"", "\\\"").replace("\n", "\\n"),
        max_tokens = client_config.max_tokens,
        temperature = client_config.temperature
    );

    println!("Sending request to: {}", api_url);
    println!("Request details:");
    println!("  URL: {}", api_url);
    println!("  Model: {}", client_config.model);
    println!("  Request body: {}", request_body);
    
    let response = match ureq::post(&api_url)
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {}", client_config.api_key))
        .send_string(&request_body) {
            Ok(res) => res,
            Err(ureq::Error::Status(code, res)) => {
                let error_body = res.into_string()
                    .unwrap_or_else(|_| "Could not read error response".to_string());
                return Err(anyhow::anyhow!(
                    "API request failed with status {}: {}", 
                    code, error_body
                ));
            },
            Err(err) => {
                return Err(anyhow::anyhow!(
                    "Network error while making API request: {}", err
                ));
            }
        };

    let response_text = response.into_string()
        .context("Failed to read response body")?;

    Ok(response_text)
}
