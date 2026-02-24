// Example Orchestrator Implementation
// Shows how to handle events and emit responses for the UI

use tokio::sync::broadcast;
use std::sync::Arc;
use std::time::Duration;
use anyhow::Result;

use crate::core::state::AppState;
use crate::event::event::AxonEvent;

pub async fn run(
    mut event_rx: broadcast::Receiver<AxonEvent>,
    state: Arc<AppState>,
    event_tx: broadcast::Sender<AxonEvent>,
) -> Result<()> {

    tracing::info!("🧠 Orchestrator started");

    // ═══════════════════════════════════════════════════
    // BACKGROUND TASKS
    // ═══════════════════════════════════════════════════

    // Task 1: System metrics broadcaster (every 2s)
    tokio::spawn({
        let tx = event_tx.clone();
        async move {
            let mut interval = tokio::time::interval(Duration::from_secs(2);
            let mut sys = sysinfo::System::new_all();
            
            loop {
                interval.tick().await;
                sys.refresh_all();
                
                let cpu = sys.global_cpu_info().cpu_usage();
                let ram_gb = (sys.used_memory() as f32) / 1_073_741_824.0;
                let vram_gb = 5.2; // TODO: Get real VRAM
                let disk_gb = 847.0; // TODO: Get real disk usage
                
                // This will be picked up by ws_bridge and sent to UI
                let _ = tx.send(AxonEvent::SystemMetrics {
                    cpu,
                    ram_gb,
                    vram_gb,
                    disk_gb,
                });
            }
        }
    });

    // Task 2: Worker status reporter (every 5s)
    tokio::spawn({
        let tx = event_tx.clone();
        let state = Arc::clone(&state);
        async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5);
            
            loop {
                interval.tick().await;
                
                let workers = state.workers.read();
                for (name, info) in workers.iter() {
                    let _ = tx.send(AxonEvent::WorkerStatus {
                        name: name.clone(),
                        health: info.health.clone(),
                        task: info.task.clone(),
                    });
                }
            }
        }
    });

    // Task 3: Uptime ticker (every 1s)
    tokio::spawn({
        let state = Arc::clone(&state);
        async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1);
            
            loop {
                interval.tick().await;
                state.session.write().increment_uptime();
            }
        }
    });

    // ═══════════════════════════════════════════════════
    // MAIN EVENT LOOP
    // ═══════════════════════════════════════════════════

    loop {
        match event_rx.recv().await {
            Ok(event) => {
                if let Err(e) = handle_event(event, &state, &event_tx).await {
                    tracing::error!("Event handling error: {}", e);
                }
            }
            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                tracing::warn!("Orchestrator lagged, skipped {} events", skipped);
            }
            Err(broadcast::error::RecvError::Closed) => {
                tracing::info!("Event bus closed, shutting down");
                break;
            }
        }
    }

    Ok(())
}

