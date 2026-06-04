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

/// One column's placement context: in-chunk coordinates `(x, z)`, the surface height `top_y`, and the
/// global coordinates `(gx, gz)` that seed the per-column rolls. Bundled so the per-biome placers take
/// a single argument instead of threading five coordinates each.
struct Column {
    x: u8,
    z: u8,
    top_y: i16,
    gx: i32,
    gz: i32,
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

                let col = Column {
                    x,
                    z,
                    top_y,
                    gx: base_x + i32::from(x),
                    gz: base_z + i32::from(z),
                };

                // Grassland (plains/forest): a mix of short grass, ferns, flowers, and the
                // occasional two-block plant.
                if matches!(biome, 40 | 21) && match_block!("grass_block", ground) {
                    self.place_grassland(chunk, &col, biome);
                    continue;
                }

                // Desert: sparse cacti (1–2 tall) and dead bushes on sand.
                if biome == 14 && match_block!("sand", ground) {
                    self.place_desert(chunk, &col);
                }
            }
        }
    }

    /// Places grassland ground cover at `(x, z)` whose surface is `top_y`. Cumulative probability
    /// bands pick the plant; two-block plants are only placed where there is headroom.
    fn place_grassland(&self, chunk: &mut Chunk, col: &Column, biome: u8) {
        let Column {
            x,
            z,
            top_y,
            gx,
            gz,
        } = *col;
        const SALT: u64 = 0x5f3a_91c7;
        let forest = biome == 21;
        let r = self.roll(gx, gz, SALT);

        // Rare two-block plants first (tall flowers, then tall grass / large fern), then one-block
        // flowers, ferns (forest), and short grass; the rest of the ground stays bare.
        if r < 0.015 {
            let (lower, upper) = self.pick_tall_flower(gx, gz);
            if Self::place_double(chunk, x, z, top_y, lower, upper) {
                return;
            }
        }
        if r < 0.05 {
            let (lower, upper) = if forest && (self.hash(gx, gz, SALT) & 1 == 0) {
                (
                    block!("large_fern", { half: "lower" }),
                    block!("large_fern", { half: "upper" }),
                )
            } else {
                (
                    block!("tall_grass", { half: "lower" }),
                    block!("tall_grass", { half: "upper" }),
                )
            };
            if Self::place_double(chunk, x, z, top_y, lower, upper) {
                return;
            }
        }

        let plant = if r < 0.09 {
            self.pick_flower(gx, gz)
        } else if forest && r < 0.18 {
            block!("fern")
        } else if r < 0.34 {
            block!("short_grass")
        } else {
            return;
        };
        chunk.set_block(ChunkBlockPos::new(x, top_y + 1, z), plant);
    }

    /// Places sparse desert plants. Cacti follow the vanilla rule that no block may be horizontally
    /// adjacent to any part of the column.
    fn place_desert(&self, chunk: &mut Chunk, col: &Column) {
        let Column {
            x,
            z,
            top_y,
            gx,
            gz,
        } = *col;
        const SALT: u64 = 0x2c9b_71e4;
        const SALT_HEIGHT: u64 = 0x84d1_55af;
        let r = self.roll(gx, gz, SALT);

        if r < 0.012 {
            let height = 1 + (self.hash(gx, gz, SALT_HEIGHT) % 2) as i16;
            let cactus = block!("cactus", { age: 0 });
            for dy in 1..=height {
                let y = top_y + dy;
                // Cactus cannot have any horizontally adjacent block; bail (leaving a shorter
                // cactus, or none) the moment a level is blocked or occupied.
                if !match_block!("air", chunk.get_block(ChunkBlockPos::new(x, y, z)))
                    || !Self::neighbours_clear(chunk, x, z, y)
                {
                    break;
                }
                chunk.set_block(ChunkBlockPos::new(x, y, z), cactus);
            }
        } else if r < 0.03 {
            // Keep dead bushes off cactus sides too, so a cactus placed earlier in the scan never
            // ends up with an adjacent block.
            let y = top_y + 1;
            if !Self::has_cactus_neighbour(chunk, x, z, y) {
                chunk.set_block(ChunkBlockPos::new(x, y, z), block!("dead_bush"));
            }
        }
    }

    /// Whether any in-chunk horizontal neighbour of `(x, z)` at height `y` is a cactus.
    fn has_cactus_neighbour(chunk: &Chunk, x: u8, z: u8, y: i16) -> bool {
        for (dx, dz) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
            let nx = i32::from(x) + dx;
            let nz = i32::from(z) + dz;
            if (0..16).contains(&nx)
                && (0..16).contains(&nz)
                && match_block!(
                    "cactus",
                    chunk.get_block(ChunkBlockPos::new(nx as u8, y, nz as u8))
                )
            {
                return true;
            }
        }
        false
    }

    /// Places a two-block plant (`lower` at `top_y + 1`, `upper` above it) if both cells are air.
    /// Returns whether it was placed.
    fn place_double(
        chunk: &mut Chunk,
        x: u8,
        z: u8,
        top_y: i16,
        lower: BlockStateId,
        upper: BlockStateId,
    ) -> bool {
        let lo = ChunkBlockPos::new(x, top_y + 1, z);
        let hi = ChunkBlockPos::new(x, top_y + 2, z);
        if match_block!("air", chunk.get_block(lo)) && match_block!("air", chunk.get_block(hi)) {
            chunk.set_block(lo, lower);
            chunk.set_block(hi, upper);
            true
        } else {
            false
        }
    }

    /// Whether all four horizontal neighbours of `(x, z)` at height `y` are inside this chunk and
    /// air. Edge columns (a neighbour in another chunk) count as not clear, so a cactus is never
    /// placed where its clearance cannot be verified — a conservative, deterministic choice.
    fn neighbours_clear(chunk: &Chunk, x: u8, z: u8, y: i16) -> bool {
        for (dx, dz) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
            let nx = i32::from(x) + dx;
            let nz = i32::from(z) + dz;
            if !(0..16).contains(&nx) || !(0..16).contains(&nz) {
                return false;
            }
            let b = chunk.get_block(ChunkBlockPos::new(nx as u8, y, nz as u8));
            if !match_block!("air", b) {
                return false;
            }
        }
        true
    }

    /// Picks one of the one-block flower set for the column.
    fn pick_flower(&self, gx: i32, gz: i32) -> BlockStateId {
        const SALT_FLOWER: u64 = 0xa17e_03d5;
        match self.hash(gx, gz, SALT_FLOWER) % 11 {
            0 => block!("dandelion"),
            1 => block!("poppy"),
            2 => block!("cornflower"),
            3 => block!("oxeye_daisy"),
            4 => block!("allium"),
            5 => block!("azure_bluet"),
            6 => block!("red_tulip"),
            7 => block!("orange_tulip"),
            8 => block!("white_tulip"),
            9 => block!("pink_tulip"),
            _ => block!("lily_of_the_valley"),
        }
    }

    /// Picks one of the two-block tall flowers (lower, upper halves) for the column.
    fn pick_tall_flower(&self, gx: i32, gz: i32) -> (BlockStateId, BlockStateId) {
        const SALT_TALL: u64 = 0x6b2d_44f1;
        match self.hash(gx, gz, SALT_TALL) % 3 {
            0 => (
                block!("lilac", { half: "lower" }),
                block!("lilac", { half: "upper" }),
            ),
            1 => (
                block!("rose_bush", { half: "lower" }),
                block!("rose_bush", { half: "upper" }),
            ),
            _ => (
                block!("peony", { half: "lower" }),
                block!("peony", { half: "upper" }),
            ),
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
