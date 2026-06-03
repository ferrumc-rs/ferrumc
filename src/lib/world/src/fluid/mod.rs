//! Runtime fluid property lookups.
//!
//! The [`crate::block_state_id::BlockStateId`] type stores blocks as flat protocol IDs, which is
//! efficient but opaque: it carries no information about whether a given state is a fluid or what
//! its level is. The [`block!`](ferrumc_macros::block) macro can resolve names and properties, but
//! only at compile time. Fluid spreading needs to inspect and construct fluid states at runtime
//! (reading a block's level, producing the next-lower level, etc.), so this module precomputes a
//! pair of small lookup tables at startup.
//!
//! The tables are intentionally the only place that knows how fluid states map to IDs. Fluid
//! spreading logic depends on the [`FluidKind`] / level abstraction exposed here rather than on
//! raw block IDs, which keeps the spreading algorithm decoupled from the block registry and easy
//! to revise later.

use crate::block_state_id::{BlockStateId, ID2BLOCK};
use crate::dimension::Dimension;
use ferrumc_config::server_config::FluidAlgorithm;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub mod spread;
pub mod vanilla;

/// Dispatches a single fluid tick to the kernel selected by `algorithm`.
///
/// Both kernels share the same seams (a read-only [`spread::BlockView`] in, a `Vec` of
/// [`spread::FluidChange`] out) and both return real mutations only, leaving neighbour waking to
/// the caller. This indirection is the single switch point: callers pass the configured
/// [`FluidAlgorithm`] and never name a kernel directly.
#[inline]
pub fn compute_tick<V: spread::BlockView>(
    algorithm: FluidAlgorithm,
    pos: crate::pos::BlockPos,
    view: &V,
    rules: FluidRules,
) -> Vec<spread::FluidChange> {
    match algorithm {
        FluidAlgorithm::Simplified => spread::compute_fluid_tick(pos, view, rules),
        FluidAlgorithm::Vanilla => vanilla::compute_fluid_tick_vanilla(pos, view, rules),
    }
}

#[cfg(test)]
mod tests_integration;
#[cfg(test)]
mod osc_probe;

/// The category of a fluid.
///
/// Water and lava share the same spreading machinery but differ in their parameters (spread
/// distance, tick delay). Keeping them as a small enum lets the spreading algorithm branch on
/// behaviour without hard-coding block IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FluidKind {
    Water,
    Lava,
}

impl FluidKind {
    /// The Minecraft block name backing this fluid.
    const fn block_name(self) -> &'static str {
        match self {
            FluidKind::Water => "minecraft:water",
            FluidKind::Lava => "minecraft:lava",
        }
    }
}

/// Per-fluid, per-dimension behaviour parameters.
///
/// Vanilla differentiates water and lava on two axes (how fast they tick, how far they spread)
/// and lava additionally behaves differently in the Nether (faster + further). Encoding this as
/// a small struct keeps the spreading algorithm dimension-agnostic: callers look up the rules
/// once with [`FluidRules::for_kind`] and pass the struct in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FluidRules {
    /// How much the fluid `level` increases for each horizontal step away from a source.
    /// Vanilla water = 1 (spreads 7 blocks), overworld lava = 2 (spreads 3 blocks),
    /// Nether lava = 1 (spreads 7 blocks).
    pub level_step: u8,
    /// Maximum `level` that still spreads horizontally. A flowing block at `level >
    /// max_spread_level` stops spreading. Combined with `level_step`, this defines the maximum
    /// reach: `floor(max_spread_level / level_step)` blocks from a source on flat ground.
    pub max_spread_level: u8,
    /// Number of game ticks between scheduled updates of a flowing block of this fluid.
    /// Vanilla water = 5, overworld lava = 30, Nether lava = 10.
    pub tick_delay: u64,
    /// How many blocks the vanilla-style slope search (`getSlopeDistance`) looks outward for the
    /// nearest hole before giving up. Vanilla: water = 4, overworld lava = 2, Nether lava = 4.
    /// Only used by the vanilla spread kernel; the simplified model ignores it.
    pub slope_find_distance: u8,
    /// Whether two or more adjacent source blocks can spontaneously create a new source between
    /// them (infinite water). Vanilla: water = true, lava = false (lava is never infinite in
    /// unmodified overworld/Nether). Only used by the vanilla spread kernel.
    pub can_form_source: bool,
}

impl FluidRules {
    /// Returns the rules vanilla uses for `(kind, dimension)`.
    ///
    /// `Dimension::End` falls back to overworld parameters because vanilla treats the End the
    /// same as the overworld for fluid behaviour.
    pub const fn for_kind(kind: FluidKind, dimension: Dimension) -> Self {
        match (kind, dimension) {
            (FluidKind::Water, _) => Self {
                level_step: 1,
                max_spread_level: 7,
                tick_delay: 5,
                slope_find_distance: 4,
                can_form_source: true,
            },
            (FluidKind::Lava, Dimension::Nether) => Self {
                level_step: 1,
                max_spread_level: 7,
                tick_delay: 10,
                slope_find_distance: 4,
                can_form_source: false,
            },
            (FluidKind::Lava, Dimension::Overworld | Dimension::End) => Self {
                level_step: 2,
                max_spread_level: 7,
                tick_delay: 30,
                slope_find_distance: 2,
                can_form_source: false,
            },
        }
    }
}

/// The maximum `level` property value a fluid state can have (inclusive). Vanilla fluids use
/// levels 0..=15, where 0 is the source/full block.
pub const MAX_FLUID_LEVEL: u8 = 15;

