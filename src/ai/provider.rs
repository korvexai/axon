use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

// --- DefiniÈ›ii necesare pentru fuziune ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LlmResponse {
    pub output: String,
    pub model: String,
    pub tokens_used: Option<u32>,
}

// Definim tipul pentru callback-ul de streaming
pub type StreamCallback = Box<dyn Fn(String) + Send + Sync>;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, prompt: &str, model: &str, max_tokens: u32) -> Result<LlmResponse>;
    async fn generate_stream(
        &self,
        prompt: &str,
        model: &str,
        max_tokens: u32,
        on_token: Option<StreamCallback>,
    ) -> Result<LlmResponse>;
    async fn health(&self) -> Result<()>;
}

// --- Implementarea Ollama ---

pub struct OllamaProvider {
    pub endpoint: String,
    pub timeout_seconds: u64,
}

impl OllamaProvider {
    pub fn new(endpoint: String, timeout_seconds: u64) -> Self {
        Self {
            endpoint,
            timeout_seconds,
        }
    }

    fn client(&self) -> Result<Client> {
        Ok(Client::builder()
            .timeout(Duration::from_secs(self.timeout_seconds))
            .build()?)
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    async fn generate(&self, prompt: &str, model: &str, max_tokens: u32) -> Result<LlmResponse> {
        let client = self.client()?;

        let resp = client
            .post(format!("{}/api/generate", self.endpoint))
            .json(&json!({
                "model": model,
                "prompt": prompt,
                "stream": false,
                "options": {
                    "num_predict": max_tokens
                }
            }))
            .send()
            .await
            .context("Ollama request failed")?;

        let json: serde_json::Value = resp.json().await.context("Invalid Ollama response")?;

        let output = json
            .get("response")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        Ok(LlmResponse {
            output,
            model: model.to_string(),
            tokens_used: None,
        })
    }

    async fn generate_stream(
        &self,
        prompt: &str,
        model: &str,
        max_tokens: u32,
        on_token: Option<StreamCallback>,
    ) -> Result<LlmResponse> {
        let client = self.client()?;

        let mut resp = client
            .post(format!("{}/api/generate", self.endpoint))
            .json(&json!({
                "model": model,
                "prompt": prompt,
                "stream": true,
                "options": {
                    "num_predict": max_tokens
                }
            }))
            .send()
            .await
            .context("Ollama streaming request failed")?;

        let mut final_output = String::new();

        while let Some(chunk) = resp.chunk().await? {
            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(line) {
                    if let Some(token) = parsed.get("response").and_then(|v| v.as_str()) {
                        final_output.push_str(token);

                        if let Some(cb) = &on_token {
                            cb(token.to_string());
                        }
                    }
                }
            }
        }

        Ok(LlmResponse {
            output: final_output,
            model: model.to_string(),
            tokens_used: None,
        })
    }

    async fn health(&self) -> Result<()> {
        let client = self.client()?;

        let resp = client
            .get(format!("{}/api/tags", self.endpoint))
            .send()
            .await
            .context("Ollama health check failed")?;

        if !resp.status().is_success() {
            anyhow::bail!("Ollama not healthy");
        }

        Ok(())
    }
}
