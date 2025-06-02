use ferrumc_world::errors::WorldError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorldGenError {
    #[error("Failed to generate biome: {0}")]
    BiomeGenerationError(String),
    #[error("Failed to generate chunk: {0}")]
    ChunkGenerationError(String),
    #[error("World error: {0}")]
    WorldError(#[from] WorldError),
}
