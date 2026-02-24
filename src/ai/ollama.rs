use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::time::Duration;

pub struct OllamaProvider;

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

impl OllamaProvider {
    pub fn new() -> Self {
        OllamaProvider
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        // Configurăm clientul cu timeout pentru a preveni blocajele de runtime
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        // Folosim 127.0.0.1 pentru stabilitate pe Windows
        let res = client
            .post("http://127.0.0.1:11434/api/generate")
            .json(&OllamaRequest {
                model: "qwen2.5:7b".to_string(), // Sincronizat cu 'ollama list'
                prompt: prompt.to_string(),
                stream: false,
            })
            .send()
            .await?;

        // Verificăm dacă statusul este succes înainte de a parsa JSON-ul
        if !res.status().is_success() {
            let err_text = res.text().await?;
            return Err(anyhow::anyhow!("Ollama API Error: {}", err_text));
        }

        let body: OllamaResponse = res.json().await?;
        Ok(body.response)
    }
}