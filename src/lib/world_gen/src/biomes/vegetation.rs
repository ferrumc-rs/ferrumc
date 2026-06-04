//! Ground vegetation: short grass, ferns, flowers, and desert plants scattered on top of the
//! finished surface.
//!
//! Vegetation is placed in its own pass after trees (see [`crate::WorldGenerator::generate_chunk`])
//! so it lands on the real, post-cave surface and never inside a trunk or a cave. Every plant is a
//! single column (cacti stack vertically but stay in one column), so — unlike tree canopies — there
//! is no cross-chunk overhang and no overscan is needed. Placement is a pure function of the world
//! seed and the global column, so it is stable and reproducible.

use ferrumc_macros::{block, match_block};
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;

use crate::{MAX_GENERATED_HEIGHT, MIN_WORLD_Y};

/// Scatters ground vegetation. Holds only the world seed; all randomness is derived deterministically
/// per column so the same plant appears at a column regardless of which chunk places it.
pub(crate) struct Vegetation {
    seed: u64,
}

impl Vegetation {
    pub(crate) fn new(seed: u64) -> Self {
        Vegetation { seed }
    }

    /// Deterministic per-position hash, salted so independent decisions (placement vs. which plant
    /// vs. cactus height) do not correlate. Same mix as the tree placer's hash.
    fn hash(&self, gx: i32, gz: i32, salt: u64) -> u64 {
        let mut h = self.seed ^ salt;
        h ^= (gx as u64).wrapping_mul(0x517c_c1b7_2722_0a95);
        h ^= (gz as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
        h ^= h >> 33;
        h
    }

    /// A deterministic roll in `[0, 1)` for column `(gx, gz)` under `salt`.
    fn roll(&self, gx: i32, gz: i32, salt: u64) -> f32 {
        (self.hash(gx, gz, salt) >> 40) as f32 / (1u64 << 24) as f32
    }

    /// Places vegetation across the chunk. `col_biome_ids[x][z]` is the biome of each column;
    /// `base_x`/`base_z` are the chunk's world-space origin.
    pub(crate) fn decorate(
        &self,
        chunk: &mut Chunk,
        base_x: i32,
        base_z: i32,
        col_biome_ids: &[[u8; 16]; 16],
    ) {
        for x in 0..16u8 {
            for z in 0..16u8 {
                let biome = col_biome_ids[usize::from(x)][usize::from(z)];
                // Vegetation only grows on grassland and desert; skip everything else early.
                if !matches!(biome, 40 | 21 | 14) {
                    continue;
                }

                // Find the surface (topmost solid, non-air, non-water) block and the air above it.
                let Some(top_y) = Self::surface_y(chunk, x, z) else {
                    continue;
                };
                let above = ChunkBlockPos::new(x, top_y + 1, z);
                if !match_block!("air", chunk.get_block(above)) {
                    continue; // occupied (e.g. a tree trunk or a snow layer)
                }
                let ground = chunk.get_block(ChunkBlockPos::new(x, top_y, z));

                let gx = base_x + i32::from(x);
                let gz = base_z + i32::from(z);

                // Grassland (plains/forest): short grass, ferns (forest only), and a few flowers.
                if matches!(biome, 40 | 21) && match_block!("grass_block", ground) {
                    const SALT: u64 = 0x5f3a_91c7;
                    let r = self.roll(gx, gz, SALT);
                    // ~3% flowers, then ferns (forest only), then short grass; the rest stays bare.
                    let plant = if r < 0.03 {
                        self.pick_flower(gx, gz)
                    } else if biome == 21 && r < 0.10 {
                        block!("fern")
                    } else if r < 0.28 {
                        block!("short_grass")
                    } else {
                        continue;
                    };
                    chunk.set_block(ChunkBlockPos::new(x, top_y + 1, z), plant);
                    continue;
                }

                // Desert: sparse cacti (1–2 tall) and dead bushes on sand.
                if biome == 14 && match_block!("sand", ground) {
                    const SALT: u64 = 0x2c9b_71e4;
                    const SALT_HEIGHT: u64 = 0x84d1_55af;
                    let r = self.roll(gx, gz, SALT);
                    if r < 0.012 {
                        let height = 1 + (self.hash(gx, gz, SALT_HEIGHT) % 2) as i16;
                        let cactus = block!("cactus", { age: 0 });
                        for dy in 1..=height {
                            let pos = ChunkBlockPos::new(x, top_y + dy, z);
                            if !match_block!("air", chunk.get_block(pos)) {
                                break;
                            }
                            chunk.set_block(pos, cactus);
                        }
                    } else if r < 0.03 {
                        chunk.set_block(ChunkBlockPos::new(x, top_y + 1, z), block!("dead_bush"));
                    }
                }
            }
        }
    }

    /// Picks one of a small flower set for the column.
    fn pick_flower(&self, gx: i32, gz: i32) -> BlockStateId {
        const SALT_FLOWER: u64 = 0xa17e_03d5;
        match self.hash(gx, gz, SALT_FLOWER) % 4 {
            0 => block!("dandelion"),
            1 => block!("poppy"),
            2 => block!("cornflower"),
            _ => block!("oxeye_daisy"),
        }
    }

    /// Topmost solid (non-air, non-water) block Y in a column, or `None` if the column is empty.
    fn surface_y(chunk: &Chunk, x: u8, z: u8) -> Option<i16> {
        (MIN_WORLD_Y..=MAX_GENERATED_HEIGHT).rev().find(|&y| {
            let b = chunk.get_block(ChunkBlockPos::new(x, y, z));
            !match_block!("air", b) && !match_block!("water", b)
        })
    }
}
