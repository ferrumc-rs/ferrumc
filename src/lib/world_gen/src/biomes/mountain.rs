use crate::errors::WorldGenError;
use crate::BiomeGenerator;
use ferrumc_world::chunk_format::Chunk;

pub(crate) struct MountainBiome {}

impl BiomeGenerator for MountainBiome {
    fn _biome_id(&self) -> u8 {
        1 // Example ID for mountain biome
    }

    fn _biome_name(&self) -> String {
        "mountain".to_string()
    }

    fn decorate(&self, chunk: &mut Chunk, x: u8, z: u8) -> Result<(), WorldGenError> {
        // Doesn't generate any blocks, just a placeholder
        Ok(())
    }

    fn new(_: u64) -> Self
    where
        Self: Sized,
    {
        MountainBiome {}
    }
}