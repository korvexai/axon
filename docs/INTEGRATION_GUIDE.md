# AXON UI ↔ Engine Integration Guide

## 🎯 Mission
Connect your beautiful HTML dashboard to the real Rust engine with **live WebSocket streaming**.

---

## 📋 Prerequisites

1. **Rust** installed (`rustc 1.75+`)
2. **Ollama** running on `localhost:11434` (optional for AI)
3. Your AXON Rust project structure:
   ```
   axon/
   ├── src/
   │   ├── main.rs
   │   ├── core/
   │   │   ├── mod.rs
   │   │   ├── state.rs
   │   │   └── ws_bridge.rs  ← NEW
   │   ├── event/
   │   │   ├── mod.rs
   │   │   ├── bus.rs
   │   │   └── event.rs
   │   └── orchestrator/
   │       └── handler.rs
   ├── Cargo.toml
   └── dashboard/
       └── index.html  ← Your UI
   ```

---

## 🚀 Step 1: Update Cargo.toml

Replace your `Cargo.toml` with the provided one that includes:
- `axum` for WebSocket server
- `tokio` for async runtime
- `serde`/`serde_json` for serialization
- All other required dependencies

```bash
cp Cargo.toml your-axon-project/Cargo.toml
```

---

## 🚀 Step 2: Add WebSocket Bridge

Copy the production WebSocket bridge to your project:

```bash
cp ws_bridge_production.rs your-axon-project/src/core/ws_bridge.rs
```

Then update `src/core/mod.rs`:

```rust
pub mod state;
pub mod ws_bridge;  // ← ADD THIS
pub mod runtime;
```

---

## 🚀 Step 3: Update Your Event System

Make sure your `event::event::AxonEvent` enum derives `Clone`:

```rust
#[derive(Debug, Clone)]  // ← Must have Clone
pub enum AxonEvent {
    // ... your events
}
```

---

## 🚀 Step 4: Switch from mpsc to broadcast

In `event/bus.rs`, change from `mpsc` to `broadcast`:

```rust
use tokio::sync::broadcast;

pub type EventSender = broadcast::Sender<AxonEvent>;
pub type EventReceiver = broadcast::Receiver<AxonEvent>;

pub fn create_event_bus(capacity: usize) -> (EventSender, EventReceiver) {
    broadcast::channel(capacity)
}
```

**Why?** Multiple workers need to listen to the same events (fan-out pattern).

---

## 🚀 Step 5: Update main.rs

Your `main.rs` should spawn the WebSocket bridge:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // ... existing setup ...

    // Create event bus
    let (event_tx, _event_rx) = event::bus::create_event_bus(2048);

    // Start WebSocket bridge (non-blocking)
    let ws_state = Arc::clone(&state);
    let ws_tx = event_tx.clone();
    tokio::spawn(async move {
        if let Err(e) = core::ws_bridge::run(ws_state, ws_tx).await {
            tracing::warn!("WebSocket bridge stopped: {}", e);
        }
    });

    // Start workers
    core::runtime::start_workers(event_tx.clone(), Arc::clone(&state)).await;

    // Run orchestrator
    let mut orchestrator_rx = event_tx.subscribe();
    orchestrator::handler::run(orchestrator_rx, state, event_tx).await?;

    Ok(())
}
```

---

## 🚀 Step 6: Update Your Config

Add WebSocket config to your `config.toml`:

```toml
[websocket]
enabled = true
bind = "127.0.0.1"
port = 7878
```

And update your config struct:

```rust
#[derive(Debug, Deserialize)]
pub struct Config {
    // ... existing fields
    pub websocket: WebSocketConfig,
}

