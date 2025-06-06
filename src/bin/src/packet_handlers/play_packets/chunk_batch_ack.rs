use bevy_ecs::prelude::{Query, Res};
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::ChunkBatchAckReceiver;
use tracing::error;

pub fn handle(
    events: Res<ChunkBatchAckReceiver>,
    mut query: Query<(&mut ChunkReceiver, &mut StreamWriter)>,
) {
    for (event, eid) in events.0.try_iter() {
        let Ok((mut chunk_recv, conn)) = query.get_mut(eid) else {
            error!(
                "Failed to get chunk receiver or connection for entity: {:?}",
                eid
            );
            continue;
        };
        if !conn.running.load(std::sync::atomic::Ordering::Relaxed) {
            continue;
        }
        chunk_recv.chunks_per_tick = event.chunks_per_tick;
    }
}
