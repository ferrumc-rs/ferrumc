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
    let mut all_chunks: HashSet<ChunkPos> = HashSet::new();
    let mut visible_chunks = HashSet::new();
    'chunk_iter: for chunk_candidate in state.0.world.get_cache() {
        let (k, _v) = chunk_candidate.pair();
        // Track all chunk positions seen in the cache
        all_chunks.insert(k.0);
        // Track chunks that are visible to any connected player
        for chunk_receiver in query.iter() {
            if chunk_receiver.loaded.contains(&(k.0.x(), k.0.z())) {
                visible_chunks.insert(k.0);
                continue 'chunk_iter;
            }
        }
    }
    let mut unloaded_entries = 0;
    let mut written_chunks = 0;
    for chunk_pos in all_chunks.difference(&visible_chunks) {
        let removed_chunk = state
            .0
            .world
            .get_cache()
            .remove(&(*chunk_pos, "overworld".to_string()));
        match removed_chunk {
            Some(((pos, dim), chunk)) => {
                let dirty = chunk.sections.iter().any(|section| section.dirty);
                if dirty {
                    state
                        .0
                        .world
                        .insert_chunk(pos, dim.as_str(), chunk)
                        .expect("Failed to re-insert chunk after unloading from cache.");
                    written_chunks += 1;
                }
                unloaded_entries += 1;
            }
            None => {
                error!("Chunk at position {:?} could not be removed because it does not exist in the cache.", chunk_pos);
            }
        }
    }
    debug!(
        "Unloaded {} chunks from cache ({} written to world).",
        unloaded_entries, written_chunks
    );
}
