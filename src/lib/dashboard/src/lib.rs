//! # Dashboard WebSocket API
//!
//! This module provides a WebSocket-based API for the FerrumC dashboard.
//!
//! ## WebSocket Protocol
//!
//! Connect to `/ws` to receive server events. The protocol uses JSON messages
//! with a `type` field indicating the event type.
//!
//! ### Handshake (sent once on connect)
//!
//! ```json
//! {
//!   "type": "Handshake",
//!   "data": {
//!     "system": { "cpu_model": "...", "cpu_cores": 12, "cpu_threads": 24 },
//!     "config": { "max_players": 100 }
//!   }
//! }
//! ```
//!
//! ### Metrics (sent every second)
//!
//! ```json
//! {
//!   "type": "Metric",
//!   "data": {
//!     "cpu_usage": 15.5,
//!     "ram_usage": 1073741824,
//!     "total_ram": 17179869184,
//!     "uptime": 3600,
//!     "storage_used": 536870912,
//!     "player_count": 42
//!   }
//! }
//! ```

use crate::telemetry::DashboardEvent;
use axum::extract::Path;
use axum::http::{header, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use ferrumc_config::server_config::get_global_config;
use ferrumc_state::GlobalState;
use include_dir::{include_dir, Dir};
use tokio::sync::broadcast;
use tracing::{debug, info};

mod handshake;
mod telemetry;
mod ws;

pub fn start_dashboard(state: GlobalState) {
    std::thread::Builder::new()
        .name("ferrumc-dashboard".to_string())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .worker_threads(2)
                .build()
                .expect("Failed to build dashboard runtime");

            rt.block_on(start_webserver(state));
        })
        .expect("Failed to spawn dashboard thread");
}

/// The HTTP protocol used by the dashboard.
/// (i made this variable cuz rustrover was complaining about insecure url 💔😔)
const PROTOCOL: &str = "http";

/// This macro runs at COMPILE TIME.
/// It embeds the entire dashboard directory into the binary.
/// Uses OUT_DIR if available, otherwise falls back to CARGO_MANIFEST_DIR.
#[cfg(dashboard_in_out_dir)]
static DASHBOARD_DIR: Dir<'_> = include_dir!("$OUT_DIR/dashboard");

#[cfg(dashboard_in_manifest_dir)]
static DASHBOARD_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/dashboard-dist");

// Fallback if neither cfg is set (shouldn't happen, but prevents compile errors)
#[cfg(not(any(dashboard_in_out_dir, dashboard_in_manifest_dir)))]
static DASHBOARD_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/dashboard-dist");

async fn start_webserver(state: GlobalState) {
    debug!("Starting FerrumC dashboard webserver...");

    // Gather handshake data once at startup
    let handshake = handshake::Handshake::gather();
    debug!("Handshake data gathered: {:?}", handshake);

    // Create a rx/tx (with max 100 messages buffered) for telemetry events
    let (tx, _rx) = broadcast::channel::<DashboardEvent>(100);

    // Spawn the Telemetry Loop in the background with GlobalState for player count
    let tx_clone = tx.clone();
    let state_clone = state.clone();
    tokio::spawn(telemetry::start_telemetry_loop(tx_clone, state_clone));

    // WebSocket state includes broadcast channel and handshake data
    let ws_state = ws::WsState { tx, handshake };

    // axum app/router
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/ws", get(ws::ws_handler))
        .route("/{*path}", get(static_handler))
        .with_state(ws_state);

    let config = get_global_config();
    let addr = format!("{}:{}", config.host, config.dashboard.port);

    match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            info!("Dashboard listening on {PROTOCOL}://{}", addr);
            if let Err(e) = axum::serve(listener, app).await {
                tracing::error!("Dashboard server error: {}", e);
            }
        }
        Err(e) => {
            tracing::error!("Failed to bind dashboard to {}: {}", addr, e);
        }
    }
}

async fn index_handler() -> impl IntoResponse {
    match DASHBOARD_DIR.get_file("index.html") {
        Some(file) => Html(file.contents()).into_response(),
        None => (StatusCode::NOT_FOUND, "index.html not found").into_response(),
    }
}

async fn static_handler(Path(path): Path<String>) -> Response {
    // Try to find the file in the embedded directory
    let path = path.trim_start_matches('/');

    // Try multiple fallbacks for the path
    let file = DASHBOARD_DIR
        .get_file(path)
        // Try with index.html appended (for directory-style URLs like /admin/)
        .or_else(|| {
            let with_index = if path.is_empty() {
                "index.html".to_string()
            } else if path.ends_with('/') {
                format!("{}index.html", path)
            } else {
                format!("{}/index.html", path)
            };
            DASHBOARD_DIR.get_file(&with_index)
        })
        // Try with .html extension (for clean URLs like /about -> /about.html)
        .or_else(|| {
            if !path.contains('.') {
                DASHBOARD_DIR.get_file(format!("{}.html", path))
            } else {
                None
            }
        });

    match file {
        Some(file) => {
            let file_path = file.path().to_string_lossy();
            let mime_type = mime_guess::from_path(file_path.as_ref())
                .first_or_octet_stream()
                .to_string();

            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, mime_type)],
                file.contents(),
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}
