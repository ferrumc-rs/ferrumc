use crate::telemetry::DashboardEvent;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use tokio::sync::broadcast;

// The state needed for the websocket (just the broadcast channel)
#[derive(Clone)]
pub struct WsState {
    pub tx: broadcast::Sender<DashboardEvent>,
}

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<WsState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: WsState) {
    // Subscribe to the telemetry channel
    let mut rx = state.tx.subscribe();

    // Simple loop: Receive from channel -> Send to Websocket
    while let Ok(event) = rx.recv().await {
        if let Ok(json) = serde_json::to_string(&event) {
            if socket
                .send(Message::Text(Utf8Bytes::from(json)))
                .await
                .is_err()
            {
                // Client disconnected
                break;
            }
        }
    }
}
