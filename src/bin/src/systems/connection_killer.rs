use bevy_ecs::prelude::{Commands, EventReader, Query};
use ferrumc_core::conn::conn_kill_event::ConnectionKillEvent;
use ferrumc_net::connection::StreamWriter;
use tracing::trace;

pub fn connection_killer(
    mut events: EventReader<ConnectionKillEvent>,
    mut query: Query<&mut StreamWriter>,
    mut cmd: Commands,
) {
    for event in events.read() {
        let reason = event.reason.clone().unwrap_or_else(|| "Unknown reason".to_string());
        if let Ok(mut conn) = query.get(event.entity) {
            conn.kill(Some(reason)).unwrap()
        } else {
            tracing::error!("Could not find StreamWriter for entity {:?}", event.entity);
        }
        trace!("Connection killed entity {:?}", event.entity);
        cmd.entity(event.entity).despawn();
    }
}