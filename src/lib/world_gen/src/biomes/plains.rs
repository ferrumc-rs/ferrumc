use crate::chunk_builder::ChunkBuilder;
use crate::errors::WorldGenError;
use crate::{BiomeGenerator, NoiseGenerator};
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::{ChunkColumnPos, ChunkPos};
use ferrumc_world::structure::FerrumcChunk;

/// Plains biome generator.
///
/// Generates rolling grassland terrain with noise-based height variation.
pub(crate) struct PlainsBiome;

impl BiomeGenerator for PlainsBiome {
    fn _biome_id(&self) -> u8 {
        0
    }

    fn _biome_name(&self) -> String {
        "plains".to_string()
    }

    fn generate_chunk(
        &self,
        pos: ChunkPos,
        noise: &NoiseGenerator,
    ) -> Result<FerrumcChunk, WorldGenError> {
        // Create a chunk builder for overworld dimensions
        let mut builder = ChunkBuilder::new(pos.x(), pos.z(), -64, 384);

        // Get block state IDs once (the block! macro returns BlockStateId)
        let stone = block!("stone").raw();
        let water = block!("water", {level: 0}).raw();
        let sand = block!("sand").raw();
        let grass = block!("grass_block", {snowy: false}).raw();

        // Pre-calculate heights for all columns
        let mut heights = Vec::with_capacity(256);
        for chunk_x in 0..16u8 {
            for chunk_z in 0..16u8 {
                let curr_pos = pos.column_pos(ChunkColumnPos::new(chunk_x, chunk_z));
                let global_x = curr_pos.x();
                let global_z = curr_pos.z();
                let height = noise.get_noise(f64::from(global_x), f64::from(global_z));
                let height = (height * 64.0) as i32 + 64;
                heights.push((chunk_x, chunk_z, height));
            }
        }

        // Find the minimum height for efficient section filling
        let y_min = heights.iter().map(|(_, _, h)| *h).min().unwrap_or(0);
        let highest_full_section = y_min / 16;

        // Fill sections below sea level with water first (sea level is 64)
        for section_y in -4..4i8 {
            builder.fill_section(section_y, water);
        }

        // Fill sections that are completely underground with stone
        for section_y in -4..highest_full_section as i8 {
            builder.fill_section(section_y, stone);
        }

        // Generate terrain above the fully-filled sections
        let above_filled_sections = (highest_full_section * 16) - 1;
        for (chunk_x, chunk_z, height) in heights {
            // Fill from bedrock to height
            for y in (above_filled_sections + 1)..=height {
                let block_id = if y <= 64 {
                    // Below or at sea level, use sand (beach/ocean floor)
                    sand
                } else {
                    // Above sea level, use grass
                    grass
                };
                builder.set_block(chunk_x, y, chunk_z, block_id);
            }
        }

        Ok(builder.build())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_ok() {
        let generator = PlainsBiome {};
        let noise = NoiseGenerator::new(0);
        assert!(
            generator
                .generate_chunk(ChunkPos::new(0, 0), &noise)
                .is_ok()
        );
    }

    #[test]
    fn test_chunk_structure() {
        let generator = PlainsBiome {};
        let noise = NoiseGenerator::new(0);
        let chunk = generator
            .generate_chunk(ChunkPos::new(0, 0), &noise)
            .unwrap();

        // Verify chunk metadata
        assert_eq!(chunk.x, 0);
        assert_eq!(chunk.z, 0);
        assert_eq!(chunk.min_y, -64);
        assert_eq!(chunk.height, 384);
        assert_eq!(chunk.sections.len(), 24);
    }

    #[test]
    fn test_random_chunk_generation() {
        let generator = PlainsBiome {};
        let noise = NoiseGenerator::new(0);
        for _ in 0..100 {
            let x = rand::random::<i32>() & ((1 << 22) - 1);
            let z = rand::random::<i32>() & ((1 << 22) - 1);
            assert!(
                generator
                    .generate_chunk(ChunkPos::new(x, z), &noise)
                    .is_ok()
            );
        }
    }

    #[test]
    fn test_very_high_coordinates() {
        let generator = PlainsBiome {};
        let noise = NoiseGenerator::new(0);
        assert!(
            generator
                .generate_chunk(ChunkPos::new((1 << 22) - 1, (1 << 22) - 1), &noise)
                .is_ok()
        );
        assert!(
            generator
                .generate_chunk(ChunkPos::new(-((1 << 22) - 1), -((1 << 22) - 1)), &noise)
                .is_ok()
        );
    }

    #[test]
    fn test_random_seeds() {
        for _ in 0..100 {
            let generator = PlainsBiome {};
            let seed = rand::random::<u64>();
            let noise = NoiseGenerator::new(seed);
            assert!(
                generator
                    .generate_chunk(ChunkPos::new(0, 0), &noise)
                    .is_ok()
            );
        }
    }
}
