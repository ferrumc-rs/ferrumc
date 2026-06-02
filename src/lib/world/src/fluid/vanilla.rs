//! Vanilla-faithful fluid spreading kernel (work in progress).
//!
//! This is the second of two interchangeable spreading kernels. The first,
//! [`crate::fluid::spread::compute_fluid_tick`], is a cheap approximation; this one mirrors
//! Minecraft's `FlowingFluid` logic, including the bounded "slope search" (`getSlopeDistance`)
//! that makes fluid steer toward the nearest hole instead of spreading uniformly.
//!
//! It deliberately shares the same seams as the simplified kernel so the two can be A/B compared
//! and swapped without touching the caller:
//! * input is a read-only [`BlockView`] plus the ticking [`BlockPos`] and the [`FluidRules`];
//! * output is a `Vec<FluidChange>` of *real* mutations only (neighbour waking stays the caller's
//!   job);
//! * the lava/water solidification rules and the `fizz` flag are identical (reused from
//!   [`crate::fluid::spread`]).
//!
//! # Level model
//!
//! Vanilla encodes a flowing fluid's "amount" as 1..=8 where 8 is full (just below a source) and 1
//! is the thinnest, and uses a separate `falling` flag. FerrumC's block model instead stores a
//! `level` 0..=15 where 0 is the source and higher is thinner (see [`FluidState`]). To stay within
//! the existing block mapping we keep FerrumC's convention and express "amount" as
//! `max_spread_level + 1 - level` internally only where the slope comparison needs a magnitude.
//!
//! # Determinism
//!
//! The slope search and the spread decision must be order-independent so the parallel and serial
//! evaluators agree (the parity test is the guard). Every neighbour iteration uses the fixed
//! [`HORIZONTAL`] order, and ties in the slope search are broken by that same order.

use crate::block_state_id::BlockStateId;
use crate::fluid::spread::{
    is_replaceable_by, lava_harden_check, lava_water_down_reaction, BlockView, FluidChange,
    HORIZONTAL,
};
use crate::fluid::{fluid_block, fluid_state, FluidKind, FluidRules};
use crate::pos::BlockPos;
use ferrumc_macros::block;

fn below(pos: BlockPos) -> BlockPos {
    pos + (0, -1, 0)
}
fn above(pos: BlockPos) -> BlockPos {
    pos + (0, 1, 0)
}

/// Whether fluid of `kind` can flow *down* into the block below `pos` (used to decide if a
/// position counts as a "hole" the slope search should steer toward).
fn can_flow_down_into<V: BlockView>(kind: FluidKind, pos: BlockPos, view: &V) -> bool {
    is_replaceable_by(kind, view.block_at(below(pos)))
}

/// Returns the shortest horizontal distance (in blocks, 1..=`limit`) from `from` to a cell where
/// the fluid could fall downward, searching only through cells the fluid could actually flow
/// across. Returns `limit` if no hole is found within range — matching vanilla, which treats
/// "no hole in range" as the maximum distance so flat spreading still happens.
///
/// This is a bounded breadth-first search (vanilla's `getSlopeDistance` is the recursive
/// equivalent). `origin` is excluded from being treated as its own hole.
fn slope_distance<V: BlockView>(
    kind: FluidKind,
    origin: BlockPos,
    from: BlockPos,
    skip_back_toward: Option<BlockPos>,
    depth: u8,
    limit: u8,
    view: &V,
) -> u8 {
    let mut best = limit;
    for &offset in HORIZONTAL.iter() {
        let next = from + offset;
        // Do not search back toward the cell we came from, and never revisit the origin.
        if Some(next) == skip_back_toward || next == origin {
            continue;
        }
        if !is_replaceable_by(kind, view.block_at(next)) {
            continue;
        }
        // Found a hole: this branch costs `depth + 1` blocks.
        if can_flow_down_into(kind, next, view) {
            return depth + 1;
        }
        // Otherwise keep searching outward until the limit is reached.
        if depth + 1 < limit {
            let found = slope_distance(kind, origin, next, Some(from), depth + 1, limit, view);
            if found < best {
                best = found;
            }
        }
    }
    best
}

/// The level a freshly spread flowing block should take when fed from a block of `source_level`
/// (or from a source, when `source_level` is 0). One `level_step` thinner than its feeder, clamped
/// to the cap.
fn spread_level(rules: FluidRules, source_level: u8) -> u8 {
    source_level
        .saturating_add(rules.level_step)
        .min(rules.max_spread_level)
}

