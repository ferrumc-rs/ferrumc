use bevy_ecs::prelude::{EventWriter, Res};
use bevy_ecs::system::Query;
use ferrumc_core::conn::conn_kill_event::ConnectionKillEvent;
use ferrumc_core::conn::keepalive::KeepAliveTracker;
use ferrumc_net::IncomingKeepAlivePacketReceiver;
use std::time::SystemTime;
use tracing::{error, warn};

pub fn handle(
    events: Res<IncomingKeepAlivePacketReceiver>,
    mut query: Query<&mut KeepAliveTracker>,
    mut conn_kill: EventWriter<ConnectionKillEvent>,
) {
    for (event, eid) in events.0.try_iter() {
        let Ok(mut keep_alive_tracker) = query.get_mut(eid) else {
            error!("Could not get keep alive tracker for entity {:?}", eid);
            continue;
        };
        if event.timestamp != keep_alive_tracker.last_sent_keep_alive {
            warn!(
                "Invalid keep alive packet received from {:?} with id {:?} (expected {:?})",
                eid, event.timestamp, keep_alive_tracker.last_sent_keep_alive
            );
            conn_kill.write(ConnectionKillEvent {
                reason: Some("Invalid keep alive packet".to_string()),
                entity: eid,
            });
        } else {
            keep_alive_tracker.last_received_keep_alive = SystemTime::now();
            keep_alive_tracker.has_received_keep_alive = true;
        }
    }
}
