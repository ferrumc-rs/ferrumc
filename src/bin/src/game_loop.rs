//! Main game loop module.
//!
//! This module contains the core server loop that:
//! - Initializes the ECS (Entity Component System) world
//! - Sets up networking (TCP connection acceptor)
//! - Runs timed schedules (tick, world sync, keepalive, etc.)
//! - Handles graceful shutdown

use crate::errors::BinaryError;
use crate::packet_handlers::{play_packets, register_player_systems};
use crate::register_messages::register_messages;
use crate::register_resources::register_resources;
use crate::systems::emit_player_joined::emit_player_joined;
use crate::systems::lan_pinger::LanPinger;
use crate::systems::listeners::register_gameplay_listeners;
use crate::systems::mobs::register_mob_systems;
use crate::systems::new_connections::accept_new_connections;
use crate::systems::physics::register_physics;
use crate::systems::register_game_systems;
use crate::systems::shutdown_systems::register_shutdown_systems;
use bevy_ecs::prelude::World;
use bevy_ecs::schedule::{ApplyDeferred, ExecutorKind, IntoScheduleConfigs, Schedule};
use crossbeam_channel::Sender;
use ferrumc_commands::infrastructure::register_command_systems;
use ferrumc_config::server_config::get_global_config;
use ferrumc_net::connection::{handle_connection, NewConnection};
use ferrumc_net::server::create_server_listener;
use ferrumc_net::PacketSender;
use ferrumc_performance::tick::TickData;
use ferrumc_performance::ServerPerformance;
use ferrumc_scheduler::MissedTickBehavior;
use ferrumc_scheduler::{drain_registered_schedules, Scheduler, TimedSchedule};
use ferrumc_state::{GlobalState, GlobalStateResource};
use ferrumc_utils::formatting::format_duration;
use play_packets::register_packet_handlers;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, error, info, info_span, trace, warn, Instrument};