/// Computes a single fluid block's changes for this tick, vanilla-style.
///
/// Mirrors `FlowingFluid.tick` + `spread`: resolve lava/water reactions first, then fall straight
/// down if possible (full column), otherwise spread horizontally — but only toward the directions
/// the slope search says are closest to a hole (steering), decrementing the level by `level_step`.
///
/// Like the simplified kernel this returns only real mutations; the caller wakes neighbours.
pub fn compute_fluid_tick_vanilla<V: BlockView>(
    pos: BlockPos,
    view: &V,
    rules: FluidRules,
) -> Vec<FluidChange> {
    let Some(state) = fluid_state(view.block_at(pos)) else {
        return Vec::new();
    };
    let kind = state.kind;
    let mut changes = Vec::new();

    // --- Lava/water interaction first (identical rules to the simplified kernel). ---
    if kind == FluidKind::Lava {
        if let Some(rock) = lava_harden_check(pos, state, view) {
            changes.push(FluidChange::reaction(pos, rock));
            return changes;
        }
        if let Some(reaction) = lava_water_down_reaction(pos, view) {
            changes.push(reaction);
            return changes;
        }
    }

    // --- Recede / dry-up for flowing blocks that have lost their feed. ---
    // The vanilla "new liquid" computation: a flowing block's level is dictated by its strongest
    // horizontal feeder or by fluid above. If that drops below what the block currently is, it
    // recedes one step; if no feeder remains, it dries up. (Shared with the simplified model via
    // `new_liquid_level`.)
    if !state.is_source() {
        match new_liquid_level(pos, kind, rules, view) {
            None => {
                changes.push(FluidChange::flow(pos, block!("air"), false));
                return changes;
            }
            Some(target) if target != state.level => {
                changes.push(FluidChange::flow(pos, fluid_block(kind, target), true));
                return changes;
            }
            Some(_) => {}
        }
    }

    // --- Downward flow takes priority: a full column. ---
    let below_pos = below(pos);
    if is_replaceable_by(kind, view.block_at(below_pos)) {
        let falling = fluid_block(kind, rules.level_step.min(rules.max_spread_level));
        if view.block_at(below_pos) != falling {
            changes.push(FluidChange::flow(below_pos, falling, true));
        }
        return changes;
    }

    // --- Horizontal spread with slope steering. ---
    let next_level = spread_level(rules, state.level);
    if next_level > rules.max_spread_level {
        return changes;
    }

    // Compute, for each open horizontal direction, the slope distance to the nearest hole. Only
    // the directions tied for the minimum distance actually receive flow (vanilla steering).
    let limit = rules.slope_find_distance.max(1);
    let mut dir_distance: [Option<u8>; 4] = [None; 4];
    let mut min_distance = u8::MAX;
    for (i, &offset) in HORIZONTAL.iter().enumerate() {
        let n_pos = pos + offset;
        if !can_spread_into(kind, n_pos, next_level, view) {
            continue;
        }
        let d = if can_flow_down_into(kind, n_pos, view) {
            0 // an immediately-falling neighbour is the strongest possible attractor
        } else {
            slope_distance(kind, pos, n_pos, Some(pos), 1, limit, view)
        };
        dir_distance[i] = Some(d);
        if d < min_distance {
            min_distance = d;
        }
    }

    if min_distance == u8::MAX {
        // Nothing to spread into.
        return changes;
    }

    let outflow = fluid_block(kind, next_level);
    for (i, &offset) in HORIZONTAL.iter().enumerate() {
        if dir_distance[i] == Some(min_distance) {
            changes.push(FluidChange::flow(pos + offset, outflow, true));
        }
    }

    changes
}

/// Whether flow at `next_level` may enter `n_pos` (open to this fluid and an improvement).
fn can_spread_into<V: BlockView>(
    kind: FluidKind,
    n_pos: BlockPos,
    next_level: u8,
    view: &V,
) -> bool {
    let n_block = view.block_at(n_pos);
    if !is_replaceable_by(kind, n_block) {
        return false;
    }
    match fluid_state(n_block) {
        Some(n_state) if n_state.kind == kind => next_level < n_state.level,
        Some(_) => false, // opposite fluid: reaction path handles it
        None => true,     // air
    }
}

