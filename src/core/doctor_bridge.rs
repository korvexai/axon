use std::process::Command;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

#[derive(Debug, Clone)]
pub enum HealthStatus {
    Healthy,
    Degraded(String),
    Critical(String),
}

pub struct DoctorBridge {
    project_root: String,
    last_report: Arc<RwLock<Option<String>>>,
}

impl DoctorBridge {
    pub fn new() -> Self {
        Self {
            project_root: ".".to_string(),
            last_report: Arc::new(RwLock::new(None)),
        }
    }

    /// InvocÄƒ Doctorul ca proces extern pentru a asigura izolarea memoriei
    pub async fn run_self_diagnostic(&self, auto_fix: bool) -> HealthStatus {
        info!("ðŸ©º Invocare Bridge Doctor: AnalizÄƒ integritate Ã®n curs...");

        let mut cmd = Command::new("cargo");
        cmd.arg("run").arg("--bin").arg("axon_doctor");
        
        if auto_fix {
            cmd.arg("--").arg("--fix");
        }

        match cmd.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut report = self.last_report.write().await;
                *report = Some(stdout.to_string());

                if output.status.success() {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Critical("Doctorul a detectat erori structurale!".to_string())
                }
            }
            Err(e) => HealthStatus::Critical(format!("EÈ™ec la invocarea Doctorului: {}", e)),
        }
    }

    /// VerificÄƒ dacÄƒ mediul de rulare este valid Ã®nainte de start-ul motorului
    pub fn pre_flight_check() -> Result<(), String> {
        println!("ðŸ›¡ï¸ AXON Pre-flight Check...");
        // VerificÄƒri rapide de sistem (fÄƒrÄƒ cargo check)
        if !std::path::Path::new("Cargo.toml").exists() {
            return Err("Eroare FatalÄƒ: Cargo.toml lipseÈ™te din radacinÄƒ!".into());
        }
        Ok(())
    }
}

impl Default for DoctorBridge { fn default() -> Self { Self::new() } }
