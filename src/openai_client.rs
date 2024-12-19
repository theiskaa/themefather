#![allow(dead_code)]

use futures_util::StreamExt;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

/// Available OpenAI models
#[derive(Debug, Clone)]
pub enum OpenAIModel {
    Gpt4o,
    Gpt4oMini,
}

impl OpenAIModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            OpenAIModel::Gpt4o => "gpt-4o",
            OpenAIModel::Gpt4oMini => "gpt-4o-mini",
        }
    }
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunkResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    system_fingerprint: String,
    pub choices: Vec<ChunkChoice>,
}

#[derive(Debug, Deserialize)]
pub struct ChunkChoice {
    index: u32,
    pub delta: DeltaContent,
    logprobs: Option<serde_json::Value>,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeltaContent {
    pub content: Option<String>,
}

pub struct OpenAIService {
    client: Client,
    api_key: String,
    timeout: Duration,
}

impl OpenAIService {
    const BASE_URL: &'static str = "https://api.openai.com/v1";

    /// Create a new OpenAI service instance
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let api_key = env::var("OPENAI_API_KEY")?;
        let timeout = Duration::from_secs(600);

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder()
            .timeout(timeout)
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            api_key,
            timeout,
        })
    }

    /// Create a streaming chat completion
    pub async fn create_chat_completion(
        &self,
        model: OpenAIModel,
        messages: Vec<Message>,
        temperature: f32,
    ) -> Result<
        impl futures_util::Stream<
            Item = Result<Option<String>, Box<dyn std::error::Error + Send + Sync>>,
        > + Send,
        Box<dyn std::error::Error>,
    > {
        let request = ChatCompletionRequest {
            model: model.as_str().to_string(),
            messages,
            temperature,
            stream: true,
        };

        let response = self
            .client
            .post(&format!("{}/chat/completions", Self::BASE_URL))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API request failed: {}", error_text).into());
        }

        let stream = response.bytes_stream().map(move |chunk| {
            let chunk = chunk.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if !line.starts_with("data: ") {
                    continue;
                }

                let json_str = &line[6..];
                if json_str.is_empty() {
                    continue;
                }
                if json_str == "[DONE]" {
                    return Ok(None);
                }

                let response: ChatCompletionChunkResponse = serde_json::from_str(json_str)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                if let Some(choice) = response.choices.first() {
                    if let Some(content) = &choice.delta.content {
                        return Ok(Some(content.clone()));
                    }
                }
            }
            Ok(Some(String::new()))
        });

        Ok(stream)
    }

    /// Create a non-streaming chat completion
    pub async fn create_chat_completion_non_stream(
        &self,
        model: OpenAIModel,
        messages: Vec<Message>,
        temperature: f32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request = ChatCompletionRequest {
            model: model.as_str().to_string(),
            messages,
            temperature,
            stream: false,
        };

        let response = self
            .client
            .post(&format!("{}/chat/completions", Self::BASE_URL))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API request failed: {}", error_text).into());
        }

        let completion: ChatCompletionResponse = response.json().await?;
        if let Some(choice) = completion.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err("No completion choices returned".into())
        }
    }
}
