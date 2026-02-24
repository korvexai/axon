use std::sync::Arc;
use notify::{Watcher, RecursiveMode};
use crate::event::bus::EventSender;
use crate::event::event::{AxonEvent, LogLevel};
use crate::core::state::AppState;
use std::path::PathBuf;

pub async fn run(tx: EventSender, _state: Arc<AppState>, root: PathBuf) -> anyhow::Result<()> {
    let (watcher_tx, mut watcher_rx) = tokio::sync::mpsc::channel(100);

    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = watcher_tx.blocking_send(res);
    })?;

    watcher.watch(&root, RecursiveMode::Recursive)?;
    tracing::info!("File watcher ACTIV: MonitorizÄƒm {:?}", root);

    while let Some(res) = watcher_rx.recv().await {
        if let Ok(event) = res {
            if event.kind.is_modify() || event.kind.is_create() {
                for path in event.paths {
                    if path.extension().map_or(false, |ext| ext == "rs" || ext == "toml") {
                        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
                        if let Ok(content) = tokio::fs::read_to_string(&path).await {
                            if !content.is_empty() {
                                tracing::info!("Schimbare detectatÄƒ: {:?}", path);
                                let _ = tx.send(AxonEvent::LogDetected {
                                    level: LogLevel::Error,
                                    message: format!("ANALIZA: {}", content),
                                    source: "file_watcher".to_string(),
                                    source_file: Some(path),
                                    raw_lines: Some(content), // S-a schimbat din Vec Ã®n Option<String>
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
