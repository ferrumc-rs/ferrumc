use crate::errors::BinaryError;
use crate::systems::definition::{create_systems, System};
use ferrumc_config::statics::get_global_config;
use ferrumc_net::connection::handle_connection;
use ferrumc_net::packets::IncomingPacket;
use ferrumc_state::GlobalState;
use ferrumc_threadpool::ThreadPool;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{info, info_span, trace, warn, Instrument};

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
        run_systems(global_state.clone(), &threadpool, &systems)?;

        // Process incoming packets

        // Sleep to maintain the tick rate
        let elapsed_time = start_time.elapsed();
        let sleep_duration = if elapsed_time < ns_per_tick {
            ns_per_tick - elapsed_time
        } else {
            Duration::ZERO
        };

        if sleep_duration > Duration::ZERO {
            info!(
                "Server tick took {:?}, sleeping for {:?}",
                elapsed_time, sleep_duration
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
                .run(state)
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
    thread_pool: ThreadPool,
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
            let result = packet.handle(id, state);
            if let Err(e) = result {
                warn!("Error processing packet: {:?}", e);
            };
            Ok(())
        });
    }
}

fn tcp_conn_accepter(
    state: GlobalState,
    packet_queue: Arc<Mutex<Vec<(Box<dyn IncomingPacket + Send + 'static>, usize)>>>,
) -> Result<(), BinaryError> {
    let tcp_listener = &state.tcp_listener;
    while !state.shut_down.load(std::sync::atomic::Ordering::Relaxed) {
        let (stream, _) = tcp_listener.accept()?;
        let addy = stream.peer_addr()?;
        std::thread::spawn({
            let state = Arc::clone(&state);
            let packet_queue = Arc::clone(&packet_queue);
            move || {
                let _ = handle_connection(state, stream, packet_queue)
                    .instrument(info_span!("conn", %addy).or_current());
            }
        });
        info!("Accepted connection from {}", addy);
    }
    Ok(())
}
