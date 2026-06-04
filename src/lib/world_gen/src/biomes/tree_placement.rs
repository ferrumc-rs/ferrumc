//! Shared tree-placement logic.
//!
//! Several biomes scatter trees the same way — a deterministic per-column hash gated by a
//! low-frequency density field — differing only in spacing, density, and the tree kind. This module
//! factors that out so each biome supplies its own tuning and produces the same seamless,
//! seed-deterministic placement (a prerequisite for the cross-chunk canopy overscan, which resolves
//! a neighbour's trees identically to how the neighbour does).

use crate::terrain_noise::NoiseGenerator;

/// Decides where trees grow within a biome and how tall they are. Holds the per-biome tuning and a
/// density-noise sampler; placement is a pure function of the world seed and global column.
pub(crate) struct TreePlacer {
    seed: u64,
    /// Broad low-frequency noise that carves the world into dense groves vs open clearings: high
    /// values → trees grow here (if also a local hash maximum), low values → clearing.
    density_noise: NoiseGenerator,
    /// Minimum block distance between any two trunks (local-maximum radius).
    min_radius: i32,
    /// Density-noise value above which a column is a candidate (0 = trees everywhere, 1 = nowhere).
    /// Lower → denser groves.
    density_threshold: f32,
    /// Minimum surface height for a tree; columns below this (near water) stay treeless.
    min_surface_y: i16,
}

impl TreePlacer {
    /// Builds a placer. `density_freq` sets the grove/clearing scale (~1/freq blocks); a different
    /// `seed` salt per biome decorrelates their grove patterns.
    pub(crate) fn new(
        seed: u64,
        density_freq: f64,
        min_radius: i32,
        density_threshold: f32,
        min_surface_y: i16,
    ) -> Self {
        Self {
            seed,
            density_noise: NoiseGenerator::new(seed, density_freq, 3, None),
            min_radius,
            density_threshold,
            min_surface_y,
        }
    }

    /// Deterministic per-position hash. Mixes `seed`, `gx`, and `gz` with multiplicative constants
    /// derived from the fractional part of the golden ratio and the Weyl sequence, giving good
    /// avalanche behaviour without requiring the `rand` crate.
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
    /// Two-stage filter (both must pass; the result is independent of their order):
    /// 1. **Density noise threshold** — carves the world into grove patches and open clearings.
    /// 2. **Local hash maximum** — ensures at least `min_radius` blocks between trunks.
    ///
    /// The cheap single-sample density check runs first so the `O((2R+1)^2)` local-maximum scan is
    /// only performed in grove regions (the minority of columns), not for every column.
    pub(crate) fn should_place_tree(&self, gx: i32, gz: i32, surface_y: i16) -> bool {
        if surface_y < self.min_surface_y {
            return false;
        }

        let density = self.density_noise.get(gx as f32, gz as f32);
        if density < self.density_threshold {
            return false;
        }

        let score = self.tree_score(gx, gz);
        let r = self.min_radius;
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

        true
    }

    /// Trunk height for the tree at `(gx, gz)`, in `[min, min + span)` blocks. A separate hash salt
    /// decorrelates height from the placement score.
    pub(crate) fn trunk_height(&self, gx: i32, gz: i32, min: u8, span: u8) -> u8 {
        let h = self.position_hash(gx ^ 0xdead, gz ^ 0xbeef);
        min + (h % u64::from(span.max(1))) as u8
    }
}
