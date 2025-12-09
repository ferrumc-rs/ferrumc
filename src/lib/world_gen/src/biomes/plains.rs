use crate::errors::WorldGenError;
use crate::{BiomeGenerator, NoiseGenerator};
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::edit_batch::EditBatch;
use ferrumc_world::pos::{BlockPos, ChunkColumnPos, ChunkHeight, ChunkPos};

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
    ) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new(ChunkHeight::new(-64, 384));
        let mut heights = vec![];
        let stone = block!("stone"); // just to test the macro

        // Fill with water first
        for section_y in -4..4 {
            chunk.set_section(section_y as i8, block!("water", {level: 0}))?;
        }

        // Then generate some heights
        for chunk_x in 0..16 {
            for chunk_z in 0..16 {
                let curr_pos = pos.column_pos(ChunkColumnPos::new(chunk_x, chunk_z));
                let global_x = curr_pos.x();
                let global_z = curr_pos.z();
                let height = noise.get_noise(f64::from(global_x), f64::from(global_z));
                let height = (height * 64.0) as i32 + 64;
                heights.push((global_x, global_z, height));
            }
        }

        // Fill in the sections that consist of only stone first with the set_section method since
        // it's faster than set_block
        let y_min = heights.iter().min_by(|a, b| a.2.cmp(&b.2)).unwrap().2;
        let highest_full_section = y_min / 16;
        for section_y in -4..highest_full_section {
            chunk.set_section(section_y as i8, stone)?;
        }
        let mut batch = EditBatch::new(&mut chunk);
        let above_filled_sections = (highest_full_section * 16) - 1;
        for (global_x, global_z, height) in heights {
            if height > above_filled_sections {
                let height = height - above_filled_sections;
                for y in 0..height {
                    if y + above_filled_sections <= 64 {
                        batch.set_block(
                            BlockPos::of(global_x, y + above_filled_sections, global_z)
                                .chunk_block_pos(),
                            block!("sand"),
                        );
                    } else {
                        batch.set_block(
                            BlockPos::of(global_x, y + above_filled_sections, global_z)
                                .chunk_block_pos(),
                            block!("grass_block", {snowy: false}),
                        );
                    }
                }
            }
        }

        batch.apply()?;

        Ok(chunk)
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
