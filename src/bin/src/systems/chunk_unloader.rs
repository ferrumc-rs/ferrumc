use bevy_ecs::prelude::{Query, Res};
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::pos::ChunkPos;
use std::collections::HashSet;
use tracing::{debug, error};

pub fn handle(state: Res<GlobalStateResource>, query: Query<&ChunkReceiver>) {
    if query.count() == 0 {
        let mut removed = 0;
        for chunk_candidate in state.0.world.get_cache() {
            let ((pos, dim), chunk) = chunk_candidate.pair();
            removed += 1;
            state
                .0
                .world
                .insert_chunk(*pos, dim.as_str(), chunk.clone())
                .expect("Failed to re-insert chunk after unloading from cache.");
        }
        state.0.world.get_cache().clear();
        if removed > 0 {
            debug!(
                "Unloaded {} chunks from cache as there are no connected players.",
                removed
            );
        }
        return;
    }
    let all_chunks: HashSet<ChunkPos> = state
        .0
        .world
        .get_cache()
        .into_iter()
        .map(|chunk_candidate| {
            let (k, _v) = chunk_candidate.pair();
            k.0
        })
        .collect();
    let mut visible_chunks = HashSet::new();
    'chunk_iter: for chunk_candidate in state.0.world.get_cache() {
        let (k, v) = chunk_candidate.pair();
        for chunk_receiver in query.iter() {
            if chunk_receiver.loaded.contains(&(k.0.x(), k.0.z())) {
                visible_chunks.insert(k.0);
                continue 'chunk_iter;
            }
        }
    }
    let mut counter = 0;
    for chunk_pos in all_chunks.difference(&visible_chunks) {
        let removed_chunk = state
            .0
            .world
            .get_cache()
            .remove(&(*chunk_pos, "overworld".to_string()));
        match removed_chunk {
            Some(((pos, dim), chunk)) => {
                state
                    .0
                    .world
                    .insert_chunk(pos, dim.as_str(), chunk)
                    .expect("Failed to re-insert chunk after unloading from cache.");
                counter += 1;
            }
            None => {
                error!("Chunk at position {:?} could not be removed because it does not exist in the cache.", chunk_pos);
            }
        }
    }
    debug!("Unloaded {} chunks from cache.", counter);
}
