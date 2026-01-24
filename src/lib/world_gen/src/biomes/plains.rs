use crate::errors::WorldGenError;
use crate::interp::{bilerp, dither_field, smoothstep};
use crate::{BiomeGenerator, NoiseGenerator};
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::{BlockPos, ChunkHeight, ChunkPos};
use std::hash::Hasher;

fn build_heightmap_interpolated(pos: ChunkPos, noise: &NoiseGenerator) -> [i32; 16 * 16] {
    const STEP_XZ: i32 = 4;

    let gx = (16 / STEP_XZ + 1) as usize; // 5
    let gz = (16 / STEP_XZ + 1) as usize; // 5

    let idx = |ix: usize, iz: usize| -> usize { iz * gx + ix };

    // sample coarse grid
    let mut grid = vec![0.0f64; gx * gz];

    for ix in 0..gx {
        for iz in 0..gz {
            let lx = (ix as i32) * STEP_XZ;
            let lz = (iz as i32) * STEP_XZ;

            let world_x = pos.x() * 16 + lx;
            let world_z = pos.z() * 16 + lz;

            // same function you use now, just fewer calls
            grid[idx(ix, iz)] = noise.get_noise(f64::from(world_x), f64::from(world_z));
        }
    }

    // interpolate to full 16x16 heightmap
    let mut out = [0i32; 16 * 16];

    for x in 0..16i32 {
        for z in 0..16i32 {
            let base_ix = (x / STEP_XZ) as usize;
            let base_iz = (z / STEP_XZ) as usize;

            let tx = smoothstep(f64::from(x % STEP_XZ) / f64::from(STEP_XZ));
            let tz = smoothstep(f64::from(z % STEP_XZ) / f64::from(STEP_XZ));

            let ix0 = base_ix;
            let ix1 = (base_ix + 1).min(gx - 1);
            let iz0 = base_iz;
            let iz1 = (base_iz + 1).min(gz - 1);

            let c00 = grid[idx(ix0, iz0)];
            let c10 = grid[idx(ix1, iz0)];
            let c01 = grid[idx(ix0, iz1)];
            let c11 = grid[idx(ix1, iz1)];

            let h = bilerp(c00, c10, c01, c11, tx, tz);

            let height = (h * 64.0) as i32 + 64;
            out[(z as usize) * 16 + (x as usize)] = height;
        }
    }

    out
}

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
        let mut chunk = Chunk::new_empty_with_height(ChunkHeight::new(-64, 384));
        let stone = block!("stone");

        // Fill with water first
        for section_y in -4..4 {
            chunk.fill_section(section_y as i8, block!("water", {level: 0}));
        }

        // Build heightmap
        let heights = build_heightmap_interpolated(pos, noise);

        // Find minimum height to fill full stone sections fast
        let mut y_min = i32::MAX;
        for &h in heights.iter() {
            y_min = y_min.min(h);
        }

        let highest_full_section = y_min.div_euclid(16);
        for section_y in -4..highest_full_section {
            chunk.fill_section(section_y as i8, stone);
        }

        let above_filled_sections = (highest_full_section * 16) - 1;

        // Now fill columns above filled stone
        for chunk_x in 0..16i32 {
            for chunk_z in 0..16i32 {
                let height = heights[(chunk_z as usize) * 16 + (chunk_x as usize)];

                if height > above_filled_sections {
                    let fill = height - above_filled_sections;
                    let global_x = pos.x() * 16 + chunk_x;
                    let global_z = pos.z() * 16 + chunk_z;

                    let d = dither_field(noise.seed, global_x, global_z, 16); // 0..1
                    let wobble = ((d * 2.0) - 1.0) * 2.0; // [-4,+4] blocks

                    for dy in 0..fill {
                        let y = above_filled_sections + dy;
                        let dithered_y = y + wobble.round() as i32;
                        if dithered_y <= 64 {
                            chunk.set_block(
                                BlockPos::of(global_x, y, global_z).chunk_block_pos(),
                                block!("sand"),
                            );
                        } else if dithered_y >= 80 {
                            chunk.set_block(
                                BlockPos::of(global_x, y, global_z).chunk_block_pos(),
                                stone,
                            );
                        } else {
                            chunk.set_block(
                                BlockPos::of(global_x, y, global_z).chunk_block_pos(),
                                block!("grass_block", {snowy: false}),
                            );
                        }
                    }
                }
            }
        }

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
