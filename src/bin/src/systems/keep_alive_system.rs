use crate::errors::BinaryError;
use bevy_ecs::prelude::{Entity, EventWriter, Query};
use ferrumc_core::conn::conn_kill_event::ConnectionKillEvent;
use ferrumc_core::conn::keepalive::KeepAliveTracker;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::incoming::keep_alive::IncomingKeepAlivePacket;
use ferrumc_net::packets::outgoing::keep_alive::OutgoingKeepAlivePacket;
use ferrumc_state::GlobalState;
use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{error, trace, warn};

pub struct KeepAliveSystem;

impl KeepAliveSystem {
    pub const fn new() -> Self {
        Self
    }
}

fn run(
    query: Query<(Entity, &KeepAliveTracker, &StreamWriter)>,
    mut connection_kill_event: EventWriter<ConnectionKillEvent>,
) -> Result<(), BinaryError> {
    // Get the times before the queries, since it's possible a query takes more than a millisecond with a lot of entities.

    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64;

    for (entity, keep_alive_tracker, stream_writer) in query {
        // If it's been more than 15 seconds since the last keep alive packet was received, kill the connection
        if current_time - keep_alive_tracker.last_received_keep_alive > 15_000 {
            warn!("Killing connection for {:?}", entity);
            connection_kill_event.write(ConnectionKillEvent {
                reason: Some("Keep alive timeout".to_string()),
                entity,
            });
        } else if current_time - keep_alive_tracker.last_sent_keep_alive > 1000 {
            trace!("Sending keep alive packet to {:?}", entity);
            let packet = OutgoingKeepAlivePacket::new(keep_alive_tracker.last_sent_keep_alive);
            stream_writer.send_packet(packet)?;
        }
    }
    Ok(())
}
