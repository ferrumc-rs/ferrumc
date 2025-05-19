use bevy_ecs::prelude::{Commands, Entity, EventReader, Query};
use ferrumc_core::conn::conn_kill_event::ConnectionKillEvent;
use ferrumc_net::connection::StreamWriter;

pub fn connection_killer(
    mut events: EventReader<ConnectionKillEvent>,
    query: Query<(Entity, &StreamWriter)>,
    mut cmd: Commands,
) {
    for event in events.read() {
        let reason = event.reason.clone().unwrap_or_else(|| "Unknown reason".to_string());
        for (entity, conn) in query.iter() {
            if entity == event.entity {
                conn.kill(Some(reason.clone())).unwrap();
            } else {
                // TODO: Send a message to all other players
            }
        }
        cmd.entity(event.entity).despawn();
    }
}