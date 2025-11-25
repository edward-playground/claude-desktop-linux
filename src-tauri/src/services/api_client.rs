//! API client for communicating with the Anthropic API
//!
//! Supports both regular and streaming message requests.

use futures::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::error::{AppError, Result};
use crate::models::{ApiResponse, ApiUsage, Message, MessageRole, StreamChunk};

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const DEFAULT_MAX_TOKENS: u32 = 4096;

/// API client for Anthropic Claude
pub struct ApiClient {
    client: reqwest::Client,
    api_key: String,
    timeout: std::time::Duration,
}

/// Request body for the Anthropic API
#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    messages: Vec<ApiMessage>,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    stream: bool,
}

/// Message format for the API
#[derive(Debug, Serialize)]
struct ApiMessage {
    role: String,
    content: String,
}

/// Response from the Anthropic API
#[derive(Debug, Deserialize)]
struct ApiResponseBody {
    id: String,
    content: Vec<ContentBlock>,
    model: String,
    stop_reason: Option<String>,
    usage: Option<UsageResponse>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ContentBlock {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UsageResponse {
    input_tokens: i32,
    output_tokens: i32,
}

/// Streaming event types
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum StreamEvent {
    #[serde(rename = "message_start")]
    MessageStart { message: MessageStartData },
    #[serde(rename = "content_block_start")]
    ContentBlockStart { content_block: ContentBlock },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta { delta: DeltaContent },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop {},
    #[serde(rename = "message_delta")]
    MessageDelta {
        delta: MessageDeltaData,
        usage: Option<UsageResponse>,
    },
    #[serde(rename = "message_stop")]
    MessageStop {},
    #[serde(rename = "ping")]
    Ping {},
    #[serde(rename = "error")]
    Error { error: ErrorData },
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MessageStartData {
    id: String,
    model: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DeltaContent {
    #[serde(rename = "type")]
    delta_type: String,
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MessageDeltaData {
    stop_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ErrorData {
    #[serde(rename = "type")]
    error_type: String,
    message: String,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(api_key: String, timeout_secs: u64) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build()
            .map_err(|e| AppError::Network(e.to_string()))?;

        Ok(Self {
            client,
            api_key,
            timeout: std::time::Duration::from_secs(timeout_secs),
        })
    }

    /// Create a new API client with proxy
    pub fn new_with_proxy(api_key: String, timeout_secs: u64, proxy_url: &str) -> Result<Self> {
        let proxy = reqwest::Proxy::all(proxy_url).map_err(|e| AppError::Config(e.to_string()))?;

        let client = reqwest::Client::builder()
            .proxy(proxy)
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build()
            .map_err(|e| AppError::Network(e.to_string()))?;

        Ok(Self {
            client,
            api_key,
            timeout: std::time::Duration::from_secs(timeout_secs),
        })
    }

    /// Get default headers for API requests
    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(&self.api_key).unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        headers.insert(
            "anthropic-version",
            HeaderValue::from_static(ANTHROPIC_VERSION),
        );
        headers
    }

    /// Validate an API key by making a minimal request
    pub async fn validate_api_key(api_key: &str) -> Result<bool> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::Network(e.to_string()))?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(api_key)
                .map_err(|_| AppError::InvalidInput("Invalid API key format".to_string()))?,
        );
        headers.insert(
            "anthropic-version",
            HeaderValue::from_static(ANTHROPIC_VERSION),
        );

        let request = ApiRequest {
            model: "claude-3-haiku-20240307".to_string(), // Use cheapest model for validation
            messages: vec![ApiMessage {
                role: "user".to_string(),
                content: "Hi".to_string(),
            }],
            max_tokens: 1,
            system: None,
            temperature: None,
            stream: false,
        };

        let response = client
            .post(ANTHROPIC_API_URL)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        match response.status().as_u16() {
            200 => Ok(true),
            401 => Err(AppError::Authentication("Invalid API key".to_string())),
            429 => Err(AppError::RateLimited("Rate limited".to_string())),
            _ => {
                let error_text = response.text().await.unwrap_or_default();
                Err(AppError::Api(format!("API error: {}", error_text)))
            }
        }
    }

    /// Send a message and get a complete response
    pub async fn send_message(
        &self,
        model: &str,
        messages: &[Message],
        system_prompt: Option<&str>,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> Result<ApiResponse> {
        let api_messages: Vec<ApiMessage> = messages
            .iter()
            .filter(|m| m.role != MessageRole::System)
            .map(|m| ApiMessage {
                role: m.role.to_string(),
                content: m.content.clone(),
            })
            .collect();

        let request = ApiRequest {
            model: model.to_string(),
            messages: api_messages,
            max_tokens: max_tokens.unwrap_or(DEFAULT_MAX_TOKENS),
            system: system_prompt.map(String::from),
            temperature,
            stream: false,
        };

        let response = self
            .client
            .post(ANTHROPIC_API_URL)
            .headers(self.get_headers())
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return match status.as_u16() {
                401 => Err(AppError::Authentication("Invalid API key".to_string())),
                429 => Err(AppError::RateLimited(error_text)),
                _ => Err(AppError::Api(format!(
                    "API error ({}): {}",
                    status, error_text
                ))),
            };
        }

        let body: ApiResponseBody = response.json().await?;

        let content = body
            .content
            .into_iter()
            .filter_map(|c| c.text)
            .collect::<Vec<_>>()
            .join("");

        Ok(ApiResponse {
            id: body.id,
            content,
            model: body.model,
            stop_reason: body.stop_reason,
            usage: body.usage.map(|u| ApiUsage {
                input_tokens: u.input_tokens,
                output_tokens: u.output_tokens,
            }),
        })
    }

    /// Send a message and stream the response
    pub async fn send_message_streaming(
        &self,
        model: &str,
        messages: &[Message],
        system_prompt: Option<&str>,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> Result<mpsc::Receiver<StreamChunk>> {
        let (tx, rx) = mpsc::channel(100);

        let api_messages: Vec<ApiMessage> = messages
            .iter()
            .filter(|m| m.role != MessageRole::System)
            .map(|m| ApiMessage {
                role: m.role.to_string(),
                content: m.content.clone(),
            })
            .collect();

        let request = ApiRequest {
            model: model.to_string(),
            messages: api_messages,
            max_tokens: max_tokens.unwrap_or(DEFAULT_MAX_TOKENS),
            system: system_prompt.map(String::from),
            temperature,
            stream: true,
        };

        let client = self.client.clone();
        let headers = self.get_headers();
        let timeout = self.timeout;

        tokio::spawn(async move {
            let result = stream_response(client, headers, request, tx.clone(), timeout).await;
            if let Err(e) = result {
                let _ = tx
                    .send(StreamChunk {
                        chunk_type: "error".to_string(),
                        delta: None,
                        stop_reason: None,
                        error: Some(e.to_string()),
                    })
                    .await;
            }
        });

        Ok(rx)
    }
}

/// Internal function to handle streaming response
async fn stream_response(
    client: reqwest::Client,
    headers: HeaderMap,
    request: ApiRequest,
    tx: mpsc::Sender<StreamChunk>,
    _timeout: std::time::Duration,
) -> Result<()> {
    let request_builder = client
        .post(ANTHROPIC_API_URL)
        .headers(headers)
        .json(&request);

    let mut event_source = EventSource::new(request_builder)
        .map_err(|e| AppError::Network(format!("Failed to create event source: {}", e)))?;

    while let Some(event) = event_source.next().await {
        match event {
            Ok(Event::Open) => {
                tracing::debug!("SSE connection opened");
            }
            Ok(Event::Message(message)) => {
                if let Ok(stream_event) = serde_json::from_str::<StreamEvent>(&message.data) {
                    match stream_event {
                        StreamEvent::ContentBlockDelta { delta } => {
                            if let Some(text) = delta.text {
                                let chunk = StreamChunk {
                                    chunk_type: "content_block_delta".to_string(),
                                    delta: Some(text),
                                    stop_reason: None,
                                    error: None,
                                };
                                if tx.send(chunk).await.is_err() {
                                    break;
                                }
                            }
                        }
                        StreamEvent::MessageDelta { delta, .. } => {
                            if delta.stop_reason.is_some() {
                                let chunk = StreamChunk {
                                    chunk_type: "message_delta".to_string(),
                                    delta: None,
                                    stop_reason: delta.stop_reason,
                                    error: None,
                                };
                                let _ = tx.send(chunk).await;
                            }
                        }
                        StreamEvent::MessageStop {} => {
                            let chunk = StreamChunk {
                                chunk_type: "message_stop".to_string(),
                                delta: None,
                                stop_reason: None,
                                error: None,
                            };
                            let _ = tx.send(chunk).await;
                            break;
                        }
                        StreamEvent::Error { error } => {
                            let chunk = StreamChunk {
                                chunk_type: "error".to_string(),
                                delta: None,
                                stop_reason: None,
                                error: Some(error.message),
                            };
                            let _ = tx.send(chunk).await;
                            break;
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                tracing::error!("SSE error: {:?}", e);
                let chunk = StreamChunk {
                    chunk_type: "error".to_string(),
                    delta: None,
                    stop_reason: None,
                    error: Some(e.to_string()),
                };
                let _ = tx.send(chunk).await;
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_message_serialization() {
        let msg = ApiMessage {
            role: "user".to_string(),
            content: "Hello".to_string(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"role\":\"user\""));
        assert!(json.contains("\"content\":\"Hello\""));
    }
}
