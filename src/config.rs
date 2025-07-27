// Copyright (c) 2025 SharPi Contributors
// MIT License

use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
pub struct ClientConfig {
    pub api_key: String,
    pub api_url: String,
    pub model: String,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
}

fn default_max_tokens() -> u32 {
    1000
}

fn default_temperature() -> f32 {
    0.7
}

#[derive(Deserialize, Debug, Clone)]
pub struct ClientsConfig {
    pub default: String,
    #[serde(flatten)]
    pub providers: HashMap<String, ClientConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub clients: ClientsConfig,
    #[serde(default)]
    pub tools: HashMap<String, Value>,
    #[serde(default)]
    pub commands: HashMap<String, Value>,
}

impl Config {
    pub fn get_client_config(&self, client_name: Option<&str>) -> Result<&ClientConfig> {
        let client_name = client_name.unwrap_or(&self.clients.default);
        self.clients
            .providers
            .get(client_name)
            .context(format!("Client configuration not found for '{}'", client_name))
    }
}

pub fn get_config_path() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".sharpi").join("config.toml")
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path();

    if !config_path.exists() {
        return Err(anyhow::anyhow!(
            "Configuration file not found at {}. Run 'spi config init' to create it.",
            config_path.display()
        ));
    }

    let config_str = fs::read_to_string(&config_path)
        .context(format!("Failed to read config file: {}", config_path.display()))?;

    let config: Config = toml::from_str(&config_str)
        .context("Failed to parse config file")?;

    Ok(config)
}

pub fn create_default_config(force: bool) -> Result<()> {
    let config_path = get_config_path();

    let is_overwriting = config_path.exists();

    if is_overwriting && !force {
        println!("Configuration file already exists at {}", config_path.display());
        println!("Use 'spi init --force' to overwrite the existing configuration");
        return Ok(());
    }

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .context(format!("Failed to create directory: {}", parent.display()))?;
    }

    let default_config = r#"[clients]
default = "openai"

[clients.openai]
api_key = "your-api-key-here"
api_url = "https://api.openai.com/v1"
model = "claude-3-7-sonnet"
max_tokens = 1000
temperature = 0.7

[tools]

[commands]
"#;

    fs::write(&config_path, default_config)
        .context(format!("Failed to write config file: {}", config_path.display()))?;

    if is_overwriting {
        println!("Created default configuration at {} (overwritten with --force)", config_path.display());
    } else {
        println!("Created default configuration at {}", config_path.display());
    }
    Ok(())
}
