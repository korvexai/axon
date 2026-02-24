use std::process::Command;

pub struct EnterpriseEngine;

impl EnterpriseEngine {
    pub async fn heal_and_fix(failed_action: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!(">>> [DOCTOR] Diagnosing failure for: {}", failed_action);

        // STRATEGY 1: Kill Ghost Processes
        // Prevents "Access Denied (os error 5)" by clearing old AXON instances
        let _ = Command::new("powershell")
            .args(["-Command", "Stop-Process -Name axon -Force -ErrorAction SilentlyContinue"])
            .status();

        // STRATEGY 2: Reset Folder Permissions
        let _ = Command::new("powershell")
            .args(["-Command", "icacls 'C:\\Axon' /grant 'Everyone:(OI)(CI)F' /T /C"])
            .status();

        println!(">>> [DOCTOR] Environment sanitized. Retrying execution...");
        Ok(())
    }
}
