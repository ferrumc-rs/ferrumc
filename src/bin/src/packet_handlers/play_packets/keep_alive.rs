use bevy_ecs::prelude::{EventWriter, Res};
use bevy_ecs::system::Query;
use ferrumc_core::conn::conn_kill_event::ConnectionKillEvent;
use ferrumc_core::conn::keepalive::KeepAliveTracker;
use ferrumc_net::IncomingKeepAlivePacketReceiver;
use tracing::{debug, error};

pub fn handle(
    events: Res<IncomingKeepAlivePacketReceiver>,
    mut query: Query<&mut KeepAliveTracker>,
    mut conn_kill: EventWriter<ConnectionKillEvent>,
) {
    if events.0.is_empty() {
        return;
    }
    for (event, eid) in &events.0 {
        let Ok(mut last_sent_keep_alive) = query.get_mut(eid) else {
            error!("Could not get keep alive tracker for entity {:?}", eid);
            continue;
        };
        if event.timestamp != last_sent_keep_alive.last_sent_keep_alive {
            debug!(
                "Invalid keep alive packet received from {:?} with id {:?} (expected {:?})",
                eid, event.timestamp, last_sent_keep_alive.last_sent_keep_alive
            );
            conn_kill.write(ConnectionKillEvent {
                reason: Some("Invalid keep alive packet".to_string()),
                entity: eid,
            });
        } else {
            last_sent_keep_alive.last_received_keep_alive = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards. oh no")
                .as_millis() as i64;
            debug!("Keep alive packet received from {:?}", eid);
        }
    }
}