/// Main entry point for the server game loop.
///
/// This function:
/// 1. Initializes the Bevy ECS world and registers all systems/resources
/// 2. Starts the TCP connection acceptor on a separate thread
/// 3. Runs the main scheduler loop that executes timed schedules (tick, sync, etc.)
/// 4. Handles graceful shutdown when the server is stopped
pub fn start_game_loop(global_state: GlobalState) -> Result<(), BinaryError> {
    // =========================================================================
    // PHASE 1: ECS World Setup
    // =========================================================================

    // Create the Bevy ECS world - this holds all entities, components, and resources
    let mut ecs_world = World::new();

    // Schedule that runs cleanup systems when the server shuts down
    let mut shutdown_schedule = Schedule::default();

    // =========================================================================
    // PHASE 2: Channel Setup for Inter-Thread Communication
    // =========================================================================

    // Packet sender for outgoing network packets (shared across connection handlers)
    let sender_struct = Arc::new(ferrumc_net::create_packet_senders(&mut ecs_world));

    // Channel for new player connections (TCP acceptor -> main loop)
    let (new_conn_send, new_conn_recv) = crossbeam_channel::unbounded();

    // Shutdown coordination channels:
    // - shutdown_send/recv: Main loop tells TCP acceptor to stop
    // - shutdown_response: TCP acceptor confirms it has stopped
    let (shutdown_send, shutdown_recv) = tokio::sync::oneshot::channel();
    let (shutdown_response_send, shutdown_response_recv) = crossbeam_channel::unbounded();

    // =========================================================================
    // PHASE 3: Register ECS Systems and Resources
    // =========================================================================

    // Initialize default server commands (e.g., /stop, /help, etc.)
    ferrumc_default_commands::init();

    // Wrap global state for ECS resource access
    let global_state_res = GlobalStateResource(global_state.clone());

    // Register event messages the ECS will handle
    register_messages(&mut ecs_world);

    // Register shared resources (connection receiver, global state, etc.)
    register_resources(&mut ecs_world, new_conn_recv, global_state_res);

    // Build the timed scheduler with all periodic schedules (tick, sync, keepalive)
    let mut timed = build_timed_scheduler();

    // Register systems that run on shutdown (save world, disconnect players, etc.)
    register_shutdown_systems(&mut shutdown_schedule);

    // =========================================================================
    // PHASE 4: Start Network Thread
    // =========================================================================

    // Spawn the TCP connection acceptor on a dedicated thread with its own Tokio runtime
    tcp_conn_acceptor(
        global_state.clone(),
        sender_struct,
        Arc::new(new_conn_send),
        shutdown_recv,
        shutdown_response_send,
    )?;

    info!(
        "Server is ready in {}",
        format_duration(global_state.start_time.elapsed())
    );

    // =========================================================================
    // PHASE 5: Main Scheduler Loop
    // =========================================================================

    // Maximum number of schedules to run in a single iteration before yielding.
    // This prevents starvation if we fall behind (e.g., after a lag spike).
    const MAX_GLOBAL_CATCH_UP: usize = 64;

    let tick_zero = Instant::now();

    // Main loop - runs until shutdown flag is set (e.g., via Ctrl+C or /stop command)
    while !global_state
        .shut_down
        .load(std::sync::atomic::Ordering::Relaxed)
    {
        let tick_start = Instant::now();
        let mut ran_any = false;
        let mut ran_count = 0;

        // Inner loop: Run all schedules that are currently due
        loop {
            // Prevent running too many schedules in one go (catch-up limit)
            if ran_count >= MAX_GLOBAL_CATCH_UP {
                break;
            }

            let now = Instant::now();

            // Peek at the next schedule that's due to run
            let Some((idx, due)) = timed.peek_next_due() else {
                // No schedules registered, wait a bit
                // which is unexpected, because we should have at least the tick schedule
                warn!("No schedules registered (this is a bug)");
                std::thread::sleep(Duration::from_millis(1));
                continue;
            };

            // If the next schedule isn't due yet, exit inner loop
            if due > now {
                break;
            }

            // Pop the schedule from the priority queue
            let (popped_idx, _popped_due) = timed
                .pop_next_due()
                .expect("scheduler heap changed unexpectedly");
            debug_assert_eq!(popped_idx, idx);

            // Get schedule metadata for logging
            let name = timed.schedules[idx].name.clone();
            let period = timed.schedules[idx].period;

            // Execute the schedule and measure how long it took
            let start = Instant::now();
            timed.schedules[idx].schedule.run(&mut ecs_world);
            let elapsed = start.elapsed();

            // Log warning if schedule took longer than its allocated time budget
            if elapsed > period {
                warn!(
                    "Schedule '{}' overran: took {:?}, budget {:?}",
                    name, elapsed, period
                );
            } else {
                trace!(
                    "Schedule '{}' ran in {:?} (budget {:?})",
                    name,
                    elapsed,
                    period
                );
            }

            // Reschedule for next run (updates next_due time based on MissedTickBehavior)
            timed.after_run(idx);

            ran_any = true;
            ran_count += 1;
        }

        let tick_duration = tick_start.elapsed();

        // If no schedules were ready, sleep until the next one is due
        // If schedules were ran, store tick data.
        if !ran_any {
            timed.park_until_next_due();
        } else {
            let tick_data = TickData {
                start_ns: tick_zero.elapsed().as_nanos(),
                duration_ns: tick_duration.as_nanos(),
                entity_count: 0,
                ran_count,
            };

            let mut performance = ecs_world.resource_mut::<ServerPerformance>();
            performance.tps.record_tick(tick_data);
        }
    }

    // =========================================================================
    // PHASE 6: Graceful Shutdown
    // =========================================================================

    // Run shutdown systems (save world, disconnect players, cleanup)
    shutdown_schedule.run(&mut ecs_world);

    // Signal the TCP acceptor thread to stop accepting new connections
    trace!("Sending shutdown signal to TCP connection acceptor");
    shutdown_send
        .send(())
        .expect("Failed to send shutdown signal");

    // Wait for TCP acceptor to confirm it has shut down cleanly
    trace!("Waiting for TCP connection acceptor to shut down");
    shutdown_response_recv
        .recv()
        .expect("Failed to receive shutdown response");

    Ok(())
}

