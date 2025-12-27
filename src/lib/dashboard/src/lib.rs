use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use tokio::sync::broadcast;
use ferrumc_config::server_config::get_global_config;
use ferrumc_state::GlobalState;
use tracing::{debug, info};
use crate::telemetry::DashboardEvent;

/// Dashboard telemetry module
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
/// It looks inside the OUT_DIR (where build.rs saved the file)
/// and embeds the content directly into the binary's data segment.
const DASHBOARD_HTML: &str = include_str!(concat!(env!("OUT_DIR"), "/dashboard.min.html"));

async fn start_webserver(_state: GlobalState) {
    debug!("Starting FerrumC dashboard webserver...");

    // Create a rx/tx (with max 100 messages buffered) for telemetry events
    let (tx, _rx) = broadcast::channel::<DashboardEvent>(100);

    // Spawn the Telemetry Loop in the background
    let tx_clone = tx.clone();
    tokio::spawn(telemetry::start_telemetry_loop(tx_clone));

    // websocket state:
    let ws_state = ws::WsState { tx };

    // axum app/router
    let app = Router::new()
        .route("/", get(dashboard_handler))
        .route("/ws", get(ws::ws_handler))
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



async fn dashboard_handler() -> impl IntoResponse {
    Html(DASHBOARD_HTML)
}
