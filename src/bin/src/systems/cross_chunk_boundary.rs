use crate::systems::send_chunks::send_chunks;
use bevy_ecs::prelude::{MessageReader, Query, Res};
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::chunks::cross_chunk_boundary_event::ChunkBoundaryCrossed;
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;
use std::collections::HashSet;

pub fn cross_chunk_boundary(
    mut messages: MessageReader<ChunkBoundaryCrossed>,
    mut query: Query<&mut StreamWriter>,
    state: Res<GlobalStateResource>,
) {
    if messages.is_empty() {
        return;
    }
    for event in messages.read() {
        if !state.0.players.is_connected(event.player) {
            continue; // Skip if the player is not connected
        }
        let radius = get_global_config().chunk_render_distance as i32;

        let mut old_chunk_seen = HashSet::new();
        for x in event.old_chunk.0 - radius..event.old_chunk.0 + radius {
            for z in event.old_chunk.1 - radius..event.old_chunk.1 + radius {
                old_chunk_seen.insert((x, z));
            }
        }
        let mut new_chunk_seen = HashSet::new();
        for x in event.new_chunk.0 - radius..event.new_chunk.0 + radius {
            for z in event.new_chunk.1 - radius..event.new_chunk.1 + radius {
                new_chunk_seen.insert((x, z));
            }
        }
        let needed_chunks: Vec<_> = new_chunk_seen
            .iter()
            .filter(|chunk| !old_chunk_seen.contains(chunk))
            .map(|chunk| {
                let (x, z) = *chunk;
                (x, z, "overworld".to_string())
            })
            .collect();
        let center_chunk = (event.new_chunk.0, event.new_chunk.1);
        let mut conn = query.get_mut(event.player).expect("Player does not exist");
        send_chunks(state.0.clone(), needed_chunks, &mut conn, center_chunk)
            .expect("Failed to send chunks")
    }
}
