use bevy_ecs::prelude::{MessageReader, Query};
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::transform::position::Position;
use ferrumc_messages::chunk_calc::ChunkCalc;

pub fn handle(
    mut messages: MessageReader<ChunkCalc>,
    mut query: Query<(&Position, &mut ChunkReceiver)>,
) {
    for message in messages.read() {
        let (position, mut chunk_receiver) = match query.get_mut(message.0) {
            Ok(data) => data,
            Err(_) => continue, // Skip if the player does not exist
        };

        let chunk_receiver = &mut *chunk_receiver;

        let radius = get_global_config().chunk_render_distance as i32;
        let player_chunk_x = position.x.floor() as i32 >> 4;
        let player_chunk_z = position.z.floor() as i32 >> 4;

        let mut queued_chunks = Vec::new();

        // Add all chunks within the radius to the loading list if not already loaded
        for x in player_chunk_x - radius..=player_chunk_x + radius {
            for z in player_chunk_z - radius..=player_chunk_z + radius {
                let chunk_coords = (x, z);
                if !chunk_receiver.loaded.contains(&chunk_coords) {
                    queued_chunks.push(chunk_coords);
                }
            }
        }

        // Sort loading list to prioritize closer chunks
        queued_chunks.sort_by_key(|(x, z)| {
            let dx = x - player_chunk_x;
            let dz = z - player_chunk_z;
            dx * dx + dz * dz
        });

        for coords in queued_chunks {
            chunk_receiver.loading.push_back(coords);
        }

        // TODO: Handle unloading of distant chunks
    }
}
