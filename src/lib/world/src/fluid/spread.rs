//! Fluid spreading algorithm (simplified model).
//!
//! This module contains the pure decision logic for how a single fluid block evolves on its
//! scheduled tick. It is deliberately decoupled from world storage, the ECS, and networking: it
//! reads the world only through the [`BlockView`] trait and returns a list of [`FluidChange`]s
//! describing what should happen, without mutating anything itself. This makes the rules
//! independently unit-testable and easy to revise.
//!
//! # Simplified model
//!
//! The model approximates vanilla without the full "shortest path to a hole" search:
//!
//! * A block is a *source* (level 0) or *flowing* (level 1..=[`MAX_SPREAD_LEVEL`]).
//! * If the cell below is replaceable, fluid flows straight down. Downward flow always produces a
//!   near-source flowing block, so a falling column stays full.
//! * Otherwise, fluid spreads horizontally to each replaceable neighbour whose resulting level
//!   would be lower than that neighbour's current level, decrementing the level by one step each
//!   block. Once the level passes [`MAX_SPREAD_LEVEL`], the fluid no longer spreads horizontally.
//! * A flowing (non-source) block that has lost all of its upstream support dries up: it is
//!   scheduled to be removed. (Upstream = a same-fluid source/flow above, or an adjacent block
//!   with a stronger, i.e. lower, level.)
//!
//! Water and lava share this logic; they differ only in [`crate::fluid::FluidKind`]-derived
//! parameters supplied by the caller (spread distance and tick delay live with the caller, not
//! here).
//! til 2026-06-02, there is a crash problem due to(maybe) chunk storage. Placing any fluid may cause block disappear in a  specific section. It is fine, NCC is gonna fix it in another PR.

use crate::block_state_id::BlockStateId;
use crate::fluid::{fluid_block, fluid_state, FluidKind, FluidRules, FluidState};
use crate::pos::BlockPos;
use ferrumc_macros::block;

/// A read-only view of block states around the position being ticked.
///
/// The algorithm only ever inspects the ticking block and its six axis-aligned neighbours, so any
/// backing store (a live chunk, a test fixture, a snapshot) can implement this cheaply.
pub trait BlockView {
    /// Returns the block state at an absolute world position.
    fn block_at(&self, pos: BlockPos) -> BlockStateId;
}

/// A single block mutation produced by a fluid tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FluidChange {
    /// The position to modify.
    pub pos: BlockPos,
    /// The new block state to place there.
    pub new_block: BlockStateId,
    /// Whether the changed block should itself be scheduled for a follow-up fluid tick (because it
    /// is flowing fluid that may continue to spread).
    pub reschedule: bool,
}

/// Returns true if a fluid may overwrite this block.
///
/// The simplified model treats air and existing fluid of any kind as replaceable. Solid blocks are
/// not. (Fluid-vs-fluid interactions such as water meeting lava are intentionally out of scope for
/// this phase; they are recorded as future work.)
fn is_replaceable(block: BlockStateId) -> bool {
    block == block!("air") || block == block!("void_air") || fluid_state(block).is_some()
}

/// The six axis-aligned neighbours of `pos`, plus convenience accessors.
fn below(pos: BlockPos) -> BlockPos {
    pos + (0, -1, 0)
}
fn above(pos: BlockPos) -> BlockPos {
    pos + (0, 1, 0)
}
const HORIZONTAL: [(i32, i32, i32); 4] = [(1, 0, 0), (-1, 0, 0), (0, 0, 1), (0, 0, -1)];

/// Returns true if `neighbour` provides upstream support for a flowing block of `kind`.
///
/// Support comes from the same fluid directly above, or from a horizontally adjacent same-fluid
/// block with a strictly stronger (lower) level.
fn provides_support(kind: FluidKind, self_level: u8, neighbour: Option<FluidState>) -> bool {
    match neighbour {
        Some(state) if state.kind == kind => state.level < self_level,
        _ => false,
    }
}

