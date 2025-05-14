use bevy_ecs::prelude::EventWriter;
use bevy_ecs::system::Query;
use ferrumc_core::conn::conn_kill_event::ConnectionKillEvent;
use ferrumc_core::conn::keepalive::KeepAliveTracker;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::keep_alive::IncomingKeepAlivePacket;
use ferrumc_net::packets::outgoing::keep_alive::OutgoingKeepAlivePacket;
use ferrumc_net::utils::state::terminate_connection;
use ferrumc_net::IncomingKeepAlivePacketReceiver;
use ferrumc_state::ServerState;
use std::sync::Arc;
use tracing::{debug, error};

pub fn handle(events: IncomingKeepAlivePacketReceiver, mut query: Query<&mut KeepAliveTracker>, conn_kill: EventWriter<ConnectionKillEvent>) {
    for (event, eid) in events.0 {
        let Ok(last_sent_keep_alive) = query.get_mut(eid) else {
            error!("Could not get keep alive tracker for entity {:?}", eid);
            continue;
        };
        if event.timestamp != last_sent_keep_alive.last_sent_keep_alive {
            debug!(
                    "Invalid keep alive packet received from {:?} with id {:?} (expected {:?})",
                    eid, event.timestamp, last_sent_keep_alive.last_sent_keep_alive
                );
            if let Err(e) =
                terminate_connection(state, conn_id, "Invalid keep alive packet".to_string())
            {
                debug!("Error terminating connection: {:?}", e);
            }
        }
    }
}