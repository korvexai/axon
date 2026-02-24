use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AxonConfig {
    pub ai: AiConfig,
    pub discord: DiscordConfig,
    pub telegram: TelegramConfig,
    pub filesystem: FileConfig,
    pub shell: ShellConfig,
}

impl Default for AxonConfig {
    fn default() -> Self {
        Self {
            ai: AiConfig::default(),
            discord: DiscordConfig::default(),
            telegram: TelegramConfig::default(),
            filesystem: FileConfig::default(),
            shell: ShellConfig::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AiConfig {
    pub provider: String,
    pub model: String,

    pub endpoint: Option<String>,
    pub api_key: Option<String>,
    pub ollama_endpoint: Option<String>,

    pub timeout_seconds: u64,

    pub default_model: ModelInfo,
    pub coder_model: ModelInfo,
    pub embed_model: ModelInfo,

    pub max_tokens: u32,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: "ollama".into(),
            model: "Qwen2.5:7b".into(),
            endpoint: None,
            api_key: None,
            ollama_endpoint: Some("http://127.0.0.1:11434".into()),
            timeout_seconds: 60,
            default_model: ModelInfo::default(),
            coder_model: ModelInfo::default(),
            embed_model: ModelInfo {
                name: "nomic-embed-text".into(),
                max_tokens: 2048,
            },
            max_tokens: 4096,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ModelInfo {
    pub name: String,
    pub max_tokens: u32,
}

impl Default for ModelInfo {
    fn default() -> Self {
        Self {
            name: "Qwen2.5:7b".into(),
            max_tokens: 4096,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct DiscordConfig {
    pub token: String,
    pub channel_id: u64,
}

impl Default for DiscordConfig {
    fn default() -> Self {
        Self {
            token: String::new(),
            channel_id: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub admin_id: i64,
}

impl Default for TelegramConfig {
    fn default() -> Self {
        Self {
            bot_token: String::new(),
            admin_id: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct FileConfig {
    pub watch_path: String,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            watch_path: ".".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ShellConfig {
    pub allowed_commands: Vec<String>,
    pub timeout_seconds: u64,
    pub require_approval_for: Vec<String>,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            allowed_commands: vec![],
            timeout_seconds: 60,
            require_approval_for: vec![],
        }
    }
}