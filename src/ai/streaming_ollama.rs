use anyhow::Result;
use std::sync::Arc;

use crate::ai::provider::LlmProvider;
use crate::config::schema::ModelInfo;

/// Result of AI self-evaluation
#[derive(Debug, Clone)]
pub struct ReflectionResult {
    pub confidence_score: f32,
    pub reasoning: String,
    pub should_retry: bool,
}

pub struct SelfReflectionEngine<P: LlmProvider> {
    provider: Arc<P>,
}

impl<P: LlmProvider> SelfReflectionEngine<P> {

    pub fn new(provider: Arc<P>) -> Self {
        Self { provider }
    }

    /// Evaluate AI response quality
    pub async fn evaluate(
        &self,
        original_prompt: &str,
        ai_output: &str,
        model: &ModelInfo,
    ) -> Result<ReflectionResult> {

        let reflection_prompt = format!(
            "You are evaluating an AI response.\n\n\
             Original prompt:\n{}\n\n\
             AI response:\n{}\n\n\
             Evaluate:\n\
             1. Is the answer logically correct?\n\
             2. Does it contain hallucinations?\n\
             3. Rate confidence from 0.0 to 1.0\n\
             4. Should it be retried?\n\
             Respond strictly in JSON:\n\
             {{\"confidence\":0.0,\"retry\":false,\"reason\":\"...\"}}",
            original_prompt,
            ai_output
        );

        let response = self.provider
            .generate(
                &reflection_prompt,
                &model.name,
                512,
            )
            .await?;

        // Try parsing JSON
        let parsed: serde_json::Value =
            serde_json::from_str(&response.output).unwrap_or_default();

        let confidence = parsed
            .get("confidence")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.5) as f32;

        let retry = parsed
            .get("retry")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let reasoning = parsed
            .get("reason")
            .and_then(|v| v.as_str())
            .unwrap_or("No reasoning provided")
            .to_string();

        Ok(ReflectionResult {
            confidence_score: confidence,
            reasoning,
            should_retry: retry,
        })
    }
}


// === AXON_COMPAT: StreamingEngine stub ===
/// Compatibility stub StreamingEngine.
/// Replace with real streaming inference.
#[derive(Debug, Clone)]
pub struct StreamingEngine;

impl StreamingEngine {
    pub fn new() -> Self { StreamingEngine }
}

