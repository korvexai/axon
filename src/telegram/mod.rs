use crate::core::state::AppState;
use crate::event::bus::EventSender;
use anyhow::Result;
use std::sync::Arc;

pub async fn run(_tx: EventSender, _state: Arc<AppState>) -> Result<()> {
    Ok(())
}
