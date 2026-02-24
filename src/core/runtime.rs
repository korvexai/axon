use anyhow::Result;
use tracing::{info, error};
use std::io::{self, Write};
use crate::ai::ollama::OllamaProvider; // Import the provider that we previously fixed

pub struct AxonRuntime {
    provider: OllamaProvider,
}

impl AxonRuntime {
    // This is the function you call with .await
    pub async fn init() -> Result<Self> {
        // You can add DB/Redis checks here later
        Ok(Self {
            provider: OllamaProvider::new(),
        })
    }

    pub async fn process_message(&self, message: &str) -> Result<String> {
        self.provider.generate(message).await
    }
}

// If you want to run everything from runtime.rs (although it's atypical),
// the main function should be in src/main.rs and should look like this: