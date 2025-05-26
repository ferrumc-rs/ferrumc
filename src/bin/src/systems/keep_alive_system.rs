use bevy_ecs::prelude::{Entity, EventWriter, Query};
use ferrumc_core::conn::conn_kill_event::ConnectionKillEvent;
use ferrumc_core::conn::keepalive::KeepAliveTracker;
use ferrumc_net::connection::StreamWriter;
use std::time::{Duration, SystemTime};
use tracing::warn;

pub fn keep_alive_system(
    query: Query<(Entity, &mut KeepAliveTracker, &StreamWriter)>,
    mut connection_kill_event: EventWriter<ConnectionKillEvent>,
) {
    // Get the times before the queries, since it's possible a query takes more than a millisecond with a lot of entities.

    let current_time = SystemTime::now();

    for (entity, mut keep_alive_tracker, stream_writer) in query {
        if !stream_writer
            .running
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            continue;
        }
        // If it's been more than 15 seconds since the last keep alive packet was received, kill the connection
        let time_diff = current_time
            .duration_since(keep_alive_tracker.last_received_keep_alive)
            .expect("SystemTime::duration_since failed, this should never happen");
        if time_diff > Duration::from_secs(15) {
            warn!(
                "Killing connection for {}, it's been {:?} since last keepalive response",
                entity, time_diff
            );
            connection_kill_event.write(ConnectionKillEvent {
                reason: Some("Keep alive timeout".to_string()),
                entity,
            });
        } else if time_diff >= Duration::from_secs(10) && keep_alive_tracker.has_received_keep_alive
        {
            // If it's been more than 10 seconds since the last keep alive packet was sent, send a new one
            let time_stamp = rand::random();
            let keep_alive_packet =
                ferrumc_net::packets::outgoing::keep_alive::OutgoingKeepAlivePacket {
                    timestamp: time_stamp,
                };
            if let Err(err) = stream_writer.send_packet(keep_alive_packet) {
                warn!("Failed to send keep alive packet to {}: {:?}", entity, err);
            }
            keep_alive_tracker.last_sent_keep_alive = time_stamp;
            keep_alive_tracker.has_received_keep_alive = false;
        }
    }
}
