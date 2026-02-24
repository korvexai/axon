use uuid::Uuid;
use std::sync::Arc;
use crate::event::bus::EventSender;
use crate::event::event::{AxonEvent};
use crate::core::state::AppState;

/// Handler aliniat cu handler.rs (6 argumente)
pub async fn handle_ai_request(
    request_id: Uuid,
    prompt: String,
    model: Option<String>,
    context: Option<String>,
    _state: Arc<AppState>,
    tx: EventSender,
) -> anyhow::Result<()> {
    tracing::info!("AI request received: {}", request_id);
    
    tx.send(AxonEvent::AiRequest {
        id: request_id,
        prompt,
        model: model.or(Some("deepseek-r1:8b".into())),
        context,
    })?;

    Ok(())
}

/// Handler pentru fisiere detectate de Watcher
pub async fn handle_file_detected(
    path: String,
    content: String,
    tx: EventSender,
) -> anyhow::Result<()> {
    let req_id = Uuid::new_v4();
    tracing::info!("AXON ROUTER: Analiza automata pentru: {}", path);

    tx.send(AxonEvent::AiRequest {
        id: req_id,
        prompt: format!("Analizeaza acest cod Rust:\nPath: {}\n\n{}", path, content),
        model: Some("deepseek-r1:8b".into()),
        context: None,
    })?;

    Ok(())
}
