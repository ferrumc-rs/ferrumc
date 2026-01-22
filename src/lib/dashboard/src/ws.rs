use crate::handshake::Handshake;
use crate::telemetry::DashboardEvent;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use tokio::sync::broadcast;

/// The state needed for the websocket
#[derive(Clone)]
pub struct WsState {
    pub tx: broadcast::Sender<DashboardEvent>,
    pub handshake: Handshake,
}

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<WsState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: WsState) {
    // Send handshake immediately on connection
    let handshake = DashboardEvent::Handshake(state.handshake.clone());
    if let Ok(json) = serde_json::to_string(&handshake) {
        if socket
            .send(Message::Text(Utf8Bytes::from(json)))
            .await
            .is_err()
        {
            // Client disconnected during handshake
            return;
        }
    }

    // Subscribe to the telemetry channel for ongoing metrics
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
