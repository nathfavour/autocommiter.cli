use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub friendly_name: Option<String>,
    pub publisher: Option<String>,
    pub summary: Option<String>,
    pub task: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedModels {
    pub models: Vec<ModelInfo>,
}

const DEFAULT_MODELS: &[(&str, &str, &str)] = &[
    (
        "gpt-4o-mini",
        "OpenAI GPT-4o mini",
        "Fast & cost-effective, great for most tasks",
    ),
    (
        "gpt-4o",
        "OpenAI GPT-4o",
        "High quality, most capable model",
    ),
    (
        "Phi-3-mini-128k-instruct",
        "Phi-3 mini 128k",
        "Lightweight, efficient open model",
    ),
    (
        "Mistral-large",
        "Mistral Large",
        "Powerful open-source model",
    ),
];

fn get_default_models() -> Vec<ModelInfo> {
    DEFAULT_MODELS
        .iter()
        .map(|(id, friendly_name, summary)| ModelInfo {
            id: id.to_string(),
            name: id.to_string(),
            friendly_name: Some(friendly_name.to_string()),
            publisher: None,
            summary: Some(summary.to_string()),
            task: Some("chat-completion".to_string()),
            tags: None,
        })
        .collect()
}

pub async fn fetch_available_models(api_key: &str) -> Result<Vec<ModelInfo>> {
    let client = Client::new();
    let url = "https://models.inference.ai.azure.com/models";

    let response = client
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?;

    if !response.status().is_success() {
        tracing::warn!(
            "Failed to fetch models (status: {}), using defaults",
            response.status()
        );
        return Ok(get_default_models());
    }

    let models_response: Vec<serde_json::Value> = response.json().await?;
    let models: Vec<ModelInfo> = models_response
        .iter()
        .filter_map(|m| {
            if m.get("task").and_then(|t| t.as_str()) == Some("chat-completion") {
                Some(ModelInfo {
                    id: m.get("name")?.as_str()?.to_string(),
                    name: m.get("name")?.as_str()?.to_string(),
                    friendly_name: m
                        .get("friendly_name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    publisher: m
                        .get("publisher")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    summary: m
                        .get("summary")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    task: m
                        .get("task")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    tags: m.get("tags").and_then(|v| v.as_array()).map(|arr| {
                        arr.iter()
                            .filter_map(|tag| tag.as_str().map(|s| s.to_string()))
                            .collect()
                    }),
                })
            } else {
                None
            }
        })
        .collect();

    if models.is_empty() {
        return Ok(get_default_models());
    }

    Ok(models)
}

fn get_models_cache_file() -> Result<std::path::PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not determine home directory"))?;
    Ok(home.join(".autocommiter.models.json"))
}

pub fn get_cached_models() -> Result<Vec<ModelInfo>> {
    let cache_file = get_models_cache_file()?;

    if !cache_file.exists() {
        return Ok(get_default_models());
    }

    let content = fs::read_to_string(&cache_file)?;
    let cached: CachedModels = serde_json::from_str(&content).unwrap_or_else(|_| CachedModels {
        models: get_default_models(),
    });

    Ok(cached.models)
}

pub fn update_cached_models(models: &[ModelInfo]) -> Result<()> {
    let cache_file = get_models_cache_file()?;
    let cached = CachedModels {
        models: models.to_vec(),
    };
    let content = serde_json::to_string_pretty(&cached)?;
    fs::write(&cache_file, content)?;
    Ok(())
}

pub async fn refresh_model_list(api_key: &str) -> Result<(bool, String, usize)> {
    match fetch_available_models(api_key).await {
        Ok(models) => {
            if models.is_empty() {
                return Ok((false, "No chat-completion models found".to_string(), 0));
            }
            update_cached_models(&models)?;
            let count = models.len();
            Ok((
                true,
                format!("Successfully fetched and cached {} models", count),
                count,
            ))
        }
        Err(e) => Ok((false, format!("Failed to fetch models: {}", e), 0)),
    }
}

pub fn list_available_models() -> Result<Vec<ModelInfo>> {
    get_cached_models()
}
