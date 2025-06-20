mod stats;

use ferrumc_state::GlobalState;

pub async fn start_web_ui(state: GlobalState) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the web UI server
    let server = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Welcome to the Ferrumc Web UI!" }))
        .route("/stats", axum::routing::get(stats::stats_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await?;
    axum::serve(listener, server)
        .await
        .map_err(|e| format!("Failed to start web UI server: {e}"))?;

    Ok(())
}