/// Builds the timed scheduler with all periodic game schedules.
///
/// Each schedule runs at a specific interval and handles different aspects of the game:
/// - **tick**: Main game tick (player updates, packets, commands) - runs at configured TPS
/// - **world_sync**: Persists world data to disk - every 15 seconds
/// - **keepalive**: Sends keepalive packets to prevent timeouts - every 1 second
fn build_timed_scheduler() -> Scheduler {
    let mut timed = Scheduler::new();

    // -------------------------------------------------------------------------
    // TICK SCHEDULE - Main game loop tick
    // -------------------------------------------------------------------------
    // This is the core game tick that runs at the configured TPS (ticks per second).
    // It processes packets, updates players, handles commands, and runs game systems.
    // Uses Burst behavior to catch up if ticks are missed (up to 5 at a time).
    let build_tick = |s: &mut Schedule| {
        s.set_executor_kind(ExecutorKind::SingleThreaded);
        register_packet_handlers(s); // Handle incoming packets from players
        register_player_systems(s); // Update player state (position, inventory, etc.)
        register_command_systems(s); // Process queued commands

        // Player connection handling - chained to ensure proper event timing:
        // 1. accept_new_connections: Spawns entity + adds PendingPlayerJoin marker (deferred)
        // 2. ApplyDeferred: Flushes commands, entity now exists and is queryable
        // 3. emit_player_joined: Fires PlayerJoined event (listeners can now query the entity)
        s.add_systems((accept_new_connections, ApplyDeferred, emit_player_joined).chain());

        register_game_systems(s); // General game logic (chunks, day cycle, etc.)
        register_gameplay_listeners(s); // Event listeners for gameplay events
        register_physics(s); // Physics systems (movement, collision, etc.)
        register_mob_systems(s); // Mob AI and behavior
    };
    let tick_period = Duration::from_secs(1) / get_global_config().tps;
    timed.register(
        TimedSchedule::new("tick", tick_period, build_tick)
            .with_behavior(MissedTickBehavior::Burst) // Run missed ticks to catch up
            .with_max_catch_up(5), // But only catch up 5 ticks max at once
    );

    // -------------------------------------------------------------------------
    // WORLD SYNC SCHEDULE - Periodic world persistence
    // -------------------------------------------------------------------------
    // Saves the world state to disk periodically to prevent data loss.
    // Uses Skip behavior - if we miss a sync, just wait for the next one.
    let build_world_sync = |s: &mut Schedule| {
        s.add_systems(crate::systems::world_sync::sync_world);
    };
    timed.register(
        TimedSchedule::new("world_sync", Duration::from_secs(15), build_world_sync)
            .with_behavior(MissedTickBehavior::Skip),
    );

    // -------------------------------------------------------------------------
    // CHUNK GC SCHEDULE - Periodic chunk garbage collection
    // -------------------------------------------------------------------------
    //
    // Cleans up unused chunks from memory to free resources.
    // Uses Skip behavior - if we miss a GC, just wait for the next one.
    let build_chunk_gc = |s: &mut Schedule| {
        s.add_systems(crate::systems::chunk_unloader::handle);
    };
    timed.register(
        TimedSchedule::new("chunk_gc", Duration::from_secs(5), build_chunk_gc)
            .with_behavior(MissedTickBehavior::Skip),
    );

    // -------------------------------------------------------------------------
    // KEEPALIVE SCHEDULE - Prevents client timeout disconnects
    // -------------------------------------------------------------------------
    // Sends keepalive packets to all connected players to maintain the connection.
    // Has a 250ms phase offset to spread load away from tick boundaries.
    let build_keepalive = |s: &mut Schedule| {
        s.add_systems(crate::systems::keep_alive_system::keep_alive_system);
    };
    timed.register(
        TimedSchedule::new("keepalive", Duration::from_secs(1), build_keepalive)
            .with_behavior(MissedTickBehavior::Skip)
            .with_phase(Duration::from_millis(250)), // Offset from tick schedule
    );

    // -------------------------------------------------------------------------
    // PLUGIN SCHEDULES - Dynamically registered by plugins
    // -------------------------------------------------------------------------
    // Drain any schedules that plugins registered during initialization.
    for pending in drain_registered_schedules() {
        timed.register(pending.into_timed());
    }

    timed
}

/// Spawns the LAN broadcast pinger task.
///
/// This broadcasts the server's presence on the local network using UDP multicast
/// to Mojang's LAN discovery address (224.0.2.60:4445). Minecraft clients scanning
/// for LAN games will pick up these broadcasts.
///
/// The 1.5 second interval is a balance between:
/// - Fast enough for clients to discover the server quickly
/// - Slow enough to not spam the network with unnecessary traffic
async fn spawn_lan_pinger() {
    let Ok(mut pinger) = LanPinger::new().await else {
        error!("Failed creating LAN pinger");
        return;
    };

    loop {
        pinger.send().await;
        sleep(Duration::from_millis(1500)).await;
    }
}

