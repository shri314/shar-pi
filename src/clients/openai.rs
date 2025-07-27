// Copyright (c) 2025 SharPi Contributors
// MIT License

use crate::config;
use crate::core::history;
use anyhow::{Context, Result};
use serde_json::{json, Value};

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

pub fn call_openai_with_history(input: &str, conversation_id: Option<&str>, client_name: Option<&str>) -> Result<String> {
    let mut history = history::load_history()?;

    if let Some(id) = conversation_id {
        if !history.set_active_conversation(id.to_string())? {
            return Err(anyhow::anyhow!("Conversation with ID '{}' not found", id));
        }
    }

    let (id, mut conversation) = history.ensure_active_conversation()?;
    conversation.add_user_message(input.to_string());

    let response = call_openai_with_conversation(&conversation, client_name)?;

    let parsed: Value = serde_json::from_str(&response)
        .context("Failed to parse OpenAI response as JSON")?;

    let assistant_message = parsed["choices"][0]["message"]["content"]
        .as_str()
        .context("Could not find message content in API response")?
        .to_string();

    conversation.add_assistant_message(assistant_message.clone());

    history::save_conversation(&id, &conversation)?;
    history::save_history(&history)?;

    Ok(assistant_message)
}

fn call_openai_with_conversation(conversation: &history::Conversation, client_name: Option<&str>) -> Result<String> {
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

    let mut messages = Vec::new();
    for msg in &conversation.messages {
        messages.push(json!({
            "role": msg.role,
            "content": msg.content
        }));
    }

    let request_body = json!({
        "model": client_config.model,
        "messages": messages,
        "max_tokens": client_config.max_tokens,
        "temperature": client_config.temperature
    });

    println!("Sending request with conversation history to: {}", api_url);
    println!("  URL: {}", api_url);
    println!("  Model: {}", client_config.model);
    println!("  Message count: {}", messages.len());

    let response = match ureq::post(&api_url)
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {}", client_config.api_key))
        .send_string(&request_body.to_string()) {
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
