/// THIS IS TEMPRORARY AND WILL BE MOVED TO FERRUMC-WORLD ONCE OTHER WORLD-RELATED PR'S ARE MERGED
pub fn load_or_generate_chunk<'a>(
    state: &'a std::sync::Arc<ferrumc_state::ServerState>,
    chunk_pos: ferrumc_world::pos::ChunkPos,
    dimension: &str,
) -> Result<ferrumc_world::RefChunk<'a>, ferrumc_world::errors::WorldError> {
    if state.world.chunk_exists(chunk_pos, dimension)? {
        state.world.load_chunk(chunk_pos, dimension)
    } else {
        let chunk = state
            .terrain_generator
            .generate_chunk(chunk_pos)
            .map_err(|err| {
                ferrumc_world::errors::WorldError::WorldGenerationError(format!(
                    "Failed to generate chunk at {:?}: {}",
                    chunk_pos, err
                ))
            })?;
        state.world.insert_chunk(chunk_pos, dimension, chunk)?;
        state.world.load_chunk(chunk_pos, dimension)
    }
}
/// THIS IS TEMPRORARY AND WILL BE MOVED TO FERRUMC-WORLD ONCE OTHER WORLD-RELATED PR'S ARE MERGED
pub fn load_or_generate_mut<'a>(
    state: &'a std::sync::Arc<ferrumc_state::ServerState>,
    chunk_pos: ferrumc_world::pos::ChunkPos,
    dimension: &str,
) -> Result<ferrumc_world::MutChunk<'a>, ferrumc_world::errors::WorldError> {
    if state.world.chunk_exists(chunk_pos, dimension)? {
        state.world.load_chunk_mut(chunk_pos, dimension)
    } else {
        let chunk = state
            .terrain_generator
            .generate_chunk(chunk_pos)
            .map_err(|err| {
                ferrumc_world::errors::WorldError::WorldGenerationError(format!(
                    "Failed to generate chunk at {:?}: {}",
                    chunk_pos, err
                ))
            })?;
        state.world.insert_chunk(chunk_pos, dimension, chunk)?;
        state.world.load_chunk_mut(chunk_pos, dimension)
    }
}
