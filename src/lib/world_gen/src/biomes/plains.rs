//! Plains biome: a grass surface over a few blocks of dirt.

use crate::BiomeGenerator;
use crate::errors::WorldGenError;
use crate::terrain_noise::NoiseGenerator;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;

pub(crate) struct PlainsBiome {
    dirt_depth_noise: NoiseGenerator,
}

impl BiomeGenerator for PlainsBiome {
    fn biome_id(&self) -> u8 {
        40 // minecraft:plains
    }

    fn _biome_name(&self) -> String {
        "plains".to_string()
    }

    fn decorate(
        &self,
        chunk: &mut Chunk,
        x: u8,
        z: u8,
        surface_y: i16,
    ) -> Result<(), WorldGenError> {
        // Grass on the surface.
        chunk.set_block(
            ChunkBlockPos::new(x, surface_y, z),
            block!("grass_block", { snowy: false }),
        );

        // A noise-varied band of dirt below it.
        let dirt_depth = (self.dirt_depth_noise.get(f32::from(x), f32::from(z)) * 5.0) + 3.0;
        for i in 1..=dirt_depth as i16 {
            chunk.set_block(ChunkBlockPos::new(x, surface_y - i, z), block!("dirt"));
        }
        Ok(())
    }

    fn new(seed: u64) -> Self {
        PlainsBiome {
            dirt_depth_noise: NoiseGenerator::new(seed, 0.1, 4, None),
        }
    }
}