#[derive(Debug, Deserialize)]
pub struct WebSocketConfig {
    pub enabled: bool,
    pub bind: String,
    pub port: u16,
}
```

---

## 🚀 Step 7: Update Your HTML Dashboard

Replace the entire `<script>` section in your HTML with:

```html
<script src="axon_ui_client.js"></script>
<script>
  // Keep your existing page switching, clock, etc.
  // But REMOVE all simulation code:
  // - simulateBuild() implementation → keep function name but body is now: axon.rebuild()
  // - CPU randomizer setInterval
  // - fake log rate
  // - Anthropic API fetch
  
  // The axon_ui_client.js handles all real communication
</script>
```

**Key changes in your existing functions:**

```javascript
// OLD:
function simulateBuild() {
  // fake log insertion
}

// NEW:
function simulateBuild() {
  axon.rebuild();  // ← Send real command to engine
}

// OLD:
async function sendMessage() {
  const response = await fetch("https://api.anthropic.com/...");
  // ...
}

// NEW:
function sendMessage() {
  const input = document.getElementById('chat-input');
  const msg = input.value.trim();
  if (!msg) return;
  
  axon.sendChat(msg);  // ← Send to engine, not Anthropic
  
  input.value = '';
}
```

---

## 🚀 Step 8: Update Orchestrator to Emit Events

Your orchestrator needs to emit events for the UI:

```rust
pub async fn run(
    mut event_rx: broadcast::Receiver<AxonEvent>,
    state: Arc<AppState>,
    event_tx: broadcast::Sender<AxonEvent>,
) -> anyhow::Result<()> {

    // Emit system status every 2 seconds
    tokio::spawn({
        let tx = event_tx.clone();
        async move {
            let mut interval = tokio::time::interval(Duration::from_secs(2));
            loop {
                interval.tick().await;
                
                // Get system metrics (use sysinfo crate)
                let mut sys = sysinfo::System::new_all();
                sys.refresh_all();
                
                let cpu = sys.global_cpu_info().cpu_usage();
                let ram_gb = (sys.used_memory() as f32) / 1_073_741_824.0;
                
                // This will be converted to WsEvent by ws_bridge
                let _ = tx.send(AxonEvent::SystemMetrics { cpu, ram_gb });
            }
        }
    });

    loop {
        let event = event_rx.recv().await?;

        match event {
            AxonEvent::AiRequest { request_id, prompt, .. } => {
                // Call Ollama
                let response = ai::ollama::infer(&prompt).await?;
                
                // Emit response
                event_tx.send(AxonEvent::AiResponse {
                    request_id,
                    output: response,
                    model: "mistral:7b".into(),
                    tokens_used: None,
                })?;
            }

            AxonEvent::BuildRequested { project, command } => {
                // Spawn build worker
                tokio::spawn({
                    let tx = event_tx.clone();
                    let proj = project.clone();
                    async move {
                        // Execute cargo build
                        let start = std::time::Instant::now();
                        let output = tokio::process::Command::new("cargo")
                            .arg("build")
                            .arg("--release")
                            .output()
                            .await
                            .unwrap();
                        
                        let duration_ms = start.elapsed().as_millis() as u64;
                        
                        // Emit result
                        let _ = tx.send(AxonEvent::BuildFinished {
                            project: proj,
                            success: output.status.success(),
                            output: String::from_utf8_lossy(&output.stdout).to_string(),
                            duration_ms,
                        });
                    }
                });
            }

            AxonEvent::RagSearch { query, request_id } => {
                // Query your RAG system
                let results = rag::search(&query).await?;
                
                event_tx.send(AxonEvent::RagSearchResult {
                    request_id,
                    results,
                })?;
            }

            _ => {}
        }
    }
}
```

---

## 🚀 Step 9: Add Missing Types to AppState

Your `AppState` needs these fields for the initial snapshot:

```rust
use parking_lot::RwLock;
use std::collections::HashMap;

pub struct AppState {
    pub session: Arc<RwLock<Session>>,
    pub workers: Arc<RwLock<HashMap<String, WorkerInfo>>>,
    pub active_alerts: Arc<RwLock<Vec<Alert>>>,
    pub rag_indexed: Arc<RwLock<usize>>,
    pub last_build: Arc<RwLock<Option<BuildInfo>>>,
    pub safe_mode: bool,
    pub dry_run: bool,
    config: Config,
}

