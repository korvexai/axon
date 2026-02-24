use serde::{Deserialize, Serialize};
use crate::config::schema::ModelInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistry {
    pub models: Vec<ModelInfo>,
    pub default_model_name: String,
}

impl ModelRegistry {
    pub fn from_config(default: String, coder: String, embed: String, max_tokens: u32) -> Self {
        let mut models = Vec::new();
        models.push(ModelInfo { name: default.clone(), max_tokens });
        models.push(ModelInfo { name: coder, max_tokens });
        models.push(ModelInfo { name: embed, max_tokens });

        Self {
            models,
            default_model_name: default,
        }
    }

    pub fn default_model(&self) -> ModelInfo {
        self.models.iter()
            .find(|m| m.name == self.default_model_name)
            .cloned()
            .unwrap_or_else(|| ModelInfo { name: "default".into(), max_tokens: 4096 })
    }
}
