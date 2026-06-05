//! Mountain biome (windswept hills): bare stone. Snow capping above the snow line is applied by the
//! post-cave surface-finish pass in `generate_chunk`, so the cap sits on the real (possibly carved)
//! surface rather than floating.

use crate::BiomeGenerator;
use crate::errors::WorldGenError;
use ferrumc_world::chunk::Chunk;

pub(crate) struct MountainBiome {}

impl BiomeGenerator for MountainBiome {
    fn biome_id(&self) -> u8 {
        62 // minecraft:windswept_hills
    }

    fn _biome_name(&self) -> String {
        "mountain".to_string()
    }

    fn decorate(&self, _: &mut Chunk, _: u8, _: u8, _: i16) -> Result<(), WorldGenError> {
        // Bare stone is already in place from the carving stage; snow capping is handled later.
        Ok(())
    }

    fn new(_: u64) -> Self {
        MountainBiome {}
    }
}
