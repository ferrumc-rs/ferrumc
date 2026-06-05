//! Ocean biome: a sand + sandstone sea bed. The water itself is no longer placed here — a single
//! global pass floods every column below [`crate::climate::SEA_LEVEL`] during chunk generation
//! (see [`crate::WorldGenerator::generate_chunk`]), so the ocean decorator only lays the floor.
//!
//! A single decorator backs every ocean variant (ocean, deep ocean, cold/lukewarm/warm/frozen and
//! their deep forms): the sea bed is identical, and only the recorded registry biome ID differs.
//! That ID is chosen by the selection layer from temperature and depth
//! (`WorldGenerator::ocean_variant_id`); the decorator's own [`OceanBiome::biome_id`] is just a
//! placeholder default.

use crate::BiomeGenerator;
use crate::errors::WorldGenError;
use crate::terrain_noise::NoiseGenerator;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;

pub(crate) struct OceanBiome {
    sand_depth_noise: NoiseGenerator,
    sand_height_offset_noise: NoiseGenerator,
}

impl BiomeGenerator for OceanBiome {
    fn biome_id(&self) -> u8 {
        35 // minecraft:ocean — placeholder; the actual variant ID comes from ocean_variant_id.
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
        Ok(())
    }

    fn new(seed: u64) -> Self {
        OceanBiome {
            sand_depth_noise: NoiseGenerator::new(seed, 0.1, 4, None),
            sand_height_offset_noise: NoiseGenerator::new(seed.wrapping_add(1), 0.1, 4, None),
        }
    }
}
