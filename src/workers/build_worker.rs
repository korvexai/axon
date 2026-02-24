use anyhow::Result;
use std::sync::Arc;

use crate::core::state::AppState;
use crate::event::bus::EventSender;
use crate::event::event::{AxonEvent, WorkerHealth};

pub async fn run_build(
    project: String,
    _command: String,
    state: Arc<AppState>,
    tx: EventSender,
) -> Result<()> {

    state.update_worker("build_worker", WorkerHealth::Running).await;

    let start = std::time::Instant::now();

    // Simulare build
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let duration = start.elapsed().as_millis();

    tx.send(AxonEvent::BuildFinished {
        project,
        success: true,
        logs: "N/A".into(), output: "Build completed successfully".into(),
        duration_ms: duration as u64,
    })?;

    state.update_worker("build_worker", WorkerHealth::Idle).await;

    Ok(())
}

