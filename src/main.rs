use std::sync::Arc;
use std::path::Path;
use std::io::Write;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::broadcast;
use uuid::Uuid;

// Modules
mod core;
mod ai;
mod event;
mod config;
mod shell;
mod memory;
mod orchestrator;
mod util;
mod worker;
mod workers;
mod session;
mod rag;

use crate::event::event::AxonEvent;
use crate::config::loader::load_config;
use crate::core::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AXON ENGINE ONLINE | Model: Qwen2.5:7b");

    // 1️⃣ Load config
    let config_path = Path::new("config.toml");
    let config = load_config(config_path)
        .expect("Failed to load config.toml");

    let state = Arc::new(AppState::new(config));

    // 2️⃣ Channels
    let (tx, _rx) = broadcast::channel::<AxonEvent>(1024);
    let (ai_tx, ai_rx) = tokio::sync::mpsc::channel::<AxonEvent>(100);

    // 3️⃣ START CORE WS BRIDGE (IMPORTANT)
    tokio::spawn({
        let state_clone = state.clone();
        let tx_clone = tx.clone();

        async move {
            if let Err(e) = crate::core::ws_bridge::run(state_clone, tx_clone).await {
                eprintln!("WS bridge error: {:?}", e);
            }
        }
    });

    // 4️⃣ CLI RESPONSE LOGGER
    let mut rx_logger = tx.subscribe();
    tokio::spawn(async move {
        while let Ok(event) = rx_logger.recv().await {
            if let AxonEvent::AiResponse { output, .. } = event {
                println!("\n[QWEN]: {}\n", output);
                print!("> ");
                let _ = std::io::stdout().flush();
            }
        }
    });

    // 5️⃣ CLI INPUT HANDLER
    let tx_shell = ai_tx.clone();
    println!("Type something for Qwen and press Enter...");

    tokio::spawn(async move {
        let mut reader = BufReader::new(io::stdin()).lines();
        print!("> ");
        let _ = std::io::stdout().flush();

        while let Ok(Some(line)) = reader.next_line().await {
            let line = line.trim();

            if !line.is_empty() {
                let _ = tx_shell.send(AxonEvent::AiRequest {
                    id: Uuid::new_v4(),
                    prompt: line.to_string(),
                    model: Some("qwen2.5:7b".to_string()),
                    context: None,
                }).await;
            }
        }
    });

    // 6️⃣ Start AI runtime (blocking)
    crate::ai::patch_tree::run(tx, state, ai_rx).await?;

    Ok(())
}