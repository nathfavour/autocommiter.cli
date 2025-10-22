use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
    pub selected_model: Option<String>,
    pub enable_gitmoji: Option<bool>,
    pub update_gitignore: Option<bool>,
    pub gitignore_patterns: Option<Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: None,
            selected_model: Some("gpt-4o-mini".to_string()),
            enable_gitmoji: Some(false),
            update_gitignore: Some(false),
            gitignore_patterns: Some(vec![
                "*.env*".to_string(),
                ".env*".to_string(),
                "docx/".to_string(),
                ".docx/".to_string(),
            ]),
        }
    }
}

pub fn get_config_file() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow!("Could not determine home directory"))?;
    Ok(home.join(".autocommiter.json"))
}

pub fn load_config() -> Result<Config> {
    let config_file = get_config_file()?;

    if !config_file.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(&config_file)?;
    let config = serde_json::from_str(&content).unwrap_or_default();
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_file = get_config_file()?;
    let content = serde_json::to_string_pretty(&config)?;
    fs::write(&config_file, content)?;
    Ok(())
}

pub fn get_api_key() -> Result<Option<String>> {
    let config = load_config()?;
    Ok(config.api_key)
}

pub fn set_api_key(key: String) -> Result<()> {
    let mut config = load_config()?;
    config.api_key = Some(key);
    save_config(&config)
}

pub fn get_selected_model() -> Result<String> {
    let config = load_config()?;
    Ok(config
        .selected_model
        .unwrap_or_else(|| "gpt-4o-mini".to_string()))
}

pub fn set_selected_model(model: String) -> Result<()> {
    let mut config = load_config()?;
    config.selected_model = Some(model);
    save_config(&config)
}

pub fn is_gitmoji_enabled() -> Result<bool> {
    let config = load_config()?;
    Ok(config.enable_gitmoji.unwrap_or(false))
}

#[allow(dead_code)]
pub fn set_gitmoji_enabled(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.enable_gitmoji = Some(enabled);
    save_config(&config)
}

#[allow(dead_code)]
pub fn get_gitignore_patterns() -> Result<Vec<String>> {
    let config = load_config()?;
    Ok(config.gitignore_patterns.unwrap_or_else(|| {
        vec![
            "*.env*".to_string(),
            ".env*".to_string(),
            "docx/".to_string(),
            ".docx/".to_string(),
        ]
    }))
}
