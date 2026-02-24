use crate::core::state::AppState;
use crate::event::bus::EventSender;
use crate::event::event::AxonEvent;
use anyhow::Result;
use std::sync::Arc;
use tracing::info;

pub async fn handle_event(
    event: AxonEvent,
    state: Arc<AppState>,
    tx: EventSender,
) -> Result<()> {
    match event {
        AxonEvent::AiRequest { id, prompt, model, context } => {
            info!("Processing AI Request: {}", id);
            crate::orchestrator::router::handle_ai_request(
                id,
                prompt,
                model,
                context,
                state,
                tx,
            ).await?;
        }

        AxonEvent::LogDetected { level, message, .. } => {
            info!("Log detectat: [{:?}] {}", level, message);
            // DacÄƒ logul conÈ›ine trigger-ul, trimitem la AI
            if message.contains("ERROR") || message.contains("ANALIZA") {
                let req_id = uuid::Uuid::new_v4();
                info!("Trigger detectat in log. Trimitem la AI: {}", req_id);
                crate::orchestrator::router::handle_ai_request(
                    req_id,
                    message,
                    Some("deepseek-r1:8b".into()),
                    None,
                    state,
                    tx,
                ).await?;
            }
        }

        _ => {
            // Alte evenimente (WorkerStatus, AiResponse etc.) sunt ignorate momentan
        }
    }
    Ok(())
}
