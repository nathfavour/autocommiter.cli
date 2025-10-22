use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    messages: Vec<Message>,
    model: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Option<Vec<Choice>>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Option<ResponseMessage>,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: Option<String>,
}

pub async fn call_inference_api(api_key: &str, prompt: &str, model: &str) -> Result<String> {
    let client = Client::new();
    let url = "https://models.inference.ai.azure.com/chat/completions";

    let request = ChatCompletionRequest {
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful assistant that generates concise, informative git commit messages. Reply only with the commit message, nothing else.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
        model: model.to_string(),
    };

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("API request failed with status {}: {}", status, text);
    }

    let response_data: ChatCompletionResponse = response.json().await?;

    if let Some(choices) = response_data.choices {
        if let Some(choice) = choices.first() {
            if let Some(message) = &choice.message {
                if let Some(content) = &message.content {
                    return Ok(content.trim().to_string());
                }
            }
        }
    }

    anyhow::bail!("Unexpected API response format")
}

pub async fn generate_commit_message(
    api_key: &str,
    file_names: &str,
    compressed_json: &str,
    model: &str,
) -> Result<String> {
    let prompt = format!(
        "reply only with a very concise but informative commit message, and nothing else:\n\nFiles:\n{}\n\nSummaryJSON:{}",
        file_names, compressed_json
    );

    call_inference_api(api_key, &prompt, model).await
}
