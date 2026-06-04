//! Ocean biome: a sand + sandstone sea bed. The water itself is no longer placed here — a single
//! global pass floods every column below [`crate::climate::SEA_LEVEL`] during chunk generation
//! (see [`crate::WorldGenerator::generate_chunk`]), so the ocean decorator only lays the floor.
//!
//! The same decorator backs both `ocean` and `deep_ocean`; the selection layer
//! ([`crate::WorldGenerator::get_biome`]) builds the variant with the appropriate biome ID.

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
    /// Registry biome ID — `35` (ocean) or `13` (deep_ocean).
    id: u8,
}

impl OceanBiome {
    /// Builds an ocean variant with the given registry biome ID. Used by the selection layer to
    /// distinguish shallow `ocean` from `deep_ocean` while sharing the floor-laying logic.
    pub(crate) fn with_id(seed: u64, id: u8) -> Self {
        OceanBiome {
            sand_depth_noise: NoiseGenerator::new(seed, 0.1, 4, None),
            sand_height_offset_noise: NoiseGenerator::new(seed.wrapping_add(1), 0.1, 4, None),
            id,
        }
    }
}

impl BiomeGenerator for OceanBiome {
    fn biome_id(&self) -> u8 {
        self.id
    }

    fn _biome_name(&self) -> String {
        if self.id == 13 {
            "deep_ocean".to_string()
        } else {
            "ocean".to_string()
        }
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
        OceanBiome::with_id(seed, 35)
    }
}
