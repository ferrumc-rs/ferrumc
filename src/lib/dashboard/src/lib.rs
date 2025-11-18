use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::info;

pub fn start_dashboard() {
    std::thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to build Tokio runtime for dashboard");

        rt.block_on(async {
            info!("Starting FerrumC Dashboard...");

            let app = Router::new().route("/", get(|| async { "Hello, World!" }));

            let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            info!("Dashboard listening on {}", addr);
            axum::serve(listener, app).await.unwrap();
        });
    });
}