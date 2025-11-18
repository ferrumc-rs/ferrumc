use tokio_stream::StreamExt;
use axum::extract::State;
use axum::response::sse::{Event, KeepAlive};
use axum::response::{Html, Sse};
use axum::routing::get;
use axum::Router;
use ferrumc_state::GlobalState;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};
use tokio_stream::Stream;
use tracing::{debug, info};

pub fn start_dashboard(
    state: GlobalState
) {
    // start it on a separate thread
    std::thread::Builder::new()
        .name("ferrumc-dashboard".to_string())
        .spawn(|| {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build dashboard runtime");

            rt.block_on(start_webserver(state));
        })
        .expect("Failed to spawn dashboard thread");
}

async fn start_webserver(
    state: GlobalState
) {
    info!("Starting FerrumC Dashboard...");

    let app = Router::new()
        .route("/", get(index))
        .route("/stats", get(stats_stream))
        .with_state(state);

    let protocol = "http";
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("Dashboard listening on {}://{}", protocol, addr);
            if let Err(e) = axum::serve(listener, app).await {
                tracing::error!("Dashboard server error: {}", e);
            }
        }
        Err(e) => {
            tracing::error!("Failed to bind dashboard to {}: {}", addr, e);
        }
    }
}

async fn index() -> Html<&'static str> {
    Html(
        r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>FerrumC Admin</title>
        <script src="https://unpkg.com/htmx.org@1.9.10"></script>
        <script src="https://unpkg.com/htmx.org/dist/ext/sse.js"></script>
        <style>
            body { background: #111; color: #eee; font-family: monospace; display: grid; place-items: center; height: 100vh; margin: 0; }
            .panel { background: #222; padding: 2rem; border: 1px solid #444; border-radius: 8px; width: 300px; }
            .row { display: flex; justify-content: space-between; margin-bottom: 10px; }
            .val { color: #0f0; font-weight: bold; }
        </style>
    </head>
    <body>
        <div class="panel">
            <h2 style="margin-top:0; border-bottom:1px solid #444; padding-bottom:10px;">FerrumC Dashboard</h2>
            <div hx-ext="sse" sse-connect="/stats" sse-swap="message">
                Loading stats...
            </div>
        </div>
    </body>
    </html>
    "#,
    )
}

async fn stats_stream(
    State(state): State<GlobalState>,
) -> Sse<impl Stream<Item=Result<Event, Infallible>>> {
    const UPDATE_INTERVAL_SECS: u64 = 1;
    let stream = tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(
        Duration::from_secs(UPDATE_INTERVAL_SECS),
    ))
        .map(move |_| {
            // 1. Get Player Count
            let player_count = state.players.player_list.len();
            debug!("Pushing dashboard update: {} players", player_count);

            // 2. Get Process-Specific Memory
            // We limit the refresh to just Memory and just THIS process ID.
            // This is much faster than refreshing the whole OS snapshot.
            let pid = Pid::from_u32(std::process::id());
            let sys = System::new_with_specifics(
                RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing().with_memory())
            );

            sys.process(pid);

            let used_ram_mb = match sys.process(pid) {
                Some(process) => process.memory() / 1024 / 1024, // Convert bytes to MB
                None => 0,
            };

            let html = format!(r#"
                <div class="row"><span>Online Players:</span> <span class="val">{}</span></div>
                <div class="row"><span>RAM Usage:</span> <span class="val">{} MB</span></div>
                <div class="row"><span>Status:</span> <span class="val" style="color:lime">Active</span></div>
            "#, player_count, used_ram_mb);

            Ok(Event::default().data(html))
        });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
