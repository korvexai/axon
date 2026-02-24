use axum::{
    Router,
    routing::get,
    extract::{
        ws::{WebSocketUpgrade, WebSocket, Message},
        State,
    },
    response::IntoResponse,
};

use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use serde::Serialize;
use anyhow::Result;
use tracing::{info, debug};
use uuid::Uuid;

use crate::core::state::AppState;
use crate::event::event::AxonEvent;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsEvent {
    InitialState { rag_indexed: usize },
    ChatResponse { text: String, model: String },
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum UiCommand {
    Chat { message: String },
}

pub struct WsBridgeState {
    pub app_state: Arc<AppState>,
    pub event_tx: broadcast::Sender<AxonEvent>,
}

pub async fn run(
    state: Arc<AppState>,
    event_tx: broadcast::Sender<AxonEvent>,
) -> Result<()> {

    let bind_addr = "127.0.0.1:7878";

    let bridge_state = Arc::new(WsBridgeState {
        app_state: state,
        event_tx: event_tx.clone(),
    });

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(bridge_state);

    info!("WebSocket listening on ws://{}", bind_addr);

    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WsBridgeState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(
    socket: WebSocket,
    state: Arc<WsBridgeState>,
) {
    let (mut sender, mut receiver) = socket.split();
    let mut event_rx = state.event_tx.subscribe();

    let rag_indexed = *state.app_state.rag_indexed.read().await as usize;
    let init = WsEvent::InitialState { rag_indexed };

    if let Ok(json) = serde_json::to_string(&init) {
        let _ = sender.send(Message::Text(json)).await;
    }

    loop {
        tokio::select! {

            Ok(event) = event_rx.recv() => {
                if let AxonEvent::AiResponse { output, model, .. } = event {
                    let msg = WsEvent::ChatResponse { text: output, model };
                    if let Ok(json) = serde_json::to_string(&msg) {
                        if sender.send(Message::Text(json)).await.is_err() {
                            break;
                        }
                    }
                }
            }

            Some(Ok(msg)) = receiver.next() => {
                if let Message::Text(text) = msg {
                    if let Ok(cmd) = serde_json::from_str::<UiCommand>(&text) {
                        if let UiCommand::Chat { message } = cmd {
                            let _ = state.event_tx.send(AxonEvent::AiRequest {
                                id: Uuid::new_v4(),
                                prompt: message,
                                model: None,
                                context: None,
                            });
                        }
                    }
                }
            }

            else => break,
        }
    }

    debug!("UI disconnected");
}