pub struct WorkerInfo {
    pub health: WorkerHealth,
    pub task: String,
}

pub struct Alert {
    pub id: uuid::Uuid,
    pub level: AlertLevel,
    pub title: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub fix_applied: bool,
}

pub struct BuildInfo {
    pub project: String,
    pub success: bool,
    pub duration_ms: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
```

---

## 🚀 Step 10: Test the Connection

1. **Start the engine:**
   ```bash
   cd your-axon-project
   cargo run
   ```

   You should see:
   ```
   ╔══════════════════════════════════════╗
   ║     AXON v0.1.0 — DevOS Agent        ║
   ║     Local-first · Rust · Proactive   ║
   ╚══════════════════════════════════════╝
   Config loaded from: config.toml
   Session ID: ax_4f2c9b
   🌐 WebSocket UI bridge listening on ws://127.0.0.1:7878
   All workers started successfully
   AXON fully operational. Listening...
   ```

2. **Open your dashboard:**
   ```bash
   # Serve your HTML
   cd dashboard
   python3 -m http.server 8000
   ```

   Open: `http://localhost:8000`

3. **Check browser console:**
   You should see:
   ```
   🔌 Connecting to AXON engine... ws://127.0.0.1:7878/ws
   ✅ Connected to AXON engine
   📦 Received initial state: {...}
   ```

4. **Test interactions:**
   - Type in chat → should trigger real AI inference
   - Click "REBUILD" → should execute real cargo build
   - System metrics should update every 2 seconds

---

## 🔍 Troubleshooting

### "Connection refused"
- Make sure AXON is running (`cargo run`)
- Check that WebSocket is enabled in config
- Verify port 7878 is not blocked

### "Events not appearing in UI"
- Check browser console for errors
- Verify `ws_bridge.rs` is converting `AxonEvent` → `WsEvent`
- Add debug logging: `tracing::debug!("Sending WsEvent: {:?}", event);`

### "Chat not working"
- Ensure Ollama is running: `curl http://localhost:11434/api/tags`
- Check orchestrator handles `AiRequest` event
- Verify AI module is hooked up

### "Build button does nothing"
- Check orchestrator handles `BuildRequested` event
- Verify `cargo` is in PATH
- Look for errors in engine logs

---

## 🎯 What You Get

After integration:

✅ **Real-time dashboard** — all data comes from Rust engine
✅ **Live log streaming** — see logs as they happen
✅ **Real builds** — trigger actual cargo builds
✅ **Local AI chat** — Ollama integration
✅ **RAG search** — query your indexed codebase
✅ **Worker status** — see what each worker is doing
✅ **System metrics** — CPU, RAM updating live
✅ **No more simulations** — everything is real

---

## 📚 Next Steps

1. **Add more event types** as you build features
2. **Implement fix approval workflow** via Telegram
3. **Add file watcher** using `notify` crate
4. **Implement RAG indexing** with embeddings
5. **Build Sentinel auto-fix loop**

---

## 🧠 Architecture Recap

```
Browser (HTML/JS)
    ↓ WebSocket
ws_bridge.rs (axum server)
    ↓ broadcast channel
Event Bus
    ↓ subscribe
Orchestrator + Workers
    ↓ emit events
Event Bus
    ↓ broadcast
ws_bridge → JSON → Browser
```

**Key insight:** Everything flows through events. The UI is just another consumer/producer of events.

---

## 💡 Pro Tips

1. **Use Chrome DevTools** → Network → WS tab to see live messages
2. **Add event logging** in orchestrator to debug flow
3. **Start simple** — get one feature working (e.g., chat) before adding more
4. **Use `tracing::debug!`** liberally during development
5. **Test reconnection** by restarting engine while UI is open

---

**You're ready to connect your UI to the real engine. Let's make AXON come alive! 🚀**
