use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_components::chunks::{ChunkCommand, ChunkSender};
use ferrumc_net::ChunkBatchAckReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::{error, trace, warn};

/// Handles ChunkBatchAck packets from clients.
///
/// When the client finishes processing a chunk batch, it sends this packet with
/// its desired chunks-per-tick rate. We forward this to the async chunk loader
/// task so it can pace the next batch accordingly.
pub fn handle(
    receiver: Res<ChunkBatchAckReceiver>,
    query: Query<(Entity, &ChunkSender)>,
    state: Res<GlobalStateResource>,
) {
    for (event, eid) in receiver.0.try_iter() {
        let Ok((eid, chunk_sender)) = query.get(eid) else {
            error!("Failed to get ChunkSender for entity: {:?}", eid);
            continue;
        };

        if !state.0.players.is_connected(eid) {
            warn!(
                "Entity {:?} is not connected, cannot handle chunk batch ack",
                eid
            );
            continue;
        }

        // Forward the rate to the async chunk loader task
        if let Err(e) = chunk_sender
            .tx
            .try_send(ChunkCommand::BatchReceived(event.chunks_per_tick))
        {
            trace!(
                "Failed to send BatchReceived to chunk loader for {:?}: {}",
                eid,
                e
            );
        }
    }
}
