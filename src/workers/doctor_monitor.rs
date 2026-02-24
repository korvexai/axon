use crate::doctor::engine::EnterpriseEngine;
use crate::doctor::mode::EngineMode;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, error};

pub async fn start_health_check() {
    info!("Sistemul de Auto-Monitorizare Axon a pornit.");
    
    let mut engine = EnterpriseEngine::new(EngineMode::Audit);
    
    loop {
        info!("Doctor: ÃŽnceperea scanÄƒrii de integritate...");
        
        // ExecutÄƒ auditul pe folderul curent (.)
        match engine.run_with_repo(".") {
            Ok(_) => info!("Doctor: Integritate verificatÄƒ. Sistem stabil."),
            Err(e) => error!("Doctor: ALERTÄ‚! Anomalie detectatÄƒ Ã®n cod: {}", e),
        }

        // AÈ™teaptÄƒ 10 minute pÃ¢nÄƒ la urmÄƒtoarea scanare
        sleep(Duration::from_secs(600)).await;
    }
}
