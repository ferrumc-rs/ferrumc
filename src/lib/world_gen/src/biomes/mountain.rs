use crate::BiomeGenerator;
use crate::errors::WorldGenError;
use ferrumc_world::chunk_format::Chunk;

pub(crate) struct MountainBiome {}

impl BiomeGenerator for MountainBiome {
    fn _biome_id(&self) -> u8 {
        1 // Example ID for mountain biome
    }

    fn _biome_name(&self) -> String {
        "mountain".to_string()
    }

    fn decorate(&self, _: &mut Chunk, _: u8, _: u8) -> Result<(), WorldGenError> {
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
