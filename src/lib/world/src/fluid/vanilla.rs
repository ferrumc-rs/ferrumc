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
/// Upper bound on the search radius the stack-backed BFS supports. The largest `slope_find_distance`
/// any fluid uses is 4 (water); this leaves generous headroom while keeping the fixed buffers small.
const MAX_SLOPE_LIMIT: usize = 8;

fn slope_distance<V: BlockView>(
    kind: FluidKind,
    origin: BlockPos,
    from: BlockPos,
    limit: u8,
    view: &V,
) -> u8 {
    // The search is purely horizontal (every step is in `HORIZONTAL`, so y never changes) and bounded
    // to `limit` steps from the origin. Every cell it can reach therefore lies in a
    // `(2*limit+1)²` window in the x/z plane at the origin's y. That lets the visited set and the BFS
    // queue live on the stack as fixed-size arrays keyed by a cell's offset from the origin — no
    // per-call heap allocation, which matters because this runs up to four times per ticking fluid
    // block. The logic (FIFO BFS, mark-on-first-sight, first hole at minimum depth) is identical to
    // the previous `HashSet`/`VecDeque` version; keying by `(dx, dz)` is equivalent to `(x, y, z)`
    // because every visited cell shares the origin's y.
    let limit = (limit as usize).min(MAX_SLOPE_LIMIT) as u8;

    const SIDE: usize = 2 * MAX_SLOPE_LIMIT + 1;
    const CELLS: usize = SIDE * SIDE;
    let mut visited = [false; CELLS];
    // A cell at the origin's y mapped to its slot; offsets are bounded by `limit` <= MAX_SLOPE_LIMIT.
    let slot = |p: BlockPos| -> usize {
        let dx = (p.pos.x - origin.pos.x + MAX_SLOPE_LIMIT as i32) as usize;
        let dz = (p.pos.z - origin.pos.z + MAX_SLOPE_LIMIT as i32) as usize;
        dx * SIDE + dz
    };

    visited[slot(origin)] = true;
    visited[slot(from)] = true;

    // If the starting cell itself is a hole, distance is 1.
    if can_flow_down_into(kind, from, view) {
        return 1;
    }

    // FIFO queue; at most one entry per reachable cell, so `CELLS` slots always suffice.
    let mut queue: [(BlockPos, u8); CELLS] = [(from, 0); CELLS];
    let mut head = 0usize;
    let mut tail = 0usize;
    queue[tail] = (from, 1);
    tail += 1;

    while head < tail {
        let (cell, depth) = queue[head];
        head += 1;
        if depth >= limit {
            // Cannot expand further; this branch contributes no hole within range.
            continue;
        }
        for &offset in HORIZONTAL.iter() {
            let next = cell + offset;
            let s = slot(next);
            if visited[s] {
                continue;
            }
            visited[s] = true;
            // Can only flow across cells that are open to this fluid.
            if !is_replaceable_by(kind, view.block_at(next)) {
                continue;
            }
            let next_depth = depth + 1;
            if can_flow_down_into(kind, next, view) {
                // BFS guarantees the first hole reached is at the minimum distance.
                return next_depth;
            }
            queue[tail] = (next, next_depth);
            tail += 1;
        }
    }

    // No hole within range. Return a sentinel strictly larger than any real distance so a real
    // hole found at exactly `limit` is still preferred over flat ground (which returns this).
    NO_HOLE
}

/// Sentinel returned by [`slope_distance`] when no hole is reachable within the search limit.
/// Larger than any real distance so "found a hole at the max distance" still beats "no hole".
const NO_HOLE: u8 = u8::MAX;

/// The level a freshly spread flowing block would take when fed from a block of `source_level`
/// (or from a source, when `source_level` is 0): one `level_step` thinner than its feeder.
///
/// Intentionally **not** clamped to the cap. The caller compares the result against
/// `max_spread_level` to decide whether the fluid is still thick enough to spread at all; clamping
/// here would defeat that check and let a max-level cell spread forever (an edge cell would then
/// flip-flop between max-level water and air every tick).
fn spread_level(rules: FluidRules, source_level: u8) -> u8 {
    source_level.saturating_add(rules.level_step)
}

