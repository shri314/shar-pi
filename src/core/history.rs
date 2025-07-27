// Copyright (c) 2025 SharPi Contributors
// MIT License

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,  // "user" or "assistant"
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new(title: String) -> (String, Self) {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();

        let conversation = Self {
            title,
            created_at: now,
            updated_at: now,
            messages: Vec::new(),
        };

        (id, conversation)
    }

    pub fn add_user_message(&mut self, content: String) {
        self.messages.push(Message {
            role: "user".to_string(),
            content,
            timestamp: Utc::now(),
        });
        self.updated_at = Utc::now();
    }

    pub fn add_assistant_message(&mut self, content: String) {
        self.messages.push(Message {
            role: "assistant".to_string(),
            content,
            timestamp: Utc::now(),
        });
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct History {
    pub active_conversation_id: Option<String>,
}

impl History {
    pub fn create_conversation(&mut self, title: String) -> Result<(String, Conversation)> {
        let (id, conversation) = Conversation::new(title);

        save_conversation(&id, &conversation)?;
        self.active_conversation_id = Some(id.clone());
        save_active_conversation_id(&self.active_conversation_id)?;

        let loaded_conversation = load_conversation(&id)?;
        Ok((id, loaded_conversation))
    }

    pub fn get_conversation(&self, id: &str) -> Result<Conversation> {
        load_conversation(id)
    }

    pub fn get_active_conversation(&self) -> Result<Option<Conversation>> {
        match &self.active_conversation_id {
            Some(id) => Ok(Some(load_conversation(id)?)),
            None => Ok(None),
        }
    }

    pub fn set_active_conversation(&mut self, id: String) -> Result<bool> {
        if conversation_exists(&id)? {
            self.active_conversation_id = Some(id);
            save_active_conversation_id(&self.active_conversation_id)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn ensure_active_conversation(&mut self) -> Result<(String, Conversation)> {
        match self.get_active_conversation()? {
            Some(conversation) => Ok((self.active_conversation_id.clone().unwrap(), conversation)),
            None => {
                let (id, conversation) = self.create_conversation("Default Conversation".to_string())?;
                Ok((id, conversation))
            }
        }
    }

    pub fn list_conversations(&self) -> Result<HashMap<String, ConversationMetadata>> {
        let conversations_dir = get_conversations_dir()?;
        let mut conversations = HashMap::new();

        if !conversations_dir.exists() {
            return Ok(conversations);
        }

        for entry in fs::read_dir(conversations_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                if let Some(id) = path.file_stem().and_then(|s| s.to_str()) {
                    match load_conversation(id) {
                        Ok(conversation) => {
                            let metadata = ConversationMetadata {
                                title: conversation.title.clone(),
                                message_count: conversation.messages.len(),
                                created_at: conversation.created_at,
                                updated_at: conversation.updated_at,
                            };
                            conversations.insert(id.to_string(), metadata);
                        },
                        Err(_) => continue,
                    }
                }
            }
        }

        Ok(conversations)
    }

    pub fn remove_conversation(&mut self, id: &str) -> Result<()> {
        if !conversation_exists(id)? {
            return Err(anyhow::anyhow!("Conversation with ID {} does not exist", id));
        }

        let path = get_conversation_path(id)?;
        fs::remove_file(&path)
            .context(format!("Failed to delete conversation file: {}", path.display()))?;

        if self.active_conversation_id.as_deref() == Some(id) {
            self.active_conversation_id = None;
            save_active_conversation_id(&self.active_conversation_id)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ConversationMetadata {
    pub title: String,
    pub message_count: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn get_sharpi_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let sharpi_dir = home.join(".sharpi");

    if !sharpi_dir.exists() {
        fs::create_dir_all(&sharpi_dir)
            .context(format!("Failed to create directory: {}", sharpi_dir.display()))?;
    }

    Ok(sharpi_dir)
}

fn get_conversations_dir() -> Result<PathBuf> {
    let conversations_dir = get_sharpi_dir()?.join("conversations");

    if !conversations_dir.exists() {
        fs::create_dir_all(&conversations_dir)
            .context(format!("Failed to create directory: {}", conversations_dir.display()))?;
    }

    Ok(conversations_dir)
}

fn get_conversation_path(id: &str) -> Result<PathBuf> {
    let path = get_conversations_dir()?.join(format!("{}.json", id));
    Ok(path)
}

fn get_active_conversation_path() -> Result<PathBuf> {
    let path = get_sharpi_dir()?.join("active_conversation.json");
    Ok(path)
}

fn conversation_exists(id: &str) -> Result<bool> {
    let path = get_conversation_path(id)?;
    Ok(path.exists())
}

pub fn save_conversation(id: &str, conversation: &Conversation) -> Result<()> {
    let path = get_conversation_path(id)?;

    let json = serde_json::to_string_pretty(conversation)
        .context("Failed to serialize conversation to JSON")?;

    fs::write(&path, json)
        .context(format!("Failed to write conversation file: {}", path.display()))?;

    Ok(())
}

fn load_conversation(id: &str) -> Result<Conversation> {
    let path = get_conversation_path(id)?;

    if !path.exists() {
        return Err(anyhow::anyhow!("Conversation with ID {} does not exist", id));
    }

    let content = fs::read_to_string(&path)
        .context(format!("Failed to read conversation file: {}", path.display()))?;

    let conversation: Conversation = serde_json::from_str(&content)
        .context("Failed to parse conversation file")?;

    Ok(conversation)
}

fn save_active_conversation_id(id: &Option<String>) -> Result<()> {
    let path = get_active_conversation_path()?;

    let json = serde_json::to_string_pretty(id)
        .context("Failed to serialize active conversation ID to JSON")?;

    fs::write(&path, json)
        .context(format!("Failed to write active conversation ID file: {}", path.display()))?;

    Ok(())
}

pub fn load_history() -> Result<History> {
    let path = get_active_conversation_path()?;

    if !path.exists() {
        return Ok(History::default());
    }

    let content = fs::read_to_string(&path)
        .context(format!("Failed to read active conversation ID file: {}", path.display()))?;

    let active_conversation_id: Option<String> = serde_json::from_str(&content)
        .context("Failed to parse active conversation ID file")?;

    Ok(History {
        active_conversation_id,
    })
}

pub fn save_history(history: &History) -> Result<()> {
    save_active_conversation_id(&history.active_conversation_id)
}
