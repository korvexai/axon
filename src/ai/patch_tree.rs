use std::sync::Arc;
use anyhow::Result;
use tracing::{info, warn};
use tokio::sync::mpsc;

use crate::core::state::AppState;
use crate::event::bus::EventSender;
use crate::event::event::{AxonEvent, WorkerHealth};
use crate::ai::ollama::OllamaProvider;

pub async fn run(
    tx: EventSender,
    state: Arc<AppState>,
    mut rx: mpsc::Receiver<AxonEvent>,
) -> Result<()> {

    // Register worker
    state.update_worker("ai_bridge", WorkerHealth::Running).await;

    let provider = OllamaProvider::new();

    info!("AI Bridge ACTIVE (Ollama)");

    while let Some(event) = rx.recv().await {

        if let AxonEvent::AiRequest { prompt, id, model, context } = event {

            info!("Processing AI request [{}]", id);

            let response = match provider.generate(&prompt).await {
                Ok(res) => res,
                Err(e) => {
                    warn!("Ollama error: {}", e);
                    format!("AI error: {}", e)
                }
            };

            let _ = tx.send(AxonEvent::AiResponse {
    request_id: id,
    output: response.clone(),
    model: "qwen2.5:7b".to_string(),
    context: None,
    response: response,
});
        }
    }

    Ok(())
}


