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
use lazy_static::lazy_static;
use std::collections::HashMap;

pub mod spread;

#[cfg(test)]
mod tests_integration;

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
