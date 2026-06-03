//! Mountain biome: bare stone. A placeholder decorator that leaves the carved stone surface
//! exposed (no soil cover), matching the upstream branch.

use crate::BiomeGenerator;
use crate::errors::WorldGenError;
use ferrumc_world::chunk::Chunk;

pub(crate) struct MountainBiome {}

impl BiomeGenerator for MountainBiome {
    fn _biome_id(&self) -> u8 {
        1
    }

    fn _biome_name(&self) -> String {
        "mountain".to_string()
    }

    fn decorate(&self, _: &mut Chunk, _: u8, _: u8, _: i16) -> Result<(), WorldGenError> {
        // Bare stone — nothing to place.
        Ok(())
    }

    fn new(_: u64) -> Self {
        MountainBiome {}
    }
}
