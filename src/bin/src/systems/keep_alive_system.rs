use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::conn::keepalive::KeepAliveTracker;
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;
use std::time::{Duration, SystemTime};
use tracing::warn;

pub fn keep_alive_system(
    mut query: Query<(Entity, &mut KeepAliveTracker, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    let now = std::time::Instant::now(); // faster than SystemTime for diffs
    const TIMEOUT: Duration = Duration::from_secs(15);
    const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(10);

    for (entity, mut tracker, stream_writer) in query.iter_mut() {
        // Skip if connection is already closed
        if !stream_writer
            .running
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            continue;
        }

        let elapsed = now.duration_since(tracker.last_received_keep_alive);

        // Kill connection if timed out
        if elapsed > TIMEOUT {
            warn!(
                "Killing connection for {}, it's been {:?} since last keepalive response",
                entity, elapsed
            );
            state
                .0
                .players
                .disconnect(entity, Some("Connection timed out".to_string()));
            continue;
        }

        // Send keepalive if needed
        if elapsed >= KEEPALIVE_INTERVAL && tracker.has_received_keep_alive {
            let timestamp = rand::random::<i64>(); // or use a counter
            let packet =
                ferrumc_net::packets::outgoing::keep_alive::OutgoingKeepAlivePacket { timestamp };

            if let Err(err) = stream_writer.send_packet_ref(&packet) {
                warn!("Failed to send keep alive packet to {}: {:?}", entity, err);
            }

            tracker.last_sent_keep_alive = timestamp;
            tracker.has_received_keep_alive = false;
        }
    }
}