/// Computes the changes a single fluid block produces on its tick.
///
/// `pos` must currently contain a fluid (otherwise an empty change set is returned). `rules`
/// supplies the per-fluid, per-dimension parameters (level step, max spread level); the caller is
/// expected to look these up via [`FluidRules::for_kind`] using the block at `pos`.
///
/// The returned changes are not applied; the caller is responsible for writing them back and for
/// scheduling any follow-up ticks indicated by [`FluidChange::reschedule`].
pub fn compute_fluid_tick<V: BlockView>(
    pos: BlockPos,
    view: &V,
    rules: FluidRules,
) -> Vec<FluidChange> {
    let current = view.block_at(pos);
    let Some(state) = fluid_state(current) else {
        return Vec::new();
    };
    let kind = state.kind;
    let mut changes = Vec::new();

    // --- Determine whether this block still has upstream support. ---
    // Sources are always supported. Flowing blocks need either fluid above or a stronger
    // horizontal neighbour.
    let supported = if state.is_source() {
        true
    } else {
        let above_state = fluid_state(view.block_at(above(pos)));
        let above_supports = matches!(above_state, Some(s) if s.kind == kind);
        above_supports
            || HORIZONTAL.iter().any(|&offset| {
                let n = fluid_state(view.block_at(pos + offset));
                provides_support(kind, state.level, n)
            })
    };

    // A flowing block with no support dries up.
    if !supported {
        changes.push(FluidChange {
            pos,
            new_block: block!("air"),
            reschedule: false,
        });
        return changes;
    }

    // --- Vertical flow takes priority. Or fluid will pruh~ pruh~ unfolded in the air ---
    let below_pos = below(pos);
    let below_block = view.block_at(below_pos);
    if is_replaceable(below_block) {
        // Falling fluid stays effectively full so columns do not thin out as they fall.
        let falling = fluid_block(kind, 1);
        if below_block != falling {
            changes.push(FluidChange {
                pos: below_pos,
                new_block: falling,
                reschedule: true,
            });
        }
        // When fluid can fall, vanilla does not also spread horizontally from this block, so we
        // stop here. And this code should be reviewed if it is conflict with chunk saving.
        return changes;
    }

    // --- Horizontal spread. ---
    // The level step is fluid- and dimension-dependent: vanilla water steps by 1 (7 blocks of
    // reach), overworld lava by 2 (3 blocks), nether lava by 1 (7 blocks). Saturating add so a
    // pathological level near u8::MAX cannot wrap around.
    let next_level = state.level.saturating_add(rules.level_step);
    if next_level > rules.max_spread_level {
        return changes;
    }

    let outflow = fluid_block(kind, next_level);
    for &offset in HORIZONTAL.iter() {
        let n_pos = pos + offset;
        let n_block = view.block_at(n_pos);
        if !is_replaceable(n_block) {
            continue;
        }
        // Only flow in if it would make the neighbour stronger (lower level) than it currently is.
        let improves = match fluid_state(n_block) {
            Some(n_state) if n_state.kind == kind => next_level < n_state.level,
            Some(_) => false, // different fluid: skip in this phase
            None => true,     // air: always flow in
        };
        if improves {
            changes.push(FluidChange {
                pos: n_pos,
                new_block: outflow,
                reschedule: true,
            });
        }
    }

    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Simple map-backed [`BlockView`] for tests. Any position not present is treated as air.
    struct MapView {
        blocks: HashMap<(i32, i32, i32), BlockStateId>,
    }

    impl MapView {
        fn new() -> Self {
            Self {
                blocks: HashMap::new(),
            }
        }

        fn set(&mut self, pos: BlockPos, block: BlockStateId) {
            self.blocks.insert((pos.pos.x, pos.pos.y, pos.pos.z), block);
        }
    }

    impl BlockView for MapView {
        fn block_at(&self, pos: BlockPos) -> BlockStateId {
            self.blocks
                .get(&(pos.pos.x, pos.pos.y, pos.pos.z))
                .copied()
                .unwrap_or_else(|| block!("air"))
        }
    }

    fn p(x: i32, y: i32, z: i32) -> BlockPos {
        BlockPos::of(x, y, z)
    }

    /// Convenience: vanilla water rules in the overworld. Most spread tests in this module care
    /// about the underlying mechanics rather than per-fluid parameters, so they share this.
    fn water_rules() -> FluidRules {
        FluidRules::for_kind(FluidKind::Water, crate::dimension::Dimension::Overworld)
    }

    #[test]
    fn non_fluid_produces_no_changes() {
        let mut view = MapView::new();
        view.set(p(0, 64, 0), block!("stone"));
        assert!(compute_fluid_tick(p(0, 64, 0), &view, water_rules()).is_empty());
    }

    #[test]
    fn source_on_solid_ground_spreads_horizontally() {
        let mut view = MapView::new();
        // Water source with solid ground below so it cannot fall.
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0));
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, water_rules());
        // Should flow into all 4 horizontal air neighbours at level 1.
        assert_eq!(changes.len(), 4);
        for c in &changes {
            assert_eq!(c.new_block, fluid_block(FluidKind::Water, 1));
            assert!(c.reschedule);
            assert_eq!(c.pos.pos.y, 64);
        }
    }

    #[test]
    fn source_over_air_falls_instead_of_spreading() {
        let mut view = MapView::new();
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0));
        // below is air (default)

        let changes = compute_fluid_tick(p(0, 64, 0), &view, water_rules());
        assert_eq!(changes.len(), 1, "should only flow down, not sideways");
        assert_eq!(changes[0].pos, p(0, 63, 0));
        assert_eq!(changes[0].new_block, fluid_block(FluidKind::Water, 1));
    }

    #[test]
    fn flowing_decrements_level_each_step() {
        let mut view = MapView::new();
        // A supported level-3 flowing block (source above) on solid ground spreads at level 4.
        view.set(p(0, 65, 0), fluid_block(FluidKind::Water, 0));
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 3));
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, water_rules());
        // Ground below is solid and the block is supported, so it spreads sideways at level 4.
        assert_eq!(changes.len(), 4);
        for c in &changes {
            assert_eq!(c.new_block, fluid_block(FluidKind::Water, 4));
        }
    }

    #[test]
    fn does_not_spread_past_max_level() {
        let mut view = MapView::new();
        // A supported block at the rules' max level: next level would exceed the cap, so no
        // spread.
        let rules = water_rules();
        view.set(p(0, 65, 0), fluid_block(FluidKind::Water, 0));
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, rules.max_spread_level));
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, rules);
        assert!(
            changes.is_empty(),
            "fluid at max level should not spread further"
        );
    }

    #[test]
    fn unsupported_flowing_block_dries_up() {
        let mut view = MapView::new();
        // A flowing block with solid ground below, air above, and no stronger neighbours.
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 3));
        view.set(p(0, 63, 0), block!("stone"));
        // surrounded by air horizontally, nothing above

        let changes = compute_fluid_tick(p(0, 64, 0), &view, water_rules());
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].pos, p(0, 64, 0));
        assert_eq!(changes[0].new_block, block!("air"));
        assert!(!changes[0].reschedule);
    }

    #[test]
    fn flowing_block_with_source_above_is_supported() {
        let mut view = MapView::new();
        // Source above keeps this flowing block alive; ground below; it should spread, not dry up.
        view.set(p(0, 65, 0), fluid_block(FluidKind::Water, 0));
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 1));
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, water_rules());
        // Supported, on solid ground -> spreads at level 2 into 4 neighbours.
        assert_eq!(changes.len(), 4);
        for c in &changes {
            assert_eq!(c.new_block, fluid_block(FluidKind::Water, 2));
        }
    }

    #[test]
    fn does_not_overwrite_solid_neighbours() {
        let mut view = MapView::new();
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0));
        view.set(p(0, 63, 0), block!("stone"));
        view.set(p(1, 64, 0), block!("stone")); // solid wall to the east

        let changes = compute_fluid_tick(p(0, 64, 0), &view, water_rules());
        // Only 3 open neighbours now.
        assert_eq!(changes.len(), 3);
        assert!(changes.iter().all(|c| c.pos != p(1, 64, 0)));
    }

    #[test]
    fn does_not_reflow_into_equal_or_stronger_neighbour() {
        let mut view = MapView::new();
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0));
        view.set(p(0, 63, 0), block!("stone"));
        // East neighbour already level 1 (same as what we'd produce) -> no change there.
        view.set(p(1, 64, 0), fluid_block(FluidKind::Water, 1));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, water_rules());
        // 3 air neighbours get level 1; the existing level-1 neighbour is left alone.
        assert_eq!(changes.len(), 3);
        assert!(changes.iter().all(|c| c.pos != p(1, 64, 0)));
    }

    #[test]
    fn lava_uses_its_own_states() {
        use crate::dimension::Dimension;

        let mut view = MapView::new();
        view.set(p(0, 64, 0), fluid_block(FluidKind::Lava, 0));
        view.set(p(0, 63, 0), block!("stone"));

        let lava = FluidRules::for_kind(FluidKind::Lava, Dimension::Overworld);
        let changes = compute_fluid_tick(p(0, 64, 0), &view, lava);
        assert_eq!(changes.len(), 4);
        for c in &changes {
            // Overworld lava steps by 2, so a source produces level-2 flow on adjacent blocks.
            assert_eq!(c.new_block, fluid_block(FluidKind::Lava, lava.level_step));
            // Ensure it is lava, not water.
            assert_eq!(fluid_state(c.new_block).unwrap().kind, FluidKind::Lava);
        }
    }

    #[test]
    fn overworld_lava_caps_spread_short_of_water() {
        use crate::dimension::Dimension;

        // Overworld lava: level_step = 2, max_spread_level = 7. From a source we expect
        // level 2 -> 4 -> 6 to be reachable (3 blocks); a level-6 block must not spread further
        // because 6 + 2 = 8 > 7.
        let lava = FluidRules::for_kind(FluidKind::Lava, Dimension::Overworld);
        assert_eq!(lava.level_step, 2);

        let mut view = MapView::new();
        view.set(p(0, 65, 0), fluid_block(FluidKind::Lava, 0)); // support from above
        view.set(p(0, 64, 0), fluid_block(FluidKind::Lava, 6));
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, lava);
        assert!(
            changes.is_empty(),
            "overworld lava at level 6 should stop (next would be 8, > 7)"
        );
    }

    #[test]
    fn nether_lava_spreads_like_water() {
        use crate::dimension::Dimension;

        // Nether lava: level_step = 1, max_spread_level = 7 — same reach as water.
        let lava = FluidRules::for_kind(FluidKind::Lava, Dimension::Nether);
        assert_eq!(lava.level_step, 1);

        let mut view = MapView::new();
        view.set(p(0, 65, 0), fluid_block(FluidKind::Lava, 0));
        view.set(p(0, 64, 0), fluid_block(FluidKind::Lava, 6));
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, lava);
        // Nether lava can still take one more step from level 6 -> 7.
        assert_eq!(changes.len(), 4);
        for c in &changes {
            assert_eq!(c.new_block, fluid_block(FluidKind::Lava, 7));
        }
    }
}
