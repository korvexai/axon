use serde::{Deserialize, Serialize};
use crate::ai::models::ModelRegistry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiTaskType { General, Coding, Analysis }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub command: String,
    pub args: Vec<String>,
}

pub struct ModelRouter {
    pub registry: ModelRegistry,
}

impl ModelRouter {
    pub fn new(registry: ModelRegistry) -> Self {
        Self { registry }
    }
}

pub async fn route_and_query(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(format!("DeepSeek Response to: {}", prompt))
}