/// Computes a single fluid block's changes for this tick, vanilla-style.
///
/// Mirrors `FlowingFluid.tick` + `spread`: resolve lava/water reactions first, then fall straight
/// down if possible (full column), otherwise spread horizontally — but only toward the directions
/// the slope search says are closest to a hole (steering), decrementing the level by `level_step`.
///
/// Like the simplified kernel this returns only real mutations; the caller wakes neighbours.
/// DEBUG ONLY: returns the slope distance the spread step computes for each of the four
/// horizontal directions [+x, -x, +z, -z] from `pos`, or `None` where the direction cannot be
/// spread into. Used by diagnostics and steering tests to inspect the spread decision directly.
pub fn debug_spread_distances<V: BlockView>(
    pos: BlockPos,
    view: &V,
    rules: FluidRules,
) -> [Option<u8>; 4] {
    let Some(state) = fluid_state(view.block_at(pos)) else {
        return [None; 4];
    };
    let kind = state.kind;
    let next_level = spread_level(rules, state.level);
    let limit = rules.slope_find_distance.max(1);
    let mut out = [None; 4];
    if next_level > rules.max_spread_level {
        return out;
    }
    for (i, &offset) in HORIZONTAL.iter().enumerate() {
        let n_pos = pos + offset;
        if !can_search_into(kind, n_pos, view) {
            continue;
        }
        let dd = if can_flow_down_into(kind, n_pos, view) {
            0
        } else {
            slope_distance(kind, pos, n_pos, limit, view)
        };
        out[i] = Some(dd);
    }
    out
}

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
        let falling_level = rules.level_step.min(rules.max_spread_level);
        // Only push the falling level when it would actually strengthen the cell below. A stronger
        // cell — in particular a source (level 0) — must never be overwritten by the thinner falling
        // level: doing so weakens it, and where that cell re-forms a source the next tick (two
        // adjacent sources on solid ground) the two rules fight forever, oscillating the cell
        // between source and flowing. Either way the column counts as occupied, so this returns
        // without also spreading horizontally.
        if can_spread_into(kind, below_pos, falling_level, view) {
            changes.push(FluidChange::flow(
                below_pos,
                fluid_block(kind, falling_level),
                true,
            ));
        }
        return changes;
    }

    // --- Horizontal spread with slope steering. ---
    let next_level = spread_level(rules, state.level);
    if next_level > rules.max_spread_level {
        return changes;
    }

    // Slope steering, in two stages so a direction that already leads to the hole keeps "claiming"
    // the steer even after its immediate cell has filled:
    //
    // 1. For every direction whose immediate neighbour is *terrain-passable* for this fluid
    //    (air or same fluid — independent of whether flowing there would currently be an
    //    improvement), compute the slope distance to the nearest hole. This is the steering
    //    decision and must not depend on the transient fill state of the first cell, otherwise a
    //    source whose downhill neighbour is already wet would wrongly fan out the other ways.
    // 2. Emit flow only to the minimum-distance direction(s) AND only where it is actually an
    //    improvement (`can_spread_into`). A direction that is the chosen steer but already full
    //    simply produces no change this tick.
    //
    // If no direction finds a hole they all tie at NO_HOLE and the fluid spreads uniformly (flat
    // ground), matching vanilla.
    let limit = rules.slope_find_distance.max(1);
    let mut dir_distance: [Option<u8>; 4] = [None; 4];
    let mut min_distance = NO_HOLE;
    for (i, &offset) in HORIZONTAL.iter().enumerate() {
        let n_pos = pos + offset;
        if !can_search_into(kind, n_pos, view) {
            continue;
        }
        let d = if can_flow_down_into(kind, n_pos, view) {
            0 // an immediately-falling neighbour is the strongest possible attractor
        } else {
            slope_distance(kind, pos, n_pos, limit, view)
        };
        dir_distance[i] = Some(d);
        if d < min_distance {
            min_distance = d;
        }
    }

    let outflow = fluid_block(kind, next_level);
    for (i, &offset) in HORIZONTAL.iter().enumerate() {
        if dir_distance[i] == Some(min_distance) {
            let n_pos = pos + offset;
            // Only actually place fluid where it would be an improvement; the steering above may
            // have selected a direction whose first cell is already adequately filled.
            if can_spread_into(kind, n_pos, next_level, view) {
                changes.push(FluidChange::flow(n_pos, outflow, true));
            }
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

/// Whether the slope search may *traverse* `n_pos` for this fluid: the cell is terrain-passable
/// (air or the same fluid), regardless of whether depositing flow there would currently be an
/// improvement. Used for the steering decision so an already-filled downhill cell still counts as
/// "the way to the hole" and keeps the fluid from fanning out the other directions.
fn can_search_into<V: BlockView>(kind: FluidKind, n_pos: BlockPos, view: &V) -> bool {
    is_replaceable_by(kind, view.block_at(n_pos))
}

/// The level a flowing block should hold this tick, or `None` if it should dry up.
///
/// Mirrors vanilla's `getNewLiquid`, in priority order:
/// 1. **Source formation** — if this fluid can form sources and the block has at least two
///    horizontally adjacent same-fluid *sources* with solid (or same-fluid source) support below,
///    it becomes a source (`Some(0)`). This is what makes a 2x2 (or two-with-a-gap) water pool
///    self-heal into infinite water.
/// 2. fluid directly above feeds a full column → near-source level;
/// 3. otherwise the strongest horizontal same-fluid neighbour (`neighbour.level + level_step`).
///
/// Returns the strongest (lowest) supportable level within the cap, or `None` if nothing feeds it.
fn new_liquid_level<V: BlockView>(
    pos: BlockPos,
    kind: FluidKind,
    rules: FluidRules,
    view: &V,
) -> Option<u8> {
    if forms_source(pos, kind, rules, view) {
        return Some(0);
    }

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

/// Whether the cell at `pos` should spontaneously become a source of `kind` this tick.
///
/// Vanilla `canConvertToSource` + the source-count check in `getNewLiquid`: the fluid must allow
/// infinite sources (`rules.can_form_source`), there must be at least two horizontally adjacent
/// *source* blocks of the same fluid, and the block below must be either a same-fluid source or a
/// solid block (so the new source has something to rest on). This is how water heals back into a
/// full source between existing sources; lava never qualifies in the overworld/Nether.
fn forms_source<V: BlockView>(pos: BlockPos, kind: FluidKind, rules: FluidRules, view: &V) -> bool {
    if !rules.can_form_source {
        return false;
    }

    let adjacent_sources = HORIZONTAL
        .iter()
        .filter(|&&offset| {
            matches!(
                fluid_state(view.block_at(pos + offset)),
                Some(s) if s.kind == kind && s.is_source()
            )
        })
        .count();
    if adjacent_sources < 2 {
        return false;
    }

    let below_block = view.block_at(below(pos));
    let below_is_same_source =
        matches!(fluid_state(below_block), Some(s) if s.kind == kind && s.is_source());
    // "Solid" here means it is neither air/replaceable nor any fluid — something the source can
    // rest on. Using the existing replaceability + fluid checks keeps this consistent with how the
    // rest of the kernel classifies blocks without needing a full collision model.
    let below_is_solid =
        fluid_state(below_block).is_none() && !is_replaceable_by(kind, below_block);

    below_is_same_source || below_is_solid
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

    // --- Infinite source formation (vanilla canConvertToSource). ---

    /// A flowing water cell flanked by two water sources, resting on solid ground, converts into a
    /// new source — the core of infinite water.
    #[test]
    fn flowing_water_between_two_sources_becomes_source() {
        let mut view = MapView::new();
        view.set(p(0, 63, 0), block!("stone")); // solid support below the gap
        view.set(p(-1, 64, 0), fluid_block(FluidKind::Water, 0)); // source west
        view.set(p(1, 64, 0), fluid_block(FluidKind::Water, 0)); // source east
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 1)); // flowing in the middle

        let changes = compute_fluid_tick_vanilla(p(0, 64, 0), &view, water());
        let self_change = changes
            .iter()
            .find(|c| c.pos == p(0, 64, 0))
            .expect("middle cell should change");
        assert_eq!(
            self_change.new_block,
            fluid_block(FluidKind::Water, 0),
            "flowing water between two sources on solid ground becomes a source"
        );
    }

    /// Falling water must not overwrite a source directly below it. Regression for an infinite
    /// oscillation: a cell flanked by two sources on solid ground re-forms a source every tick
    /// (see [`flowing_water_between_two_sources_becomes_source`]); if the flowing water above then
    /// overwrites that fresh source with the thinner falling level, the two rules fight forever and
    /// the cell flips between source and flowing every tick. Downward flow may only strengthen the
    /// cell below, never weaken a source.
    #[test]
    fn falling_water_does_not_overwrite_a_source_below() {
        let mut view = MapView::new();
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0)); // source below
        view.set(p(0, 65, 0), fluid_block(FluidKind::Water, 3)); // flowing water above it

        let changes = compute_fluid_tick_vanilla(p(0, 65, 0), &view, water());
        assert!(
            changes.iter().all(|c| c.pos != p(0, 64, 0)),
            "the flowing cell above must not rewrite the source below it, changes: {:?}",
            changes
        );
    }

    /// Only one adjacent source is not enough to form a new source.
    #[test]
    fn one_source_does_not_form_a_source() {
        let mut view = MapView::new();
        view.set(p(0, 63, 0), block!("stone"));
        view.set(p(-1, 64, 0), fluid_block(FluidKind::Water, 0)); // single source
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 1));

        let changes = compute_fluid_tick_vanilla(p(0, 64, 0), &view, water());
        // It is fed (stays level 1) but must NOT become a source.
        assert!(
            changes
                .iter()
                .all(|c| !(c.pos == p(0, 64, 0) && c.new_block == fluid_block(FluidKind::Water, 0))),
            "a single adjacent source must not create a new source, changes: {:?}",
            changes
        );
    }

    /// With no solid support below (the gap sits over air), two sources still do not form a new
    /// source — the cell would fall instead.
    #[test]
    fn no_support_below_prevents_source_formation() {
        let mut view = MapView::new();
        // No floor under the middle cell; sources to either side are floored so they persist.
        view.set(p(-1, 63, 0), block!("stone"));
        view.set(p(1, 63, 0), block!("stone"));
        view.set(p(-1, 64, 0), fluid_block(FluidKind::Water, 0));
        view.set(p(1, 64, 0), fluid_block(FluidKind::Water, 0));
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 1));

        let changes = compute_fluid_tick_vanilla(p(0, 64, 0), &view, water());
        // It should flow down rather than convert to a source.
        assert!(
            changes
                .iter()
                .all(|c| !(c.pos == p(0, 64, 0) && c.new_block == fluid_block(FluidKind::Water, 0))),
            "no support below must prevent source formation, changes: {:?}",
            changes
        );
        assert!(
            changes.iter().any(|c| c.pos == p(0, 63, 0)),
            "the unsupported cell should flow downward instead"
        );
    }

    /// Lava never forms infinite sources in the overworld, even flanked by two lava sources.
    #[test]
    fn lava_never_forms_a_source() {
        let lava = FluidRules::for_kind(FluidKind::Lava, Dimension::Overworld);
        let mut view = MapView::new();
        view.set(p(0, 63, 0), block!("stone"));
        view.set(p(-1, 64, 0), fluid_block(FluidKind::Lava, 0));
        view.set(p(1, 64, 0), fluid_block(FluidKind::Lava, 0));
        view.set(p(0, 64, 0), fluid_block(FluidKind::Lava, 2));

        let changes = compute_fluid_tick_vanilla(p(0, 64, 0), &view, lava);
        assert!(
            changes
                .iter()
                .all(|c| !(c.pos == p(0, 64, 0) && c.new_block == fluid_block(FluidKind::Lava, 0))),
            "overworld lava must never form an infinite source, changes: {:?}",
            changes
        );
    }

    /// A flowing cell already at the maximum spread level must NOT spread further. Regression for
    /// an edge-oscillation bug: when `spread_level` clamped to the cap, a max-level edge cell would
    /// repeatedly spread max-level water into a neighbour that then dried up, flip-flopping every
    /// tick forever. The fed level must exceed the cap so the spread is rejected.
    #[test]
    fn max_level_cell_does_not_spread() {
        let rules = water();
        // A level-7 (max) flowing cell, fed by a level-6 neighbour to the west so it is stable,
        // on solid ground with open air to the east. It must not spread east (7 + 1 = 8 > cap).
        let mut view = MapView::new();
        view.set(p(-1, 64, 0), fluid_block(FluidKind::Water, 6));
        view.set(
            p(0, 64, 0),
            fluid_block(FluidKind::Water, rules.max_spread_level),
        );
        view.set(p(0, 63, 0), block!("stone"));
        view.set(p(1, 63, 0), block!("stone")); // floor east so spread (not fall) would be tested

        let changes = compute_fluid_tick_vanilla(p(0, 64, 0), &view, rules);
        assert!(
            changes.iter().all(|c| c.pos != p(1, 64, 0)),
            "max-level water must not spread to its neighbour, changes: {:?}",
            changes
        );
    }

    /// On a symmetric flat platform with no hole in range, all four directions must tie and the
    /// source spreads to all four. Regression for a steering bug where the recursive slope search
    /// (no visited set) returned non-minimal, asymmetric distances and dropped two of the four
    /// symmetric directions.
    #[test]
    fn symmetric_flat_spreads_all_four_directions() {
        let r = water();
        let mut view = MapView::new();
        for x in -3..=3 {
            for z in -3..=3 {
                view.set(p(x, 63, z), block!("stone"));
            }
        }
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0));

        let changes = compute_fluid_tick_vanilla(p(0, 64, 0), &view, r);
        let dirs: std::collections::HashSet<(i32, i32)> =
            changes.iter().map(|c| (c.pos.pos.x, c.pos.pos.z)).collect();
        for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            assert!(
                dirs.contains(&d),
                "missing spread direction {:?}: {:?}",
                d,
                dirs
            );
        }
        assert_eq!(
            dirs.len(),
            4,
            "should spread exactly four ways on symmetric flat ground"
        );
    }

    /// Two holes at different distances: water steers to the NEARER one only. Confirms the BFS
    /// returns true shortest-path distances.
    #[test]
    fn steers_to_nearer_of_two_holes() {
        let r = water();
        let mut view = MapView::new();
        // East-west corridor floored at y=63, walls at z=+-1.
        for x in -4..=4 {
            view.set(p(x, 63, 0), block!("stone"));
            view.set(p(x, 64, -1), block!("stone"));
            view.set(p(x, 64, 1), block!("stone"));
        }
        view.set(p(5, 64, 0), block!("stone"));
        view.set(p(-5, 64, 0), block!("stone"));
        // East hole at x=2 (distance 2); west hole at x=-3 (distance 3).
        view.blocks.remove(&(2, 63, 0));
        view.blocks.remove(&(-3, 63, 0));
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0));

        let changes = compute_fluid_tick_vanilla(p(0, 64, 0), &view, r);
        assert!(
            changes.iter().any(|c| c.pos == p(1, 64, 0)),
            "should spread east toward the nearer hole, {:?}",
            changes
        );
        assert!(
            changes.iter().all(|c| c.pos != p(-1, 64, 0)),
            "should NOT spread west toward the farther hole, {:?}",
            changes
        );
    }

    /// A hole at *exactly* the slope search limit must still be preferred over flat directions.
    /// Regression: `slope_distance` used to return `limit` both for "hole found at limit" and for
    /// "no hole within range", so a source 4 blocks from a hole (with water's limit = 4) tied the
    /// hole direction with the three flat directions and spread uniformly instead of steering.
    #[test]
    fn hole_at_exactly_the_limit_still_steers() {
        let r = water();
        assert_eq!(r.slope_find_distance, 4);

        // Large floor so the floor edges are far outside the search range (no phantom holes).
        let mut view = MapView::new();
        for x in -10..=10 {
            for z in -10..=10 {
                view.set(p(x, 63, z), block!("stone"));
            }
        }
        // Single hole exactly 4 blocks east of the source.
        view.blocks.remove(&(4, 63, 0));
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0));

        // The source must steer ONLY east (+x); the other three directions have no hole in range.
        let dists = debug_spread_distances(p(0, 64, 0), &view, r);
        assert_eq!(
            dists[0],
            Some(4),
            "east should find the hole at distance 4: {:?}",
            dists
        );
        assert_eq!(
            dists[1],
            Some(NO_HOLE),
            "west has no hole in range: {:?}",
            dists
        );
        assert_eq!(
            dists[2],
            Some(NO_HOLE),
            "+z has no hole in range: {:?}",
            dists
        );
        assert_eq!(
            dists[3],
            Some(NO_HOLE),
            "-z has no hole in range: {:?}",
            dists
        );

        let changes = compute_fluid_tick_vanilla(p(0, 64, 0), &view, r);
        assert!(
            changes.iter().any(|c| c.pos == p(1, 64, 0)),
            "source must spread east toward the limit-distance hole: {:?}",
            changes
        );
        assert!(
            changes
                .iter()
                .all(|c| c.pos != p(-1, 64, 0) && c.pos != p(0, 64, 1) && c.pos != p(0, 64, -1)),
            "source must NOT spread to the holeless directions: {:?}",
            changes
        );
    }
}
