//! FerrumC stress-test bot.
//!
//! Spawns a configurable swarm of offline-mode [`azalea`] bots that connect to a FerrumC server and
//! optionally wander, reproducing the chunk-loading, movement, and broadcast load of many
//! simultaneous players. It exists to measure server tick performance under real concurrency rather
//! than synthetic microbenchmarks.
//!
//! Pinned to azalea 0.14 (Minecraft 1.21.8 / protocol 772) to match FerrumC's protocol version;
//! newer azalea releases follow the latest Minecraft version and will not connect without a
//! translating proxy.
//!
//! ```text
//! cargo run --release -- --server 127.0.0.1:25565 --bots 100 --join-delay-ms 150
//! ```

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use std::time::Duration;

use azalea::bot::BotClientExt;
use azalea::prelude::*;
use azalea::swarm::prelude::*;
use azalea::WalkDirection;
use clap::Parser;
use rand::Rng;

/// Command-line configuration for a stress run.
#[derive(Parser, Debug, Clone)]
#[command(
    name = "ferrumc-stress-bot",
    about = "Swarm load tester for FerrumC (azalea, Minecraft 1.21.8)"
)]
struct Args {
    /// Server address to connect to (`host:port`).
    #[arg(long, default_value = "127.0.0.1:25565")]
    server: String,

    /// Number of bots to spawn.
    #[arg(long, default_value_t = 50)]
    bots: usize,

    /// Delay between successive bot joins, in milliseconds. Staggering the joins avoids a single
    /// connection storm and better mimics players trickling in.
    #[arg(long, default_value_t = 200)]
    join_delay_ms: u64,

    /// Username prefix; bots are named `<prefix>_<n>`.
    #[arg(long, default_value = "stress")]
    name_prefix: String,

    /// Connect and idle without moving — measures pure connection/keepalive/chunk-tracking load.
    #[arg(long)]
    idle: bool,

    /// Seconds between metrics reports.
    #[arg(long, default_value_t = 5)]
    stats_interval_secs: u64,

    /// Seconds to wait before reconnecting a bot that was disconnected. Keeps the swarm size
    /// roughly constant under churn.
    #[arg(long, default_value_t = 5)]
    reconnect_delay_secs: u64,
}

/// Process-wide configuration, set once at startup and read by every bot handler. Storing it
/// globally keeps the per-bot ECS state a zero-sized type instead of threading config through the
/// handler signature.
static CONFIG: OnceLock<Args> = OnceLock::new();

// Aggregate metrics across all bots. Plain atomics avoid plumbing a shared resource through the
// swarm and are sufficient for the coarse connection/movement counters reported here.
static CONNECTED: AtomicUsize = AtomicUsize::new(0);
static PEAK_CONNECTED: AtomicUsize = AtomicUsize::new(0);
static TOTAL_JOINS: AtomicUsize = AtomicUsize::new(0);
static DISCONNECTS: AtomicUsize = AtomicUsize::new(0);

/// Per-bot state. Deliberately empty: all configuration is global and all metrics are atomics, so
/// a bot carries no individual state.
#[derive(Component, Default, Clone)]
struct BotState;

/// Swarm-level state. Unused, but the swarm handler signature requires a resource type.
#[derive(Resource, Default, Clone)]
struct SwarmState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                // Quiet by default: azalea is chatty at info. Override with RUST_LOG.
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn,ferrumc_stress_bot=info")),
        )
        .init();

    let args = Args::parse();
    println!(
        "[stress] starting {} bot(s) -> {} (join delay {}ms, {})",
        args.bots,
        args.server,
        args.join_delay_ms,
        if args.idle { "idle" } else { "wandering" }
    );
    let stats_interval = Duration::from_secs(args.stats_interval_secs.max(1));
    let join_delay = Duration::from_millis(args.join_delay_ms);
    CONFIG
        .set(args.clone())
        .expect("CONFIG set exactly once at startup");

    // Periodic metrics reporter.
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(stats_interval).await;
            println!(
                "[stress] connected={} peak={} joins={} disconnects={}",
                CONNECTED.load(Ordering::Relaxed),
                PEAK_CONNECTED.load(Ordering::Relaxed),
                TOTAL_JOINS.load(Ordering::Relaxed),
                DISCONNECTS.load(Ordering::Relaxed),
            );
        }
    });

    let mut builder = SwarmBuilder::new()
        .set_handler(handle)
        .set_swarm_handler(swarm_handle);
    for i in 0..args.bots {
        builder = builder.add_account(Account::offline(&format!("{}_{}", args.name_prefix, i)));
    }

    builder
        .join_delay(join_delay)
        .start(args.server.as_str())
        .await?;
    Ok(())
}

/// Per-bot event handler: counts connections and, unless `--idle`, drives continuous movement so
/// the server processes position updates, chunk tracking, and entity broadcasts for every bot.
async fn handle(bot: Client, event: Event, _state: BotState) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            TOTAL_JOINS.fetch_add(1, Ordering::Relaxed);
            let now = CONNECTED.fetch_add(1, Ordering::Relaxed) + 1;
            PEAK_CONNECTED.fetch_max(now, Ordering::Relaxed);
        }
        Event::Tick => {
            let cfg = CONFIG.get().expect("CONFIG initialised before any tick");
            if cfg.idle {
                return Ok(());
            }
            // Wander: keep walking forward, occasionally pick a new heading, occasionally jump.
            // Driven off the bot's own tick so movement scales with the simulated tick rate.
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.05) {
                let yaw: f32 = rng.gen_range(-180.0..180.0);
                bot.set_direction(yaw, 0.0);
            }
            bot.walk(WalkDirection::Forward);
            if rng.gen_bool(0.02) {
                bot.jump();
            }
        }
        _ => {}
    }
    Ok(())
}

/// Swarm-level handler: reconnects disconnected bots after a delay so the swarm size stays roughly
/// constant for the duration of a run.
async fn swarm_handle(
    swarm: Swarm,
    event: SwarmEvent,
    _state: SwarmState,
) -> anyhow::Result<()> {
    if let SwarmEvent::Disconnect(account, join_opts) = event {
        CONNECTED
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |c| Some(c.saturating_sub(1)))
            .ok();
        DISCONNECTS.fetch_add(1, Ordering::Relaxed);
        let delay = CONFIG
            .get()
            .map(|c| c.reconnect_delay_secs)
            .unwrap_or(5);
        tokio::time::sleep(Duration::from_secs(delay)).await;
        swarm.add_with_opts(&account, BotState, &join_opts).await?;
    }
    Ok(())
}