/// A decoded fluid state: which fluid it is and its `level` property.
///
/// In Minecraft's data model, `level` 0 is a source (full) block and increasing values are
/// progressively shallower flowing fluid. (Levels 8..=15 are the "falling" variants in vanilla;
/// the simplified spreading model does not yet distinguish them, but the raw level is preserved
/// here so a future revision can.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FluidState {
    pub kind: FluidKind,
    pub level: u8,
}

impl FluidState {
    /// Returns true if this is a source block (level 0).
    #[inline]
    pub fn is_source(&self) -> bool {
        self.level == 0
    }
}

struct FluidTables {
    /// Maps a block state ID to its decoded fluid state, for every fluid block state.
    id_to_fluid: HashMap<u32, FluidState>,
    /// Maps `(kind, level)` to the corresponding block state ID. Indexed as
    /// `level_to_id[kind][level]`.
    water_levels: [BlockStateId; (MAX_FLUID_LEVEL as usize) + 1],
    lava_levels: [BlockStateId; (MAX_FLUID_LEVEL as usize) + 1],
}

impl FluidTables {
    fn build() -> Self {
        let mut id_to_fluid = HashMap::new();
        let mut water_levels = [BlockStateId::default(); (MAX_FLUID_LEVEL as usize) + 1];
        let mut lava_levels = [BlockStateId::default(); (MAX_FLUID_LEVEL as usize) + 1];

        for kind in [FluidKind::Water, FluidKind::Lava] {
            let name = kind.block_name();
            let levels = match kind {
                FluidKind::Water => &mut water_levels,
                FluidKind::Lava => &mut lava_levels,
            };

            for (id, block) in ID2BLOCK.iter().enumerate() {
                if block.name != name {
                    continue;
                }
                let level = block
                    .properties
                    .as_ref()
                    .and_then(|props| props.get("level"))
                    .and_then(|lvl| lvl.parse::<u8>().ok());

                let Some(level) = level else {
                    continue;
                };
                if level > MAX_FLUID_LEVEL {
                    continue;
                }

                let state_id = BlockStateId::new(id as u32);
                levels[level as usize] = state_id;
                id_to_fluid.insert(id as u32, FluidState { kind, level });
            }
        }

        FluidTables {
            id_to_fluid,
            water_levels,
            lava_levels,
        }
    }

    #[inline]
    fn levels_for(&self, kind: FluidKind) -> &[BlockStateId; (MAX_FLUID_LEVEL as usize) + 1] {
        match kind {
            FluidKind::Water => &self.water_levels,
            FluidKind::Lava => &self.lava_levels,
        }
    }
}

lazy_static! {
    static ref FLUID_TABLES: FluidTables = FluidTables::build();
}

/// Returns the [`FluidState`] for a block, or `None` if the block is not a fluid.
#[inline]
pub fn fluid_state(block: BlockStateId) -> Option<FluidState> {
    FLUID_TABLES.id_to_fluid.get(&block.raw()).copied()
}

/// Returns true if the block is any fluid (water or lava, at any level).
#[inline]
pub fn is_fluid(block: BlockStateId) -> bool {
    FLUID_TABLES.id_to_fluid.contains_key(&block.raw())
}

/// Returns true if `block` is a solid obstacle from a fluid's point of view: not air, not any
/// fluid. Fluid flow must never overwrite such a block; the apply path uses this as a safety guard
/// against accidentally "eating" terrain.
#[inline]
pub fn is_solid_obstacle(block: BlockStateId) -> bool {
    use ferrumc_macros::block;
    block != block!("air") && block != block!("void_air") && !is_fluid(block)
}

/// Returns the block state ID for the given fluid kind and level.
///
/// `level` is clamped to the valid range `0..=MAX_FLUID_LEVEL`.
#[inline]
pub fn fluid_block(kind: FluidKind, level: u8) -> BlockStateId {
    let level = level.min(MAX_FLUID_LEVEL);
    FLUID_TABLES.levels_for(kind)[level as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferrumc_macros::block;

    #[test]
    fn water_source_round_trips() {
        let source = fluid_block(FluidKind::Water, 0);
        assert_eq!(source, block!("water", { level: 0 }));

        let state = fluid_state(source).expect("water source should be a fluid");
        assert_eq!(state.kind, FluidKind::Water);
        assert_eq!(state.level, 0);
        assert!(state.is_source());
    }

    #[test]
    fn lava_levels_resolve() {
        for level in 0..=MAX_FLUID_LEVEL {
            let id = fluid_block(FluidKind::Lava, level);
            let state = fluid_state(id).expect("lava state should be a fluid");
            assert_eq!(state.kind, FluidKind::Lava);
            assert_eq!(state.level, level);
        }
    }

    #[test]
    fn non_fluid_is_none() {
        assert!(fluid_state(block!("stone")).is_none());
        assert!(!is_fluid(block!("stone")));
        assert!(!is_fluid(block!("air")));
    }

    #[test]
    fn water_and_lava_are_distinct() {
        let water = fluid_block(FluidKind::Water, 0);
        let lava = fluid_block(FluidKind::Lava, 0);
        assert_ne!(water, lava);
        assert!(is_fluid(water));
        assert!(is_fluid(lava));
    }

    #[test]
    fn level_is_clamped() {
        // Out-of-range levels clamp to the max rather than panicking.
        let clamped = fluid_block(FluidKind::Water, 200);
        let max = fluid_block(FluidKind::Water, MAX_FLUID_LEVEL);
        assert_eq!(clamped, max);
    }
}
