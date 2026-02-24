use anyhow::Result;
use std::sync::Arc;

use crate::ai::provider::LlmProvider;
use crate::config::schema::ModelInfo;

/// High-level streaming helper for Ollama
pub struct StreamingEngine<P: LlmProvider> {
    provider: Arc<P>,
}

impl<P: LlmProvider> StreamingEngine<P> {

    pub fn new(provider: Arc<P>) -> Self {
        Self { provider }
    }

    /// Generate response with live streaming to WebSocket UI
    pub async fn generate_streaming(
        &self,
        prompt: String,
        model: &ModelInfo,
    ) -> Result<String> {

        let mut full_output = String::new();

        let response = self.provider
            .generate_stream(
                &prompt,
                &model.name,
                model.max_tokens,
                Some(Box::new(|token: String| {
                })),
            )
            .await?;

        full_output.push_str(&response.output);

        Ok(full_output)
    }
}


// === AXON_COMPAT: ToolRouter stub ===
/// Compatibility stub.
/// If you already have a real ToolRouter elsewhere, re-export it instead.
#[derive(Debug, Clone)]
pub struct ToolRouter;

impl ToolRouter {
    pub fn new() -> Self { ToolRouter }
}


