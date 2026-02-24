use anyhow::Result;
use tokio::process::Command;
use tokio::time::{timeout, Duration};
use tracing::warn;

use crate::config::schema::AxonConfig;

/// Runs a shell command with a configurable timeout
pub async fn run_command(
    bin: &str,
    args: &[String],
    config: &AxonConfig,
) -> Result<(String, String, i32)> {
    let timeout_secs = config.shell.timeout_seconds;

    let mut cmd = Command::new(bin);
    cmd.args(args);

    let future = cmd.output();

    let output = timeout(Duration::from_secs(timeout_secs), future).await;

    match output {
        Ok(Ok(out)) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            let code = out.status.code().unwrap_or(-1);

            Ok((stdout, stderr, code))
        }

        Ok(Err(e)) => {
            warn!("Command execution error: {}", e);
            Err(anyhow::anyhow!("Command execution failed"))
        }

        Err(_) => {
            warn!("Command timed out: {} {}", bin, args.join(" "));
            Err(anyhow::anyhow!("Command timed out"))
        }
    }
}