# AXON UI ↔ Engine Data Flow

## Complete Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         BROWSER (UI)                             │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │              AXON Dashboard (HTML/JS)                    │   │
│  │                                                          │   │
│  │  - Chat Interface      - Log Panel                     │   │
│  │  - Build Controls      - Worker Status                 │   │
│  │  - Alert Cards         - System Metrics                │   │
│  │  - RAG Search          - Telegram Feed                 │   │
│  └──────────────────────────────────────────────────────────┘   │
│                            ▲  │                                  │
│                            │  │                                  │
│                       WebSocket (ws://127.0.0.1:7878/ws)        │
│                            │  │                                  │
│                      JSON  │  │  JSON                            │
│                    Events  │  │  Commands                        │
└────────────────────────────┼──┼──────────────────────────────────┘
                             │  ▼
┌─────────────────────────────────────────────────────────────────┐
│                      RUST ENGINE (AXON)                          │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    ws_bridge.rs                          │   │
│  │  ┌────────────────────────────────────────────────────┐  │   │
│  │  │  WebSocket Server (Axum)                          │  │   │
│  │  │  - Accepts UI connections                         │  │   │
│  │  │  - Sends: WsEvent → JSON                          │  │   │
│  │  │  - Receives: UiCommand ← JSON                     │  │   │
│  │  └────────────────────────────────────────────────────┘  │   │
│  │                        ▲  │                              │   │
│  │                        │  │                              │   │
│  │              WsEvent   │  │  AxonEvent                   │   │
│  │              (convert) │  │  (emit)                      │   │
│  └────────────────────────┼──┼──────────────────────────────┘   │
│                           │  ▼                                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │              Broadcast Channel (Event Bus)               │   │
│  │                                                          │   │
│  │  All components publish and subscribe to AxonEvent      │   │
│  │  - Supports multiple readers (fan-out)                  │   │
│  │  - Non-blocking                                         │   │
│  └──────────────────────────────────────────────────────────┘   │
│         │               │               │               │        │
│         ▼               ▼               ▼               ▼        │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │Orches-   │   │ Build    │   │  RAG     │   │ Log      │     │
│  │trator    │   │ Worker   │   │ Indexer  │   │ Watcher  │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
│         │               │               │               │        │
│         ▼               ▼               ▼               ▼        │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │   AI     │   │ Telegram │   │ Sentinel │   │  Shell   │     │
│  │ Handler  │   │ Handler  │   │  Agent   │   │ Executor │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
│         │               │               │               │        │
│         └───────────────┴───────────────┴───────────────┘        │
│                         │                                        │
│                         ▼                                        │
│                  ┌──────────────┐                                │
│                  │  AppState    │                                │
│                  │  (Shared)    │                                │
│                  └──────────────┘                                │
└─────────────────────────────────────────────────────────────────┘
                         │
                         ▼
               ┌──────────────────┐
               │  External APIs   │
               │  - Ollama        │
               │  - Telegram Bot  │
               │  - File System   │
               └──────────────────┘
```

---

## Event Flow Examples

### 1. User Sends Chat Message

```
UI:
  User types: "cargo build failed, help"
  sendMessage() → axon.sendChat(msg)
    │
    ▼
  WebSocket.send({
    type: "Chat",
    payload: { message: "..." }
  })

───────────────────────────────────────

Engine:
  ws_bridge receives JSON
    │
    ▼
  Parse UiCommand::Chat
    │
    ▼
  event_tx.send(AxonEvent::AiRequest {
    request_id: uuid,
    prompt: "...",
    ...
  })
    │
    ▼
  Broadcast Channel
    │
    ▼
  Orchestrator receives AiRequest
    │
    ▼
  Spawn task: ai::ollama::infer(prompt)
    │
    ▼
  event_tx.send(AxonEvent::AiResponse {
    output: "Missing hnsw_rs...",
    ...
  })
    │
    ▼
  Broadcast Channel
    │
    ▼
  ws_bridge receives AiResponse
    │
    ▼
  Convert to WsEvent::ChatResponse
    │
    ▼
  WebSocket.send(JSON)

───────────────────────────────────────

UI:
  axon.handleEngineEvent(data)
    │
    ▼
  if (data.type === "ChatResponse")
    │
    ▼
  addChatMessage("ai", response.text)
    │
    ▼
  User sees AI response in chat
```

---

### 2. System Metrics Update

```
Engine:
  Background task (every 2s)
    │
    ▼
  sysinfo::System::refresh_all()
    │
    ▼
  event_tx.send(AxonEvent::SystemMetrics {
    cpu: 45.2,
    ram_gb: 11.4,
    ...
  })
    │
    ▼
  Broadcast Channel
    │
    ▼
  ws_bridge receives SystemMetrics
    │
    ▼
  Convert to WsEvent::SystemMetrics
    │
    ▼
  WebSocket.send(JSON)

───────────────────────────────────────

UI:
  axon.handleSystemMetrics(metrics)
    │
    ▼
  Update CPU bar: cpuBarEl.style.width = "45%"
  Update RAM text: ramEl.textContent = "11.4G"
    │
    ▼
  User sees live metrics
```

---

### 3. Build Request → Execution → Result

```
UI:
  User clicks "REBUILD" button
    │
    ▼
  simulateBuild() → axon.rebuild()
    │
    ▼
  WebSocket.send({
    type: "Rebuild",
    payload: { project: null }
  })

───────────────────────────────────────

Engine:
  ws_bridge receives Rebuild command
    │
    ▼
  event_tx.send(AxonEvent::BuildRequested {
    project: "axon_core",
    command: "cargo build --release"
  })
    │
    ▼
  Orchestrator receives BuildRequested
    │
    ▼
  Spawn async task:
    tokio::process::Command::new("cargo")
      .arg("build")
      .arg("--release")
      .output()
      .await
    │
    ▼
  event_tx.send(AxonEvent::BuildFinished {
    success: true,
    duration_ms: 2140
  })
    │
    ▼
  ws_bridge converts to WsEvent::BuildFinished
    │
    ▼
  WebSocket.send(JSON)

───────────────────────────────────────

UI:
  axon.handleBuildFinished(build)
    │
    ▼
  Update status pill: "BUILD OK"
  Update stat card: success badge
    │
    ▼
  User sees build completed
```

---

## Key Design Principles

### 1. Single Source of Truth
- **Event Bus** is the only communication channel
- No direct function calls between components
- All state changes flow through events

### 2. Decoupled Components
- UI doesn't know about engine internals
- Workers don't know about each other
- WebSocket bridge is a pure translator

### 3. Fan-Out Broadcasting
- One event → multiple subscribers
- UI, logging, workers all get the same events
- Non-blocking, async

### 4. Type-Safe Boundaries
```rust
AxonEvent (internal)  →  WsEvent (external)
         ↓ convert              ↓ serialize
   typed Rust enum           JSON string
```

### 5. Resilient Connection
- UI auto-reconnects on disconnect
- Messages queued during downtime
- No data loss

---

## Message Format

### UI → Engine (UiCommand)
```json
{
  "type": "Chat",
  "payload": {
    "message": "help me debug this"
  }
}
```

### Engine → UI (WsEvent)
```json
{
  "type": "ChatResponse",
  "payload": {
    "text": "Here's the issue...",
    "model": "mistral:7b"
  }
}
```

---

## State Synchronization

### Initial Connection
1. UI connects via WebSocket
2. Engine sends `InitialState` with complete snapshot:
   - Session info
   - All workers
   - Recent alerts
   - RAG stats
   - Config flags
3. UI reconstructs entire state

### Incremental Updates
- Each change emits a specific event
- UI applies delta updates
- No polling needed

### Reconnection
- On reconnect, engine resends `InitialState`
- UI resets and rebuilds from snapshot
- Seamless for user

---

## Scalability Notes

### Current: Single Instance
- One engine process
- Multiple UI clients supported
- Shared state via Arc<RwLock<T>>

### Future: Distributed
- Event bus could be Redis pub/sub
- Multiple engine instances
- Load balancing via proxy

---

**This architecture is production-ready for local-first operation while being extensible to distributed systems.**
