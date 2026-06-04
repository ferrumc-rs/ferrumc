use bevy_ecs::prelude::{MessageReader, Query};
use bevy_math::IVec2;
use ferrumc_components::player::client_information::ClientInformationComponent;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::chunks::chunk_receiver::{effective_view_radius, ChunkReceiver};
use ferrumc_core::transform::position::Position;
use ferrumc_messages::chunk_calc::ChunkCalc;
use ferrumc_world::pos::ChunkPos;
use std::collections::HashSet;
use tracing::warn;

pub fn handle(
    mut messages: MessageReader<ChunkCalc>,
    mut query: Query<(&Position, &mut ChunkReceiver, &ClientInformationComponent)>,
) {
    for message in messages.read() {
        let (position, mut chunk_receiver, client_info) = match query.get_mut(message.0) {
            Ok(data) => data,
            Err(_) => {
                warn!("Player does not exist, skipping chunk calculation");
                continue;
            } // Skip if the player does not exist
        };

        let server_render_distance = get_global_config().chunk_render_distance as i32;
        let client_view_distance = client_info.view_distance as i32;
        let radius = effective_view_radius(server_render_distance, client_view_distance);
        let player_chunk = ChunkPos::from(position.coords);

        // Chunks already queued this session, so we don't push duplicates into `loading`. The
        // `loading` deque has no membership check of its own, and `ChunkCalc` fires on every
        // chunk-boundary crossing, so without this a chunk that is queued but not yet sent would
        // be re-queued on every move — piling up stale duplicates that crowd out genuinely new
        // chunks under the per-tick send limit.
        let already_queued: HashSet<(i32, i32)> = chunk_receiver.loading.iter().copied().collect();

        let mut queued_chunks = Vec::new();

        // Add all chunks within the (square / Chebyshev) radius to the loading list if not already
        // loaded or already queued. This MUST use the same metric the sender filters with.
        for x in player_chunk.x() - radius..=player_chunk.x() + radius {
            for z in player_chunk.z() - radius..=player_chunk.z() + radius {
                let chunk_coords = (x, z);
                // Skip chunks already sent (`loaded`), already queued this session
                // (`already_queued`), or currently being generated off-thread (`pending`). Without
                // the `pending` check, an in-flight chunk — popped from `loading` but not yet sent —
                // would be re-queued on every move and resubmitted to the thread pool.
                if !chunk_receiver.loaded.contains(&chunk_coords)
                    && !already_queued.contains(&chunk_coords)
                    && !chunk_receiver.pending.contains(&chunk_coords)
                {
                    queued_chunks.push(chunk_coords);
                }
            }
        }

        // Sort loading list to prioritize closer chunks
        queued_chunks.sort_by_key(|(x, z)| {
            let as_vec = IVec2::new(*x, *z);
            as_vec.chebyshev_distance(IVec2::new(player_chunk.x(), player_chunk.z()))
        });

        for coords in queued_chunks {
            chunk_receiver.loading.push_back(coords);
        }

        // Unload chunks that are outside the radius

        for loaded_chunk in chunk_receiver.loaded.clone() {
            let vec = IVec2::new(loaded_chunk.0, loaded_chunk.1);
            let dx = IVec2::new(player_chunk.x(), player_chunk.z()).chebyshev_distance(vec);
            if dx > radius as u32 {
                if let Some(pos) = chunk_receiver.loaded.take(&loaded_chunk) {
                    chunk_receiver.unloading.push_back(pos);
                }
            }
        }
    }
}
