use axum::routing::get;
use axum::Router;
use ferrumc_state::GlobalState;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};
use tracing::info;

mod routes;

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

async fn start_webserver(_state: GlobalState) {
    info!("Starting FerrumC Dashboard...");

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/overview", get(routes::overview))
        .route("/console", get(routes::console))
        .route("/players", get(routes::players))
        // Serve the specific icon from the global assets folder
        .route_service("/assets/icon.png", ServeFile::new("assets/data/icon.png"))
        // Serve other dashboard assets (css, js, etc.)
        .nest_service("/assets", ServeDir::new("src/lib/dashboard/assets"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("Dashboard listening on http://{}", addr);
            if let Err(e) = axum::serve(listener, app).await {
                tracing::error!("Dashboard server error: {}", e);
            }
        }
        Err(e) => {
            tracing::error!("Failed to bind dashboard to {}: {}", addr, e);
        }
    }
}