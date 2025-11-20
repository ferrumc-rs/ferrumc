use bevy_ecs::prelude::{Entity, Query, Res};

use ferrumc_components::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_components::state::server_state::GlobalStateResource;
use ferrumc_net::ChunkBatchAckReceiver;

use tracing::{error, warn};

pub fn handle(
    events: Res<ChunkBatchAckReceiver>,
    mut query: Query<(Entity, &mut ChunkReceiver)>,
    state: Res<GlobalStateResource>,
) {
    for (event, eid) in events.0.try_iter() {
        let Ok((eid, mut chunk_recv)) = query.get_mut(eid) else {
            error!(
                "Failed to get chunk receiver or connection for entity: {:?}",
                eid
            );
            continue;
        };
        if !state.0.players.is_connected(eid) {
            warn!(
                "Entity {:?} is not connected, cannot handle chunk batch ack",
                eid
            );
            continue;
        }
        chunk_recv.chunks_per_tick = event.chunks_per_tick;
    }
}
