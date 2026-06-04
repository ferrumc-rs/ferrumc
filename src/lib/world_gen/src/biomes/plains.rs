//! Plains biome: grass over dirt, with sparse oak trees.

use crate::BiomeGenerator;
use crate::biomes::trees::place_oak_tree;
use crate::errors::WorldGenError;
use crate::terrain_noise::NoiseGenerator;
use ferrumc_macros::{block, match_block};
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;

// ── Tuning constants ──────────────────────────────────────────────────────────

/// Minimum block distance between any two tree trunks. The local-maximum check over this radius
/// guarantees that no two trees are placed within this many blocks of each other.
const MIN_TREE_RADIUS: i32 = 4;

/// Fraction of local-maximum candidates that actually grow a tree (0 = none, 1 = all maxima).
/// Combined with `MIN_TREE_RADIUS` this controls average inter-tree spacing in dense zones.
/// `tree_density_noise` then carves out open clearings between those dense zones.
const TREE_CANDIDATE_THRESHOLD: f32 = 0.70;

/// Minimum surface height for a tree to spawn. Columns below this are close to water and should
/// remain treeless (they are often on beach/ocean transitions even inside the plains biome).
const TREE_MIN_SURFACE_Y: i16 = 55;

// ─────────────────────────────────────────────────────────────────────────────

pub(crate) struct PlainsBiome {
    seed: u64,
    dirt_depth_noise: NoiseGenerator,
    /// Broad low-frequency noise that controls where dense tree groves vs open fields appear.
    /// High values → trees grow here (if also a local hash maximum); low values → clearing.
    tree_density_noise: NoiseGenerator,
}

impl PlainsBiome {
    /// Deterministic per-position hash. Mixes `seed`, `gx`, and `gz` with a pair of
    /// multiplicative constants derived from the fractional part of the golden ratio and
    /// the Weyl sequence, giving good avalanche behaviour without requiring the `rand` crate.
    fn position_hash(&self, gx: i32, gz: i32) -> u64 {
        let mut h = self.seed;
        h ^= (gx as u64).wrapping_mul(0x517c_c1b7_2722_0a95);
        h ^= (gz as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
        h ^= h >> 33;
        h
    }

    /// Maps `position_hash` to a normalised score in `[0, 1]`.
    fn tree_score(&self, gx: i32, gz: i32) -> f32 {
        (self.position_hash(gx, gz) >> 32) as f32 / u32::MAX as f32
    }

    /// Returns `true` if column `(gx, gz)` should grow a tree.
    ///
    /// Two-stage filter:
    /// 1. **Local hash maximum** — ensures at least `MIN_TREE_RADIUS` blocks between trunks.
    /// 2. **Density noise threshold** — carves the world into grove patches and open clearings.
    fn should_place_tree(&self, gx: i32, gz: i32, surface_y: i16) -> bool {
        if surface_y < TREE_MIN_SURFACE_Y {
            return false;
        }

        let score = self.tree_score(gx, gz);

        // Stage 1: local maximum within a square of side 2*R+1.
        // Any neighbour with a strictly higher score disqualifies this column.
        let r = MIN_TREE_RADIUS;
        for dx in -r..=r {
            for dz in -r..=r {
                if dx == 0 && dz == 0 {
                    continue;
                }
                if self.tree_score(gx + dx, gz + dz) > score {
                    return false;
                }
            }
        }

        // Stage 2: density noise controls grove vs clearing.
        let density = self.tree_density_noise.get(gx as f32, gz as f32);
        density >= TREE_CANDIDATE_THRESHOLD
    }

    /// Trunk height for the tree at `(gx, gz)`: 4, 5, or 6 blocks.
    fn trunk_height(&self, gx: i32, gz: i32) -> u8 {
        // Mix a different salt into the hash to decorrelate height from placement score.
        let h = self.position_hash(gx ^ 0xdead, gz ^ 0xbeef);
        4 + (h % 3) as u8
    }
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
        chunk_x: i32,
        chunk_z: i32,
    ) -> Result<(), WorldGenError> {
        // ── Surface blocks ────────────────────────────────────────────────────
        chunk.set_block(
            ChunkBlockPos::new(x, surface_y, z),
            block!("grass_block", { snowy: false }),
        );

        let dirt_depth = (self.dirt_depth_noise.get(f32::from(x), f32::from(z)) * 5.0) + 3.0;
        for i in 1..=dirt_depth as i16 {
            chunk.set_block(ChunkBlockPos::new(x, surface_y - i, z), block!("dirt"));
        }

        // ── Tree placement ────────────────────────────────────────────────────
        let gx = chunk_x * 16 + i32::from(x);
        let gz = chunk_z * 16 + i32::from(z);

        if self.should_place_tree(gx, gz, surface_y) {
            let height = self.trunk_height(gx, gz);

            // Verify the trunk column is unobstructed. At decoration time everything above
            // surface_y is already air, so this mainly guards against two trees overlapping
            // (e.g. leaves from an earlier tree blocking a later trunk) or future re-ordering.
            let trunk_clear = (1..=(i16::from(height) + 2)).all(|dy| {
                match_block!(
                    "air",
                    chunk.get_block(ChunkBlockPos::new(x, surface_y + dy, z))
                )
            });

            if trunk_clear {
                place_oak_tree(chunk, x, z, surface_y, height);
            }
        }

        Ok(())
    }

    fn new(seed: u64) -> Self {
        PlainsBiome {
            seed,
            dirt_depth_noise: NoiseGenerator::new(seed, 0.1, 4, None),
            // Low frequency → large-scale groves / clearings (~80-block patches).
            // Three octaves add some medium-scale variation within each grove.
            tree_density_noise: NoiseGenerator::new(seed.wrapping_add(7), 0.012, 3, None),
        }
    }
}
