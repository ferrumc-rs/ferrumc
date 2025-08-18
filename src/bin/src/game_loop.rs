use crate::errors::BinaryError;
use crate::packet_handlers::{play_packets, register_player_systems};
use crate::register_events::register_events;
use crate::register_resources::register_resources;
use crate::systems::register_game_systems;
use crate::systems::shutdown_systems::register_shutdown_systems;
use bevy_ecs::prelude::World;
use bevy_ecs::schedule::ExecutorKind;
use crossbeam_channel::Sender;
use ferrumc_commands::infrastructure::register_command_systems;
use ferrumc_config::server_config::get_global_config;
use ferrumc_net::connection::{handle_connection, NewConnection};
use ferrumc_net::server::create_server_listener;
use ferrumc_net::PacketSender;
use ferrumc_state::{GlobalState, GlobalStateResource};
use ferrumc_utils::formatting::format_duration;
use play_packets::register_packet_handlers;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, info_span, trace, warn, Instrument};

pub fn start_game_loop(global_state: GlobalState) -> Result<(), BinaryError> {
    // Setup the ECS world and schedules
    let mut ecs_world = World::new();

    let mut schedule = bevy_ecs::schedule::Schedule::default();
    schedule.set_executor_kind(ExecutorKind::SingleThreaded);

    // This schedule is ticked once when the server is shutting down
    // If you need to run any cleanup systems, add them to `ferrumc::systems::shutdown_systems::register_shutdown_systems`
    let mut shutdown_schedule = bevy_ecs::schedule::Schedule::default();

    // Setup channels and stuff for new connections
    let sender_struct = Arc::new(ferrumc_net::create_packet_senders(&mut ecs_world));
    let (new_conn_send, new_conn_recv) = crossbeam_channel::unbounded();

    // Setup shutdown related channels
    let (shutdown_send, shutdown_recv) = tokio::sync::oneshot::channel();
    let (shutdown_response_send, shutdown_response_recv) = crossbeam_channel::unbounded();

    ferrumc_default_commands::init();

    // Register systems and resources
    let global_state_res = GlobalStateResource(global_state.clone());

    register_events(&mut ecs_world);
    register_resources(&mut ecs_world, new_conn_recv, global_state_res);
    register_packet_handlers(&mut schedule);
    register_player_systems(&mut schedule);
    register_command_systems(&mut schedule);
    register_game_systems(&mut schedule);

    register_shutdown_systems(&mut shutdown_schedule);

    let time_per_tick = Duration::from_secs(1) / get_global_config().tps;

    // Start the TCP connection acceptor
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

    while !global_state
        .shut_down
        .load(std::sync::atomic::Ordering::Relaxed)
    {
        let tick_start = Instant::now();
        // Run the ECS schedule
        schedule.run(&mut ecs_world);

        // Sleep to maintain the tick rate
        let elapsed_time = tick_start.elapsed();
        let sleep_duration = time_per_tick.saturating_sub(elapsed_time);

        if sleep_duration > Duration::ZERO {
            trace!(
                "Server tick took {:?}, sleeping for {:?}",
                elapsed_time,
                sleep_duration
            );
            std::thread::sleep(sleep_duration);
        } else {
            warn!(
                "Server tick took too long: {:?}, max {:?}",
                elapsed_time, time_per_tick
            );
        }
    }

    shutdown_schedule.run(&mut ecs_world);

    // tell the TCP connection acceptor to shut down
    trace!("Sending shutdown signal to TCP connection acceptor");
    shutdown_send
        .send(())
        .expect("Failed to send shutdown signal");

    // Wait until the TCP connection acceptor has shut down
    trace!("Waiting for TCP connection acceptor to shut down");
    shutdown_response_recv
        .recv()
        .expect("Failed to receive shutdown response");

    Ok(())
}

// This is the bit where we bridge to async
fn tcp_conn_acceptor(
    state: GlobalState,
    packet_sender: Arc<PacketSender>,
    sender: Arc<Sender<NewConnection>>,
    mut shutdown_notify: tokio::sync::oneshot::Receiver<()>,
    shutdown_response: Sender<()>,
) -> Result<(), BinaryError> {
    let named_thread = std::thread::Builder::new().name("TokioNetworkThread".to_string());
    named_thread.spawn(move || {
        let caught_panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let async_runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .thread_name("Tokio-Async-Network")
                .build()?;
            async_runtime.block_on({
                let state = Arc::clone(&state);
                async move {
                    let Ok(listener) = create_server_listener().await else {
                        error!("Failed to create TCP listener");
                        return Err::<(), BinaryError>(BinaryError::Custom(
                            "Failed to create TCP listener".to_string(),
                        ));
                    };
                    while !state.shut_down.load(std::sync::atomic::Ordering::Relaxed) {
                        // Wait for a new connection or shutdown signal
                        tokio::select! {
                            accept_result = listener.accept() => {
                                match accept_result {
                                    Ok((stream, _)) => {
                                        let addy = stream.peer_addr()?;
                                        debug!("Got TCP connection from {}", addy);
                                        tokio::spawn({
                                            let state = Arc::clone(&state);
                                            let packet_sender = Arc::clone(&packet_sender);
                                            let sender = Arc::clone(&sender);
                                            async move {
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

            shutdown_response.send(()).expect("Failed to send shutdown response");
            Ok::<(), BinaryError>(())
        }));
        if let Err(e) = caught_panic {
            error!("TCP connection acceptor thread panicked: {:?}", e);
            // If we get here, the thread panicked
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
