//! Ocean biome: a sand + sandstone bed, flooded with water up to the world water level.

use crate::errors::WorldGenError;
use crate::terrain_noise::NoiseGenerator;
use crate::{BASELINE_HEIGHT, BiomeGenerator};
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;
use rand::Rng;
use rand::SeedableRng;

pub(crate) struct OceanBiome {
    sand_depth_noise: NoiseGenerator,
    sand_height_offset_noise: NoiseGenerator,
    world_water_level: i16,
}

impl BiomeGenerator for OceanBiome {
    fn biome_id(&self) -> u8 {
        35 // minecraft:ocean
    }

    fn _biome_name(&self) -> String {
        "ocean".to_string()
    }

    fn decorate(
        &self,
        chunk: &mut Chunk,
        x: u8,
        z: u8,
        surface_y: i16,
        _chunk_x: i32,
        _chunk_z: i32,
    ) -> Result<(), WorldGenError> {
        // Sand bed at and just below the surface.
        let sand_depth =
            ((self.sand_depth_noise.get(f32::from(x), f32::from(z)) * 3.0) + 3.0) as i16;
        for i in 0..=sand_depth {
            chunk.set_block(ChunkBlockPos::new(x, surface_y - i, z), block!("sand"));
        }

        // Sandstone below the sand.
        let sand_stone_depth = (self
            .sand_height_offset_noise
            .get(f32::from(x), f32::from(z))
            * 2.0) as i16
            + 5;
        for i in 1..=sand_stone_depth {
            chunk.set_block(
                ChunkBlockPos::new(x, surface_y - sand_depth - i, z),
                block!("sandstone"),
            );
        }

        // Flood with water from just above the surface up to the water level.
        for y in (surface_y + 1)..=self.world_water_level {
            chunk.set_block(ChunkBlockPos::new(x, y, z), block!("water", { level: 0 }));
        }
        Ok(())
    }

    fn new(seed: u64) -> Self {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        OceanBiome {
            sand_depth_noise: NoiseGenerator::new(seed, 0.1, 4, None),
            sand_height_offset_noise: NoiseGenerator::new(seed + 1, 0.1, 4, None),
            // A water level a fixed distance below the baseline, jittered slightly per world.
            world_water_level: BASELINE_HEIGHT - rng.gen_range(38..=42) as i16,
        }
    }
}
