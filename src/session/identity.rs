use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionIdentity {
    pub session_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub hostname: String,
    pub version: String,
    pub total_runs: u64,
    pub crashes_fixed: u64,
    pub builds_run: u64,
}

impl SessionIdentity {
    pub fn new() -> anyhow::Result<Self> {
        let hostname = hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        Ok(Self {
            session_id: Uuid::new_v4(),
            started_at: Utc::now(),
            hostname,
            version: env!("CARGO_PKG_VERSION").to_string(),
            total_runs: 1,
            crashes_fixed: 0,
            builds_run: 0,
        })
    }

    pub fn uptime_seconds(&self) -> i64 {
        (Utc::now() - self.started_at).num_seconds()
    }

    pub fn short_id(&self) -> String {
        format!("ax_{}", &self.session_id.to_string()[..6])
    }
}
