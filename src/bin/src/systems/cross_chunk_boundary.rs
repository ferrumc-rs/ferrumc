use crate::systems::send_chunks::send_chunks;
use bevy_ecs::prelude::{EventReader, Query, Res};
use bevy_math::IVec2;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::chunks::cross_chunk_boundary_event::CrossChunkBoundaryEvent;
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;
use std::collections::HashSet;

pub fn cross_chunk_boundary(
    mut events: EventReader<CrossChunkBoundaryEvent>,
    mut query: Query<&mut StreamWriter>,
    state: Res<GlobalStateResource>,
) {
    if events.is_empty() {
        return;
    }
    for event in events.read() {
        if !state.0.players.is_connected(event.player) {
            continue; // Skip if the player is not connected
        }
        let radius = get_global_config().chunk_render_distance as i32;

        let mut old_chunk_seen = HashSet::new();
        for x in event.old_chunk.x - radius..event.old_chunk.x + radius {
            for z in event.old_chunk.y - radius..event.old_chunk.y + radius {
                old_chunk_seen.insert(IVec2::new(x, z));
            }
        }
        let mut new_chunk_seen = HashSet::new();
        for x in event.new_chunk.x - radius..event.new_chunk.x + radius {
            for z in event.new_chunk.y - radius..event.new_chunk.y + radius {
                new_chunk_seen.insert(IVec2::new(x, z));
            }
        }
        let needed_chunks: Vec<_> = new_chunk_seen
            .iter()
            .filter(|chunk| !old_chunk_seen.contains(chunk))
            .map(|chunk| (*chunk, "overworld".to_string()))
            .collect();
        let mut conn = query.get_mut(event.player).expect("Player does not exist");
        send_chunks(state.0.clone(), needed_chunks, &mut conn, event.new_chunk)
            .expect("Failed to send chunks")
    }
}
