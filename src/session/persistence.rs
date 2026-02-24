use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use tracing::info;

use crate::session::identity::SessionIdentity;

pub fn load_or_create(path: &Path) -> Result<SessionIdentity> {
    if path.exists() {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read session file: {}", path.display()))?;

        let identity: SessionIdentity =
            serde_json::from_str(&content).context("Invalid session JSON format")?;

        Ok(identity)
    } else {
        let identity = SessionIdentity::new()?;

        let json =
            serde_json::to_string_pretty(&identity).context("Failed to serialize session")?;

        fs::write(path, json)
            .with_context(|| format!("Failed to write session file: {}", path.display()))?;

        info!("New session created: {}", identity.short_id());

        Ok(identity)
    }
}
