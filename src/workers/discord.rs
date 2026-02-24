use std::sync::Arc;
use anyhow::Result;
use crate::core::state::AppState;
use crate::event::bus::EventSender;

pub async fn run(
    _tx: EventSender,
    _state: Arc<AppState>,
) -> Result<()> {
    Ok(())
}
