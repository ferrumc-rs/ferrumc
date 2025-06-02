use bevy_ecs::prelude::{Commands, Entity, EventReader, EventWriter, Query};
use ferrumc_core::conn::conn_kill_event::ConnectionKillEvent;
use ferrumc_core::conn::force_player_recount_event::ForcePlayerRecountEvent;
use ferrumc_net::connection::StreamWriter;
use tracing::trace;

pub fn connection_killer(
    mut events: EventReader<ConnectionKillEvent>,
    query: Query<(Entity, &StreamWriter)>,
    mut force_events: EventWriter<ForcePlayerRecountEvent>,
    mut cmd: Commands,
) {
    let mut force_recount = false;
    for event in events.read() {
        let reason = event.reason.clone();
        for (entity, conn) in query.iter() {
            if entity == event.entity || !conn.running.load(std::sync::atomic::Ordering::Relaxed) {
                trace!(
                    "Killing connection for entity {:?} with reason: {:?}",
                    entity,
                    reason
                );
                conn.kill(reason.clone()).unwrap();
                force_recount = true;
            } else {
                // TODO: Send a message to all other players
            }
        }
        match cmd.get_entity(event.entity) {
            Ok(_) => {
                cmd.entity(event.entity).despawn();
            }
            Err(_) => {
                // Entity does not exist, do nothing
                // Probably means multiple systems are trying to kill the same entity
            }
        }
    }
    if force_recount {
        force_events.write(ForcePlayerRecountEvent);
    }
}
