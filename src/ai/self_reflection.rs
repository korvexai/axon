use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

use crate::ai::provider::LlmProvider;
use crate::ai::models::ModelRegistry; 
use crate::ai::prompt_builder::PromptBuilder;

/// Tipuri de agenți disponibili în AXON
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentType {
    General,
    Coder,
    Analyst,
    Planner,
}

/// Un agent AI configurat
pub struct AiAgent<P: LlmProvider> {
    pub agent_type: AgentType,
    pub system_prompt: String,
    pub provider: Arc<P>,
    pub model_name: String,
    pub max_tokens: u32,
}

impl<P: LlmProvider> AiAgent<P> {

    pub async fn execute(&self, input: &str) -> Result<String> {
        let builder = PromptBuilder::new();

        let _fake_session = {
            use crate::ai::chat::{ChatSession, ChatRole};
            let mut s = ChatSession::new();
            s.push(ChatRole::User, input.to_string());
            s
        };

        let prompt = builder.build();

        let response = self.provider
            .generate(
                &prompt,
                &self.model_name,
                self.max_tokens,
            )
            .await?;

        Ok(response.output)
    }
}

/// Router care gestionează mai mulți agenți
pub struct MultiAgentRouter<P: LlmProvider> {
    agents: HashMap<AgentType, AiAgent<P>>,
}

impl<P: LlmProvider> MultiAgentRouter<P> {

    pub fn new(provider: Arc<P>, registry: ModelRegistry) -> Self {

        let mut agents = HashMap::new();

        agents.insert(
            AgentType::General,
            AiAgent {
                agent_type: AgentType::General,
                system_prompt: "You are a helpful AI assistant.".to_string(),
                provider: provider.clone(),
                model_name: registry.default_model().name.clone(),
                max_tokens: registry.default_model().max_tokens,
            },
        );

        agents.insert(
            AgentType::Coder,
            AiAgent {
                agent_type: AgentType::Coder,
                system_prompt: "You are a senior software engineer. Provide precise code.".to_string(),
                provider: provider.clone(),
                model_name: "default".to_string(),
                max_tokens: 4096,
            },
        );

        agents.insert(
            AgentType::Analyst,
            AiAgent {
                agent_type: AgentType::Analyst,
                system_prompt: "You analyze problems step by step with structured reasoning.".to_string(),
                provider: provider.clone(),
                model_name: registry.default_model().name.clone(),
                max_tokens: registry.default_model().max_tokens,
            },
        );

        agents.insert(
            AgentType::Planner,
            AiAgent {
                agent_type: AgentType::Planner,
                system_prompt: "You break complex tasks into actionable plans.".to_string(),
                provider,
                model_name: registry.default_model().name.clone(),
                max_tokens: registry.default_model().max_tokens,
            },
        );

        Self { agents }
    }

    /// Select agent based on task heuristics
    pub async fn route(&self, input: &str) -> Result<String> {

        let lower = input.to_lowercase();

        let agent_type = if lower.contains("code") ||
                            lower.contains("bug") ||
                            lower.contains("error") {
            AgentType::Coder
        } else if lower.contains("plan") ||
                  lower.contains("steps") {
            AgentType::Planner
        } else if lower.contains("analyze") {
            AgentType::Analyst
        } else {
            AgentType::General
        };

        let agent = self.agents
            .get(&agent_type)
            .expect("Agent not found");

        agent.execute(input).await
    }
}

// === AXON_COMPAT: SelfReflectionEngine stub ===
/// Compatibility stub for SelfReflectionEngine.
#[derive(Debug, Clone)]
pub struct SelfReflectionEngine;

impl SelfReflectionEngine {
    pub fn new() -> Self { SelfReflectionEngine }
}







