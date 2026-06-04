//! Desert biome: sand over sandstone, treeless.

use crate::BiomeGenerator;
use crate::errors::WorldGenError;
use crate::terrain_noise::NoiseGenerator;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;

pub(crate) struct DesertBiome {
    sand_depth_noise: NoiseGenerator,
}

impl BiomeGenerator for DesertBiome {
    fn biome_id(&self) -> u8 {
        14 // minecraft:desert
    }

    fn _biome_name(&self) -> String {
        "desert".to_string()
    }

    fn decorate(
        &self,
        chunk: &mut Chunk,
        x: u8,
        z: u8,
        surface_y: i16,
    ) -> Result<(), WorldGenError> {
        // A few blocks of sand over sandstone the rest of the way down through the soil layer.
        let sand_depth =
            ((self.sand_depth_noise.get(f32::from(x), f32::from(z)) * 3.0) + 3.0) as i16;
        for i in 0..=sand_depth {
            chunk.set_block(ChunkBlockPos::new(x, surface_y - i, z), block!("sand"));
        }
        for i in 1..=5 {
            chunk.set_block(
                ChunkBlockPos::new(x, surface_y - sand_depth - i, z),
                block!("sandstone"),
            );
        }
        Ok(())
    }

    fn new(seed: u64) -> Self {
        DesertBiome {
            sand_depth_noise: NoiseGenerator::new(seed, 0.1, 4, None),
        }
    }
}
