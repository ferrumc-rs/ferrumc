use crate::errors::BinaryError;
use crate::packet_handlers::{play_packets, register_player_systems};
use crate::register_events::register_events;
use crate::systems::new_connections::NewConnectionRecv;
use crate::systems::register_game_systems;
use bevy_ecs::prelude::World;
use bevy_ecs::schedule::ExecutorKind;
use crossbeam_channel::Sender;
use ferrumc_config::statics::get_global_config;
use ferrumc_net::connection::{handle_connection, NewConnection};
use ferrumc_net::packets::IncomingPacket;
use ferrumc_net::server::create_server_listener;
use ferrumc_net::PacketSender;
use ferrumc_state::{GlobalState, GlobalStateResource};
use ferrumc_threadpool::ThreadPool;
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
    ecs_world.insert_resource(NewConnectionRecv(new_conn_recv));
    let global_state_res = GlobalStateResource(global_state.clone());
    ecs_world.insert_resource(global_state_res);

    let mut schedule = bevy_ecs::schedule::Schedule::default();
    schedule.set_executor_kind(ExecutorKind::MultiThreaded);

    register_events(&mut ecs_world);

    register_packet_handlers(&mut schedule);
    register_player_systems(&mut schedule);
    register_game_systems(&mut schedule);

    let mut tick = 0u128;

    let ns_per_tick = Duration::from_nanos(NS_PER_SECOND / get_global_config().tps as u64);

    // let threadpool = ThreadPool::new();

    // Start the TCP connection accepter
    tcp_conn_accepter(global_state.clone(), sender_struct, Arc::new(new_conn_send))?;

    while !global_state
        .shut_down
        .load(std::sync::atomic::Ordering::Relaxed)
    {
        let start_time = std::time::Instant::now();

        // Run the ECS schedule
        schedule.run(&mut ecs_world);

        // Run the game systems
        // run_systems(global_state.clone(), &threadpool, &tick)?;

        // Process incoming packets
        // process_packets(global_state.clone(), &threadpool);

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
        // Increment the tick
        tick += 1;
    }

    Ok(())
}

// fn run_systems(
//     global_state: GlobalState,
//     thread_pool: &ThreadPool,
//     systems: &[Arc<dyn System>],
//     tick: u128,
// ) -> Result<(), BinaryError> {
//     // Run each system in the thread pool
//     let mut batch: ferrumc_threadpool::ThreadPoolBatch<'_, Result<_, BinaryError>> =
//         thread_pool.batch();
//     for system in systems {
//         let system = Arc::clone(system);
//         let state = Arc::clone(&global_state);
//         batch.execute(move || {
//             let sys_name = system.name().to_string();
//             system
//                 .run(state, tick)
//                 .instrument(info_span!("system ", name = sys_name))
//                 .into_inner()?;
//             Ok(())
//         });
//     }
//     let results = batch.wait();
//     for result in results {
//         if let Err(e) = result {
//             warn!("System error: {:?}", e);
//         }
//     }
//     Ok(())
// }

// fn process_packets(state: GlobalState, thread_pool: &ThreadPool) {
//     // Move all the packets to a temporary vector so we don't hold the lock while processing
//     let mut packets = Vec::new();
//     {
//         let query = state
//             .universe
//             .query::<&ferrumc_net::connection::LocalPacketQueue>();
//         for (eid, queue) in query {
//             let mut local_packets = vec![];
//             while let Some(packet) = queue.queue.pop() {
//                 local_packets.push(packet);
//             }
//             if !local_packets.is_empty() {
//                 packets.push((local_packets, eid));
//             }
//         }
//     }
// 
//     let mut batch: ferrumc_threadpool::ThreadPoolBatch<'_, Result<_, BinaryError>> =
//         thread_pool.batch();
//     for packet_set in packets {
//         let state = Arc::clone(&state);
//         batch.execute(move || {
//             let (packet_collection, id) = packet_set;
//             for packet in packet_collection {
//                 let result = packet
//                     .handle(id, state.clone())
//                     .instrument(info_span!("packet_handle", conn = id))
//                     .into_inner();
//                 if let Err(e) = result {
//                     warn!("Error processing packet: {:?}", e);
//                 };
//             }
//             Ok(())
//         });
//     }
// }

// This is the bit where we bridge to async
fn tcp_conn_accepter(state: GlobalState, packet_sender: Arc<PacketSender>, sender: Arc<Sender<NewConnection>>) -> Result<(), BinaryError> {
    let named_thread = std::thread::Builder::new().name("TokioNetworkThread".to_string());
    named_thread.spawn(move || {
        let caught_panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            debug!("Created TCP connection accepter thread");
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
                    debug!("Shutting down TCP connection accepter thread");
                    Ok(())
                }
            })?;
            info!("Shutting down TCP connection accepter");
            Ok::<(), BinaryError>(())
        }));
        if let Err(e) = caught_panic {
            error!("TCP connection accepter thread panicked: {:?}", e);
            // If we get here, the thread panicked
            state
                .shut_down
                .store(true, std::sync::atomic::Ordering::Relaxed);
            return Err::<(), BinaryError>(BinaryError::Custom(
                "TCP connection accepter thread panicked".to_string(),
            ));
        }
        Err(BinaryError::Custom(
            "TCP connection accepter thread panicked".to_string(),
        ))
    })?;
    Ok(())
}
