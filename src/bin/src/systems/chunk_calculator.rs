use bevy_ecs::prelude::{MessageReader, Query};
use bevy_math::IVec2;
use ferrumc_components::player::client_information::ClientInformation;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::transform::position::Position;
use ferrumc_messages::chunk_calc::ChunkCalc;
use ferrumc_world::pos::ChunkPos;

pub fn handle(
    mut messages: MessageReader<ChunkCalc>,
    mut query: Query<(&Position, &mut ChunkReceiver, &ClientInformation)>,
) {
    for message in messages.read() {
        let (position, mut chunk_receiver, client_info) = match query.get_mut(message.0) {
            Ok(data) => data,
            Err(_) => continue, // Skip if the player does not exist
        };

        let server_render_distance = get_global_config().chunk_render_distance as i32;
        let client_view_distance = client_info.view_distance as i32;
        let radius = server_render_distance.min(client_view_distance);
        let player_chunk = ChunkPos::from(position.coords);

        let mut queued_chunks = Vec::new();

        // Add all chunks within the radius to the loading list if not already loaded
        for x in player_chunk.x() - radius..=player_chunk.x() + radius {
            for z in player_chunk.z() - radius..=player_chunk.z() + radius {
                let chunk_coords = (x, z);
                if !chunk_receiver.loaded.contains(&chunk_coords) {
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
