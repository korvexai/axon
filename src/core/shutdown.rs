use tokio::sync::watch;

pub type ShutdownSender = watch::Sender<bool>;
pub type ShutdownReceiver = watch::Receiver<bool>;

pub fn create_shutdown_signal() -> (ShutdownSender, ShutdownReceiver) {
    watch::channel(false)
}

/// Register Ctrl+C handler â€” sends shutdown signal when triggered.
pub async fn handle_ctrl_c(tx: ShutdownSender) {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl+c");

    tracing::info!("Ctrl+C received â€” initiating graceful shutdown");

    let _ = tx.send(true);
}
