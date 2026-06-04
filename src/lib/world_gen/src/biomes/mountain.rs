//! Mountain biome (windswept hills): bare stone, capped with snow above the snow line.

use crate::BiomeGenerator;
use crate::errors::WorldGenError;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;

/// Surface height at and above which peaks are capped with snow.
const SNOW_LINE: i16 = 110;

pub(crate) struct MountainBiome {}

impl BiomeGenerator for MountainBiome {
    fn biome_id(&self) -> u8 {
        62 // minecraft:windswept_hills
    }

    fn _biome_name(&self) -> String {
        "mountain".to_string()
    }

    fn decorate(
        &self,
        chunk: &mut Chunk,
        x: u8,
        z: u8,
        surface_y: i16,
    ) -> Result<(), WorldGenError> {
        // Bare stone is already in place from the carving stage; cap high peaks with snow.
        if surface_y >= SNOW_LINE {
            chunk.set_block(
                ChunkBlockPos::new(x, surface_y + 1, z),
                block!("snow", { layers: 1 }),
            );
        }
        Ok(())
    }

    fn new(_: u64) -> Self {
        MountainBiome {}
    }
}
