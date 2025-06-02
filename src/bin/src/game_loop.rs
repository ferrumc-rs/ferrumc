use crate::errors::BinaryError;
use crate::packet_handlers::{play_packets, register_player_systems};
use crate::register_events::register_events;
use crate::register_resources::register_resources;
use crate::systems::register_game_systems;
use bevy_ecs::prelude::World;
use bevy_ecs::schedule::ExecutorKind;
use crossbeam_channel::Sender;
use ferrumc_config::statics::get_global_config;
use ferrumc_net::connection::{handle_connection, NewConnection};
use ferrumc_net::server::create_server_listener;
use ferrumc_net::PacketSender;
use ferrumc_state::{GlobalState, GlobalStateResource};
use play_packets::register_packet_handlers;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, info_span, trace, warn, Instrument};

const NS_PER_SECOND: u64 = 1_000_000_000;

pub fn start_game_loop(global_state: GlobalState) -> Result<(), BinaryError> {
    // Setup channels and stuff for new connections
    let mut ecs_world = World::new();
    let sender_struct = Arc::new(ferrumc_net::create_packet_senders(&mut ecs_world));
    let (new_conn_send, new_conn_recv) = crossbeam_channel::unbounded();

    let global_state_res = GlobalStateResource(global_state.clone());

    let mut schedule = bevy_ecs::schedule::Schedule::default();
    schedule.set_executor_kind(ExecutorKind::MultiThreaded);

    register_events(&mut ecs_world);
    register_resources(&mut ecs_world, new_conn_recv, global_state_res);
    register_packet_handlers(&mut schedule);
    register_player_systems(&mut schedule);
    register_game_systems(&mut schedule);

    let ns_per_tick = Duration::from_nanos(NS_PER_SECOND / get_global_config().tps as u64);

    // Start the TCP connection acceptor
    tcp_conn_acceptor(global_state.clone(), sender_struct, Arc::new(new_conn_send))?;

    while !global_state
        .shut_down
        .load(std::sync::atomic::Ordering::Relaxed)
    {
        let start_time = std::time::Instant::now();

        // Run the ECS schedule
        schedule.run(&mut ecs_world);

        // Sleep to maintain the tick rate
        let elapsed_time = start_time.elapsed();
        let sleep_duration = if elapsed_time < ns_per_tick {
            ns_per_tick - elapsed_time
        } else {
            Duration::ZERO
        };

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
                elapsed_time, ns_per_tick
            );
        }
    }

    Ok(())
}

// This is the bit where we bridge to async
fn tcp_conn_acceptor(
    state: GlobalState,
    packet_sender: Arc<PacketSender>,
    sender: Arc<Sender<NewConnection>>,
) -> Result<(), BinaryError> {
    let named_thread = std::thread::Builder::new().name("TokioNetworkThread".to_string());
    named_thread.spawn(move || {
        let caught_panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            debug!("Created TCP connection acceptor thread");
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
                        debug!("Waiting for TCP connection...");
                        let (stream, _) = listener
                            .accept()
                            .await
                            .expect("Failed to accept TCP connection");
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
                        info!("Accepted connection from {}", addy);
                    }
                    debug!("Shutting down TCP connection acceptor thread");
                    Ok(())
                }
            })?;
            info!("Shutting down TCP connection acceptor");
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