/// Spawns a dedicated thread for accepting TCP connections.
///
/// This function creates a new OS thread with its own Tokio async runtime that:
/// 1. Starts a LAN pinger to broadcast the server on local network
/// 2. Listens for incoming TCP connections on the configured port
/// 3. Spawns a handler task for each new connection
/// 4. Responds to shutdown signals for graceful termination
///
/// # Arguments
/// * `state` - Global server state (shared across all connections)
/// * `packet_sender` - Channel for sending outgoing packets
/// * `sender` - Channel to notify main loop of new connections
/// * `shutdown_notify` - Receives signal when server is shutting down
/// * `shutdown_response` - Sends confirmation when this thread has stopped
///
/// # Why a separate thread?
/// The network acceptor runs on its own thread with a dedicated Tokio runtime
/// to isolate async I/O from the main game loop. This prevents network lag
/// from affecting game tick timing and vice versa.
fn tcp_conn_acceptor(
    state: GlobalState,
    packet_sender: Arc<PacketSender>,
    sender: Arc<Sender<NewConnection>>,
    mut shutdown_notify: tokio::sync::oneshot::Receiver<()>,
    shutdown_response: Sender<()>,
) -> Result<(), BinaryError> {
    let named_thread = std::thread::Builder::new().name("TokioNetworkThread".to_string());
    named_thread.spawn(move || {
        // Catch panics to ensure graceful shutdown even if something goes wrong
        // We catch it so we can shut down the entire server instead of leaving it open with a crashed network loop
        let caught_panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // Create a dedicated single-threaded Tokio runtime for networking
            let async_runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .thread_name("Tokio-Async-Network")
                .build()?;

            // Spawn LAN broadcast pinger (for local network server discovery)
            async_runtime.spawn(spawn_lan_pinger());

            // Main connection accept loop
            async_runtime.block_on({
                let state = Arc::clone(&state);
                async move {
                    // Create the TCP listener on the configured address/port
                    let Ok(listener) = create_server_listener().await else {
                        error!("Failed to create TCP listener");
                        return Err::<(), BinaryError>(BinaryError::Custom(
                            "Failed to create TCP listener".to_string(),
                        ));
                    };

                    // Accept connections until shutdown is signaled
                    while !state.shut_down.load(std::sync::atomic::Ordering::Relaxed) {
                        // Use tokio::select! to handle both new connections AND shutdown signal
                        tokio::select! {
                            // Branch 1: New TCP connection incoming
                            accept_result = listener.accept() => {
                                match accept_result {
                                    Ok((stream, _)) => {
                                        let addy = stream.peer_addr()?;
                                        debug!("Got TCP connection from {}", addy);

                                        // Spawn a task to handle this connection asynchronously
                                        tokio::spawn({
                                            let state = Arc::clone(&state);
                                            let packet_sender = Arc::clone(&packet_sender);
                                            let sender = Arc::clone(&sender);
                                            async move {
                                                // handle_connection manages the full lifecycle:
                                                // handshake -> login -> play -> disconnect
                                                _ = handle_connection(state, stream, packet_sender, sender)
                                                    .instrument(info_span!("conn", %addy).or_current())
                                                    .await;
                                            }
                                        });
                                    }
                                    Err(e) => {
                                        error!("Failed to accept TCP connection: {:?}", e);
                                    }
                                }
                            }
                            // Branch 2: Shutdown signal received
                            _ = &mut shutdown_notify => {
                                trace!("Shutdown signal received on notify channel");
                                break;
                            }
                        }
                    }
                    Ok(())
                }
            })?;

            trace!("Shutting down TCP connection acceptor");

            // Notify main loop that we've finished shutting down
            shutdown_response.send(()).expect("Failed to send shutdown response");
            Ok::<(), BinaryError>(())
        }));

        // Handle panic case - ensure server shuts down cleanly
        if let Err(e) = caught_panic {
            error!("TCP connection acceptor thread panicked: {:?}", e);
            // Set shutdown flag so the main loop knows something went wrong
            state
                .shut_down
                .store(true, std::sync::atomic::Ordering::Relaxed);
            return Err::<(), BinaryError>(BinaryError::Custom(
                "TCP connection acceptor thread panicked".to_string(),
            ));
        }
        Err(BinaryError::Custom(
            "TCP connection acceptor thread panicked".to_string(),
        ))
    })?;
    Ok(())
}
