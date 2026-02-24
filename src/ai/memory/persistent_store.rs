use anyhow::Result;
use async_trait::async_trait;

/// Standardized response from any LLM provider
#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub output: String,
    pub model: String,
    pub tokens_used: Option<u32>,
}

/// Streaming token callback
pub type StreamCallback = Box<dyn Fn(String) + Send + Sync>;

/// Trait that ALL AI providers must implement
#[async_trait]
pub trait LlmProvider: Send + Sync {

    /// Simple completion
    async fn generate(
        &self,
        prompt: &str,
        model: &str,
        max_tokens: u32,
    ) -> Result<LlmResponse>;

    /// Streaming generation (optional override)
    async fn generate_stream(
        &self,
        prompt: &str,
        model: &str,
        max_tokens: u32,
        on_token: Option<StreamCallback>,
    ) -> Result<LlmResponse> {
        // Default fallback â†’ call non-stream
        self.generate(prompt, model, max_tokens).await
    }

    /// Health check (model availability)
    async fn health(&self) -> Result<()>;
}