/// The level a flowing block should hold this tick, or `None` if it should dry up.
///
/// Vanilla's `getNewLiquid`: the block is fed by fluid directly above (a full column) or by the
/// strongest horizontal same-fluid neighbour (`neighbour.level + level_step`). Water can also form
/// a new source between two sources, but that is handled separately. Returns the strongest
/// (lowest) supportable level within the cap.
fn new_liquid_level<V: BlockView>(
    pos: BlockPos,
    kind: FluidKind,
    rules: FluidRules,
    view: &V,
) -> Option<u8> {
    if matches!(fluid_state(view.block_at(above(pos))), Some(s) if s.kind == kind) {
        return Some(rules.level_step.min(rules.max_spread_level));
    }

    let mut best: Option<u8> = None;
    for &offset in HORIZONTAL.iter() {
        if let Some(state) = fluid_state(view.block_at(pos + offset)) {
            if state.kind == kind {
                let fed = state.level.saturating_add(rules.level_step);
                if fed <= rules.max_spread_level {
                    best = Some(best.map_or(fed, |b| b.min(fed)));
                }
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimension::Dimension;
    use std::collections::HashMap;

    /// Map-backed [`BlockView`]; missing positions read as air.
    struct MapView {
        blocks: HashMap<(i32, i32, i32), BlockStateId>,
    }
    impl MapView {
        fn new() -> Self {
            Self {
                blocks: HashMap::new(),
            }
        }
        fn set(&mut self, pos: BlockPos, b: BlockStateId) {
            self.blocks.insert((pos.pos.x, pos.pos.y, pos.pos.z), b);
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
    fn water() -> FluidRules {
        FluidRules::for_kind(FluidKind::Water, Dimension::Overworld)
    }

    /// A solid platform with a single hole: water on top should steer toward the hole rather than
    /// spreading uniformly in all four directions.
    #[test]
    fn spread_steers_toward_a_hole() {
        let mut view = MapView::new();
        // Large solid floor at y=63 over the whole search area, so no direction finds a hole by
        // simply running off the edge of a tiny platform.
        for x in -6..=6 {
            for z in -6..=6 {
                view.set(p(x, 63, z), block!("stone"));
            }
        }
        // Carve ONE hole in the floor two blocks east of the source (remove the floor at (2,63,0)).
        view.blocks.remove(&(2, 63, 0));
        // Water source at (0,64,0).
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0));

        let changes = compute_fluid_tick_vanilla(p(0, 64, 0), &view, water());
        // The nearest (only) hole is east: distance 2 going +x, larger in every other direction.
        assert!(
            changes.iter().any(|c| c.pos == p(1, 64, 0)),
            "water should spread toward the hole (+x), changes: {:?}",
            changes
        );
        // It must NOT spread in the other three directions, which are strictly farther from a hole.
        assert!(
            changes.iter().all(|c| c.pos != p(-1, 64, 0)),
            "water should not spread away from the nearest hole (west), changes: {:?}",
            changes
        );
        assert!(
            changes
                .iter()
                .all(|c| c.pos != p(0, 64, 1) && c.pos != p(0, 64, -1)),
            "water should not spread along z, away from the hole, changes: {:?}",
            changes
        );
    }

    /// On a fully flat platform with no hole in range, spread is uniform in all four directions
    /// (the slope search returns the limit everywhere, so all directions tie).
    #[test]
    fn flat_platform_spreads_uniformly() {
        let mut view = MapView::new();
        for x in -3..=3 {
            for z in -3..=3 {
                view.set(p(x, 63, z), block!("stone"));
            }
        }
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0));

        let changes = compute_fluid_tick_vanilla(p(0, 64, 0), &view, water());
        let spread: Vec<_> = changes.iter().filter(|c| c.pos.pos.y == 64).collect();
        assert_eq!(
            spread.len(),
            4,
            "flat ground should spread to all 4 sides: {:?}",
            changes
        );
        for c in &spread {
            assert_eq!(c.new_block, fluid_block(FluidKind::Water, 1));
        }
    }

    /// Source over air falls straight down and does not also spread sideways.
    #[test]
    fn source_over_air_falls() {
        let mut view = MapView::new();
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0));
        let changes = compute_fluid_tick_vanilla(p(0, 64, 0), &view, water());
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].pos, p(0, 63, 0));
    }

    /// An immediately-falling neighbour (distance 0) outranks a far hole, so flow goes there.
    #[test]
    fn prefers_adjacent_drop_over_distant_hole() {
        let mut view = MapView::new();
        // Source at origin, floored.
        view.set(p(0, 63, 0), block!("stone"));
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0));
        // East neighbour (1,64,0) has NO floor -> immediate drop (distance 0).
        // West side floored for a long way with a hole far away.
        for x in -4..=-1 {
            view.set(p(x, 63, 0), block!("stone"));
        }
        view.set(p(0, 63, 1), block!("stone"));
        view.set(p(0, 63, -1), block!("stone"));

        let changes = compute_fluid_tick_vanilla(p(0, 64, 0), &view, water());
        assert!(
            changes.iter().any(|c| c.pos == p(1, 64, 0)),
            "should flow toward the adjacent drop, changes: {:?}",
            changes
        );
        assert!(
            changes.iter().all(|c| c.pos != p(-1, 64, 0)),
            "should not flow away from the adjacent drop"
        );
    }

    /// Lava reaction parity: the vanilla kernel reuses the same solidification rules.
    #[test]
    fn lava_reactions_match_shared_rules() {
        let lava = FluidRules::for_kind(FluidKind::Lava, Dimension::Overworld);

        // flowing lava beside water -> cobblestone
        let mut v = MapView::new();
        v.set(p(0, 64, 0), fluid_block(FluidKind::Lava, 2));
        v.set(p(1, 64, 0), fluid_block(FluidKind::Water, 0));
        let c = compute_fluid_tick_vanilla(p(0, 64, 0), &v, lava);
        assert_eq!(
            c.iter().find(|c| c.pos == p(0, 64, 0)).unwrap().new_block,
            block!("cobblestone")
        );

        // lava above water -> water below becomes stone
        let mut v = MapView::new();
        v.set(p(0, 65, 0), fluid_block(FluidKind::Lava, 2));
        v.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0));
        let c = compute_fluid_tick_vanilla(p(0, 65, 0), &v, lava);
        let lower = c.iter().find(|c| c.pos == p(0, 64, 0)).unwrap();
        assert_eq!(lower.new_block, block!("stone"));
        assert!(lower.fizz);
    }
}