async fn handle_event(
    event: AxonEvent,
    state: &Arc<AppState>,
    event_tx: &broadcast::Sender<AxonEvent>,
) -> Result<()> {

    match event {
        // ═══════════════════════════════════════════════════
        // SYSTEM
        // ═══════════════════════════════════════════════════
        
        AxonEvent::Startup => {
            tracing::info!("🚀 AXON system startup");
            
            // Emit initial log
            event_tx.send(AxonEvent::LogDetected {
                source_file: std::path::PathBuf::from("system"),
                level: crate::event::event::LogLevel::Info,
                message: "AXON v0.1.0 initialized — all workers active".to_string(),
                raw_lines: String::new(),
            })?;
        }

        AxonEvent::Shutdown => {
            tracing::info!("🛑 Shutdown requested");
            // Persist state before exit
            state.save_state()?;
        }

        // ═══════════════════════════════════════════════════
        // AI CHAT
        // ═══════════════════════════════════════════════════
        
        AxonEvent::AiRequest { request_id, prompt, model, context } => {
            tracing::info!("💬 AI request: {}", prompt);

            // Spawn AI inference task
            tokio::spawn({
                let tx = event_tx.clone();
                let model = model.unwrap_or_else(|| "mistral:7b-q4".to_string();
                
                async move {
                    // Call Ollama API
                    match crate::ai::ollama::infer(&prompt, &model).await {
                        Ok(response) => {
                            let _ = tx.send(AxonEvent::AiResponse {
                                request_id,
                                output: response,
                                model,
                                tokens_used: None,
                            });
                        }
                        Err(e) => {
                            let _ = tx.send(AxonEvent::AiError {
                                request_id,
                                error: format!("Ollama error: {}", e),
                            });
                        }
                    }
                }
            });
        }

        // ═══════════════════════════════════════════════════
        // BUILD SYSTEM
        // ═══════════════════════════════════════════════════
        
        AxonEvent::BuildRequested { project, command } => {
            tracing::info!("🔨 Build requested: {} → {}", project, command);

            // Spawn build worker
            tokio::spawn({
                let tx = event_tx.clone();
                let state = Arc::clone(state);
                let proj = project.clone();
                
                async move {
                    let start = std::time::Instant::now();

                    // Execute build command
                    let output = tokio::process::Command::new("cargo")
                        .arg("build")
                        .arg("--release")
                        .output()
                        .await
                        .unwrap();

                    let duration_ms = start.elapsed().as_millis() as u64;
                    let success = output.status.success();
                    let output_str = String::from_utf8_lossy(&output.stdout).to_string();

                    // Save to state
                    *state.last_build.write() = Some(crate::core::state::BuildInfo {
                        project: proj.clone(),
                        success,
                        duration_ms,
                        timestamp: chrono::Utc::now(),
                    });

                    // Update session stats
                    state.session.write().builds_run += 1;

                    // Emit result
                    let _ = tx.send(AxonEvent::BuildFinished {
                        project: proj,
                        success,
                        output: output_str,
                        duration_ms,
                    });

                    // Log the result
                    let level = if success {
                        crate::event::event::LogLevel::Info
                    } else {
                        crate::event::event::LogLevel::Error
                    };

                    let _ = tx.send(AxonEvent::LogDetected {
                        source_file: std::path::PathBuf::from("build"),
                        level,
                        message: format!("Build {} in {:.1}s", 
                            if success { "succeeded" } else { "failed" },
                            duration_ms as f64 / 1000.0
                        ),
                        raw_lines: String::new(),
                    });
                }
            });
        }

        // ═══════════════════════════════════════════════════
        // RAG SEARCH
        // ═══════════════════════════════════════════════════
        
        AxonEvent::RagSearch { query, request_id } => {
            tracing::info!("🔍 RAG search: {}", query);

            tokio::spawn({
                let tx = event_tx.clone();
                
                async move {
                    // TODO: Implement actual RAG search
                    // For now, return mock results
                    
                    let results = vec![
                        crate::event::event::RagResult {
                            file: "src/main.rs".to_string(),
                            line: Some(42),
                            chunk: format!("Found match for: {}", query),
                            score: 0.92,
                        },
                    ];

                    let _ = tx.send(AxonEvent::RagSearchResult {
                        request_id,
                        results,
                    });
                }
            });
        }

        // ═══════════════════════════════════════════════════
        // FIX WORKFLOW
        // ═══════════════════════════════════════════════════
        
        AxonEvent::FixProposed { alert_id, description, patch, command, confidence } => {
            tracing::info!("🔧 Fix proposed for alert {}: {}", alert_id, description);

            // Create alert in state
            state.active_alerts.write().push(crate::core::state::Alert {
                id: alert_id,
                level: crate::core::state::AlertLevel::Critical,
                title: "Fix Available".to_string(),
                message: description,
                timestamp: chrono::Utc::now(),
                fix_applied: false,
            });

            // If not in safe mode, could auto-apply
            if !state.safe_mode && confidence > 90 {
                event_tx.send(AxonEvent::FixApproved { alert_id })?;
            }
        }

        AxonEvent::FixApproved { alert_id } => {
            tracing::info!("✅ Fix approved: {}", alert_id);

            // Mark as applied
            if let Some(alert) = state.active_alerts.write()
                .iter_mut()
                .find(|a| a.id == alert_id)
            {
                alert.fix_applied = true;
            }

            // Apply the fix
            // TODO: Get patch from state, apply it, rebuild
            
            event_tx.send(AxonEvent::FixApplied {
                alert_id,
                success: true,
            })?;

            // Update session stats
            state.session.write().crashes_fixed += 1;
        }

        // ═══════════════════════════════════════════════════
        // LOG MONITORING
        // ═══════════════════════════════════════════════════
        
        AxonEvent::LogDetected { level, message, .. } => {
            // If it's an actionable error, trigger AI analysis
            if level.is_actionable() {
                tracing::warn!("⚠️ Actionable error detected: {}", message);

                // Request AI to analyze
                let req_id = uuid::Uuid::new_v4();
                event_tx.send(AxonEvent::AiRequest {
                    request_id: req_id,
                    prompt: format!(
                        "Analyze this error and propose a fix:\n\n{}",
                        message
                    ),
                    model: None,
                    context: Some("error_analysis".to_string()),
                })?;
            }
        }

        // ═══════════════════════════════════════════════════
        // TELEGRAM
        // ═══════════════════════════════════════════════════
        
        AxonEvent::TelegramCommand { chat_id, command, args } => {
            tracing::info!("📱 Telegram command: {} {:?}", command, args);

            match command.as_str() {
                "/status" => {
                    let uptime = state.session.read().uptime_seconds();
                    let msg = format!(
                        "🟢 AXON Online\n⏱ Uptime: {}s\n🔨 Builds: {}\n🐛 Fixes: {}",
                        uptime,
                        state.session.read().builds_run,
                        state.session.read().crashes_fixed
                    );

                    event_tx.send(AxonEvent::TelegramSend {
                        chat_id,
                        message: msg,
                    })?;
                }

                "/build" => {
                    event_tx.send(AxonEvent::BuildRequested {
                        project: "axon_core".to_string(),
                        command: "cargo build --release".to_string(),
                    })?;
                }

                _ => {}
            }
        }

        // ═══════════════════════════════════════════════════
        // WORKER STATUS
        // ═══════════════════════════════════════════════════
        
        AxonEvent::WorkerStatus { name, health, task } => {
            // Update worker state
            state.workers.write().insert(name.clone(), crate::core::state::WorkerInfo {
                health,
                task,
            });
        }

        _ => {
            // Unhandled events pass through
        }
    }

    Ok(())
}

