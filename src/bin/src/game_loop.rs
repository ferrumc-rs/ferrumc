use crate::errors::BinaryError;
use crate::systems::definition::{create_systems, System};
use ferrumc_config::statics::get_global_config;
use ferrumc_net::connection::handle_connection;
use ferrumc_net::packets::IncomingPacket;
use ferrumc_net::server::create_server_listener;
use ferrumc_state::GlobalState;
use ferrumc_threadpool::ThreadPool;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{debug, error, info, info_span, trace, warn, Instrument};

const NS_PER_SECOND: u64 = 1_000_000_000;

pub fn start_game_loop(global_state: GlobalState) -> Result<(), BinaryError> {
    let mut tick = 0u128;

    let ns_per_tick = Duration::from_nanos(NS_PER_SECOND / get_global_config().tps as u64);

    let systems = create_systems();

    let threadpool = ThreadPool::new();

    let queued_packets = Arc::new(Mutex::new(Vec::<(
        Box<dyn IncomingPacket + Send + 'static>,
        usize,
    )>::new()));

    // Start the TCP connection accepter
    let packet_queue = Arc::clone(&queued_packets);
    tcp_conn_accepter(global_state.clone(), packet_queue)?;

    while !global_state
        .shut_down
        .load(std::sync::atomic::Ordering::Relaxed)
    {
        let start_time = std::time::Instant::now();

        // Run the game systems
        run_systems(global_state.clone(), &threadpool, &systems, tick)?;

        // Process incoming packets
        process_packets(
            global_state.clone(),
            &threadpool,
            Arc::clone(&queued_packets),
        );

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

fn run_systems(
    global_state: GlobalState,
    thread_pool: &ThreadPool,
    systems: &[Arc<dyn System>],
    tick: u128,
) -> Result<(), BinaryError> {
    // Run each system in the thread pool
    let mut batch: ferrumc_threadpool::ThreadPoolBatch<'_, Result<_, BinaryError>> =
        thread_pool.batch();
    for system in systems {
        let system = Arc::clone(system);
        let state = Arc::clone(&global_state);
        batch.execute(move || {
            let sys_name = system.name().to_string();
            system
                .run(state, tick)
                .instrument(info_span!("system ", name = sys_name))
                .into_inner()?;
            Ok(())
        });
    }
    let results = batch.wait();
    for result in results {
        if let Err(e) = result {
            warn!("System error: {:?}", e);
        }
    }
    Ok(())
}

fn process_packets(
    state: GlobalState,
    thread_pool: &ThreadPool,
    packet_queue: Arc<Mutex<Vec<(Box<dyn IncomingPacket + Send + 'static>, usize)>>>,
) {
    // Move all the packets to a temporary vector so we don't hold the lock while processing
    let mut packets = Vec::new();
    {
        let mut queue = packet_queue.lock().unwrap();
        std::mem::swap(&mut packets, &mut *queue);
    }

    let mut batch: ferrumc_threadpool::ThreadPoolBatch<'_, Result<_, BinaryError>> =
        thread_pool.batch();
    for packet in packets {
        let state = Arc::clone(&state);
        batch.execute(move || {
            let (packet, id) = packet;
            let result = packet
                .handle(id, state)
                .instrument(info_span!("packet_handle", conn = id))
                .into_inner();
            if let Err(e) = result {
                warn!("Error processing packet: {:?}", e);
            };
            Ok(())
        });
    }
}

// This is the bit where we bridge to async
fn tcp_conn_accepter(
    state: GlobalState,
    packet_queue: Arc<Mutex<Vec<(Box<dyn IncomingPacket + Send + 'static>, usize)>>>,
) -> Result<(), BinaryError> {
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
                            let packet_queue = Arc::clone(&packet_queue);
                            async move {
                                let _ = handle_connection(state, stream, packet_queue)
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
