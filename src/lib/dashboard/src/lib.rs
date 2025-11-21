use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use ferrumc_config::server_config::get_global_config;
use ferrumc_state::GlobalState;
use tracing::{debug, info};

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
async fn start_webserver(state: GlobalState) {
    debug!("Starting FerrumC dashboard webserver...");

    let app = Router::new()
        .route("/", get(dashboard_handler))
        .with_state(state);

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

// This macro runs at COMPILE TIME.
// It looks inside the OUT_DIR (where build.rs saved the file)
// and embeds the content directly into the binary's data segment.
const DASHBOARD_HTML: &str = include_str!(concat!(env!("OUT_DIR"), "/dashboard.min.html"));

async fn dashboard_handler(State(_state): State<GlobalState>) -> impl IntoResponse {
    Html(DASHBOARD_HTML)
}
