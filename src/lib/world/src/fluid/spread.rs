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
//! * A block is a *source* (level 0) or *flowing* (level 1..=`max_spread_level`).
//! * If the cell below is replaceable, fluid flows straight down. Downward flow always produces a
//!   near-source flowing block, so a falling column stays full.
//! * Otherwise, fluid spreads horizontally to each replaceable neighbour whose resulting level
//!   would be lower than that neighbour's current level, decrementing the level by one step each
//!   block. Once the level passes the cap, the fluid no longer spreads horizontally.
//! * A flowing (non-source) block re-derives its own level each tick from its strongest remaining
//!   feeder (`min feeder level + step`, vanilla-style). If that feeder is removed it recedes one
//!   step at a time rather than vanishing, and only dries up (turns to air) once no feeder can
//!   support any level within the cap. (Upstream feeders = a same-fluid source/flow above, or a
//!   horizontally adjacent same-fluid block.)
//! * When lava and water meet, the lava solidifies (vanilla's two mechanics): a lava block with
//!   water above or beside it hardens in place — a *source* into obsidian, *flowing* lava into
//!   cobblestone — and lava flowing *down* onto a water cell turns that water into stone. Water is
//!   never the block that changes. Each solidification flags a "fizz" effect for the caller to play
//!   the lava-extinguish sound.
//!
//! Water and lava share this logic; they differ only in [`crate::fluid::FluidKind`]-derived
//! parameters supplied by the caller (spread distance, level step and tick delay).
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
    /// Whether this change is a lava/water solidification, which the client should accompany with
    /// the lava-extinguish "fizz" sound and smoke (LevelEvent 1501). The caller is responsible for
    /// emitting that effect; the algorithm just flags which changes are reactions.
    pub fizz: bool,
}

impl FluidChange {
    /// A normal (non-reaction) fluid change: places `new_block` at `pos`, optionally rescheduling.
    pub fn flow(pos: BlockPos, new_block: BlockStateId, reschedule: bool) -> Self {
        Self {
            pos,
            new_block,
            reschedule,
            fizz: false,
        }
    }

    /// A lava/water solidification change: places rock at `pos` and flags the fizz effect. Rock is
    /// not fluid, so it never reschedules itself.
    pub fn reaction(pos: BlockPos, new_block: BlockStateId) -> Self {
        Self {
            pos,
            new_block,
            reschedule: false,
            fizz: true,
        }
    }
}

/// Returns true if fluid of `kind` may flow into / overwrite this block.
///
/// Air (and void air) is always replaceable. Same-fluid blocks are replaceable (the caller further
/// checks whether the flow would actually strengthen them). The *opposite* fluid is intentionally
/// not replaceable here: lava meeting water is resolved by the solidification reaction, not by one
/// fluid overwriting the other. Solid blocks are never replaceable.
pub(crate) fn is_replaceable_by(kind: FluidKind, block: BlockStateId) -> bool {
    if block == block!("air") || block == block!("void_air") {
        return true;
    }
    match fluid_state(block) {
        Some(state) => state.kind == kind,
        None => false,
    }
}

/// The six axis-aligned neighbours of `pos`, plus convenience accessors.
fn below(pos: BlockPos) -> BlockPos {
    pos + (0, -1, 0)
}
fn above(pos: BlockPos) -> BlockPos {
    pos + (0, 1, 0)
}
pub(crate) const HORIZONTAL: [(i32, i32, i32); 4] = [(1, 0, 0), (-1, 0, 0), (0, 0, 1), (0, 0, -1)];

/// The six axis-aligned neighbours of `pos` (the cells a fluid tick at `pos` can influence).
///
/// The caller uses this to propagate updates: after applying a change at `pos`, every neighbour
/// that currently holds fluid should be re-ticked so a recede/spread/solidify ripples outward
/// instead of stopping at a single ring. This is the FerrumC analogue of vanilla's neighbour
/// block updates.
pub fn fluid_neighbours(pos: BlockPos) -> [BlockPos; 6] {
    [
        above(pos),
        below(pos),
        pos + HORIZONTAL[0],
        pos + HORIZONTAL[1],
        pos + HORIZONTAL[2],
        pos + HORIZONTAL[3],
    ]
}

/// The block a lava cell hardens into when water is in contact from the side or above (the
/// vanilla `shouldSpreadLiquid` set: up + horizontals, never directly below).
///
/// A lava *source* becomes obsidian; *flowing* lava becomes cobblestone. (The "stone" result is a
/// different mechanic — lava flowing *down* onto water — handled in [`down_flow_reaction`].)
fn lava_harden_in_place(lava: FluidState) -> BlockStateId {
    debug_assert!(lava.kind == FluidKind::Lava);
    if lava.is_source() {
        block!("obsidian")
    } else {
        block!("cobblestone")
    }
}

/// If the lava at `pos` should harden *in place* this tick, returns the rock it becomes.
///
/// Mirrors vanilla: lava checks the five cells `{above, N, S, E, W}` (deliberately excluding the
/// cell directly below) for water. Water below is handled by the down-flow path instead, so that a
/// lava-above/water-below column solidifies the lower (water) cell into stone, not the upper lava.
pub(crate) fn lava_harden_check<V: BlockView>(
    pos: BlockPos,
    lava: FluidState,
    view: &V,
) -> Option<BlockStateId> {
    let touches_water = std::iter::once(above(pos))
        .chain(HORIZONTAL.iter().map(|&o| pos + o))
        .any(|n| matches!(fluid_state(view.block_at(n)), Some(s) if s.kind == FluidKind::Water));
    touches_water.then(|| lava_harden_in_place(lava))
}

/// If lava at `pos` sits directly above a water cell, returns the [`FluidChange`] that turns that
/// water cell into stone (the classic "stone generator"). Shared by both spread kernels.
pub(crate) fn lava_water_down_reaction<V: BlockView>(
    pos: BlockPos,
    view: &V,
) -> Option<FluidChange> {
    let below_pos = below(pos);
    matches!(fluid_state(view.block_at(below_pos)), Some(s) if s.kind == FluidKind::Water)
        .then(|| FluidChange::reaction(below_pos, block!("stone")))
}

/// Computes the lowest level a flowing block of `kind` could legitimately hold this tick, given its
/// surroundings, or `None` if no feeder can support any level (so it should disappear).
///
/// Feeders are same-fluid blocks that can push into `pos`:
/// * fluid directly above feeds a full (near-source) column, pinning this block to `level_step`;
/// * each horizontally adjacent same-fluid block feeds `its level + level_step`.
///
/// The result is the strongest (lowest) feed that still fits within `max_spread_level`.
fn supported_level<V: BlockView>(
    pos: BlockPos,
    kind: FluidKind,
    rules: FluidRules,
    view: &V,
) -> Option<u8> {
    // Fluid directly above => behaves like a falling column: stays near-source.
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

/// Returns true if a fluid tick at `pos` would immediately produce a lava/water solidification.
///
/// Used by the scheduler to give lava that has just come into contact with water a fast follow-up
/// tick, so the reaction resolves almost immediately (as in vanilla) instead of waiting out lava's
/// slow 30-tick cadence. Only lava is ever the reacting party.
pub fn would_react<V: BlockView>(pos: BlockPos, view: &V) -> bool {
    let Some(state) = fluid_state(view.block_at(pos)) else {
        return false;
    };
    if state.kind != FluidKind::Lava {
        return false;
    }
    // In-place harden (water above / beside) or down-flow onto water below.
    lava_harden_check(pos, state, view).is_some()
        || matches!(fluid_state(view.block_at(below(pos))), Some(s) if s.kind == FluidKind::Water)
}

/// Computes the changes a single fluid block produces on its tick.
///
/// `pos` must currently contain a fluid (otherwise an empty change set is returned). `rules`
/// supplies the per-fluid, per-dimension parameters (level step, max spread level); the caller is
/// expected to look these up via [`FluidRules::for_kind`] using the block at `pos`.
///
/// The returned changes are *real* block mutations only — this function never emits no-op
/// "neighbour wake" entries. Propagation is a scheduling concern owned by the caller: after
/// applying these changes it must re-tick the fluid neighbours of every changed position (see
/// [`fluid_neighbours`]). Doing it that way keeps conflict resolution (e.g. lava solidifying vs a
/// neighbour's flow) from being polluted by no-op writes.
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

    // --- Lava/water interaction takes precedence over everything else. ---
    // Only lava is the active party; water keeps flowing normally. Both reaction checks run before
    // the recede/flow logic, so contact with water always resolves first (even for lava that would
    // otherwise dry up or recede this tick).
    if kind == FluidKind::Lava {
        // (a) In-place harden: lava with water above or beside it solidifies where it stands.
        //     Source -> obsidian, flowing -> cobblestone. The cell directly below is deliberately
        //     excluded; that is the down-flow case.
        if let Some(rock) = lava_harden_check(pos, state, view) {
            changes.push(FluidChange::reaction(pos, rock));
            return changes;
        }
        // (b) Down-flow harden: lava sitting directly above a water cell turns that water into
        //     stone (the classic "stone generator"). The water cell is the one that changes.
        let below_pos = below(pos);
        if matches!(fluid_state(view.block_at(below_pos)), Some(s) if s.kind == FluidKind::Water) {
            changes.push(FluidChange::reaction(below_pos, block!("stone")));
            return changes;
        }
    }

    // --- A flowing block re-derives its level from its feeders. ---
    // Sources are fixed. Flowing blocks recede one step (or dry up) when their feed weakens, and
    // strengthen when a stronger feed appears. The caller wakes the neighbours so the adjustment
    // ripples through the whole body of fluid instead of stopping at one ring.
    if !state.is_source() {
        match supported_level(pos, kind, rules, view) {
            None => {
                // No feeder can support any level: dry up.
                changes.push(FluidChange::flow(pos, block!("air"), false));
                return changes;
            }
            Some(target) if target != state.level => {
                // Level changed (receding or strengthening). We do not also spread this tick; the
                // block re-ticks (and neighbours are woken) so it settles one step at a time,
                // matching vanilla's gradual recede.
                changes.push(FluidChange::flow(pos, fluid_block(kind, target), true));
                return changes;
            }
            Some(_) => { /* level unchanged: fall through to normal flow */ }
        }
    }

    let effective_level = state.level;

    // --- Vertical flow takes priority. ---
    let below_pos = below(pos);
    let below_block = view.block_at(below_pos);
    if is_replaceable_by(kind, below_block) {
        // Falling fluid stays effectively full so columns do not thin out as they fall.
        let falling = fluid_block(kind, rules.level_step.min(rules.max_spread_level));
        if below_block != falling {
            changes.push(FluidChange::flow(below_pos, falling, true));
        }
        // When fluid can fall, vanilla does not also spread horizontally from this block.
        return changes;
    }

    // --- Horizontal spread. ---
    let next_level = effective_level.saturating_add(rules.level_step);
    if next_level > rules.max_spread_level {
        return changes;
    }

    let outflow = fluid_block(kind, next_level);
    for &offset in HORIZONTAL.iter() {
        let n_pos = pos + offset;
        let n_block = view.block_at(n_pos);
        if !is_replaceable_by(kind, n_block) {
            continue;
        }
        // Only flow in if it would make the neighbour stronger (lower level) than it currently is.
        let improves = match fluid_state(n_block) {
            Some(n_state) if n_state.kind == kind => next_level < n_state.level,
            Some(_) => false, // opposite fluid: handled by the reaction path, not here
            None => true,     // air: always flow in
        };
        if improves {
            changes.push(FluidChange::flow(n_pos, outflow, true));
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
    fn flowing_spreads_at_its_level_plus_step() {
        let mut view = MapView::new();
        // A level-2 flowing block fed by a level-1 neighbour to the west (so it is stable at 2),
        // on solid ground. It should spread at level 3 into its other open neighbours.
        view.set(p(-1, 64, 0), fluid_block(FluidKind::Water, 1));
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 2));
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, water_rules());
        let spread: Vec<_> = changes
            .iter()
            .filter(|c| c.new_block == fluid_block(FluidKind::Water, 3))
            .collect();
        assert_eq!(
            spread.len(),
            3,
            "should spread at level 3 into 3 open neighbours"
        );
        // The feeder is never overwritten.
        assert!(changes.iter().all(|c| c.pos != p(-1, 64, 0)));
    }

    #[test]
    fn does_not_spread_past_max_level() {
        let mut view = MapView::new();
        // A level-7 block fed by a level-6 neighbour to the west (stable at 7) on solid ground.
        // The next step would be 8 > 7, so it must neither recede nor spread.
        let rules = water_rules();
        view.set(p(-1, 64, 0), fluid_block(FluidKind::Water, 6));
        view.set(
            p(0, 64, 0),
            fluid_block(FluidKind::Water, rules.max_spread_level),
        );
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, rules);
        assert!(
            changes.is_empty(),
            "fluid at max level should not spread further, got {:?}",
            changes
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

        // Overworld lava: level_step = 2, max_spread_level = 7. A level-6 block fed by a level-4
        // neighbour (4 + 2 = 6, stable) must not spread further, because 6 + 2 = 8 > 7.
        let lava = FluidRules::for_kind(FluidKind::Lava, Dimension::Overworld);
        assert_eq!(lava.level_step, 2);

        let mut view = MapView::new();
        view.set(p(-1, 64, 0), fluid_block(FluidKind::Lava, 4)); // horizontal feeder
        view.set(p(0, 64, 0), fluid_block(FluidKind::Lava, 6));
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, lava);
        assert!(
            changes.is_empty(),
            "overworld lava at level 6 should stop (next would be 8, > 7), got {:?}",
            changes
        );
    }

    #[test]
    fn nether_lava_spreads_like_water() {
        use crate::dimension::Dimension;

        // Nether lava: level_step = 1, max_spread_level = 7 — same reach as water. A level-6 block
        // fed by a level-5 neighbour (stable at 6) can still take one more step to level 7.
        let lava = FluidRules::for_kind(FluidKind::Lava, Dimension::Nether);
        assert_eq!(lava.level_step, 1);

        let mut view = MapView::new();
        view.set(p(-1, 64, 0), fluid_block(FluidKind::Lava, 5)); // horizontal feeder
        view.set(p(0, 64, 0), fluid_block(FluidKind::Lava, 6));
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, lava);
        let spread: Vec<_> = changes
            .iter()
            .filter(|c| c.new_block == fluid_block(FluidKind::Lava, 7))
            .collect();
        // West is the feeder; the other three horizontal neighbours get level 7.
        assert_eq!(spread.len(), 3);
    }

    // --- Receding behaviour (the source-removal scenario). ---

    #[test]
    fn flowing_recedes_one_level_when_feeder_weakens() {
        let mut view = MapView::new();
        // A level-1 block whose only feeder is a level-3 neighbour to the west. The strongest feed
        // is 3 + 1 = 4, so this block should recede from 1 to 4 (weaker), not vanish.
        view.set(p(-1, 64, 0), fluid_block(FluidKind::Water, 3));
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 1));
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, water_rules());
        let self_change = changes
            .iter()
            .find(|c| c.pos == p(0, 64, 0))
            .expect("the block should re-derive its level");
        assert_eq!(
            self_change.new_block,
            fluid_block(FluidKind::Water, 4),
            "should recede to feeder level + step, not dry up"
        );
        assert!(self_change.reschedule);
    }

    #[test]
    fn recede_only_changes_self_now_caller_wakes_neighbours() {
        let mut view = MapView::new();
        // Level-1 block with feeders: level-3 to the west (feeds 4) and level-2 to the east
        // (feeds 3). The strongest (lowest) feed wins, so it re-derives to level 3. The pure
        // function changes only this block; waking neighbours is the caller's responsibility, so
        // no neighbour entries appear here.
        view.set(p(-1, 64, 0), fluid_block(FluidKind::Water, 3));
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 1));
        view.set(p(1, 64, 0), fluid_block(FluidKind::Water, 2));
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, water_rules());
        assert_eq!(
            changes.len(),
            1,
            "only the ticking block changes: {:?}",
            changes
        );
        assert_eq!(changes[0].pos, p(0, 64, 0));
        assert_eq!(
            changes[0].new_block,
            fluid_block(FluidKind::Water, 3),
            "re-derives to strongest feeder (east level 2 -> 3)"
        );
        assert!(changes[0].reschedule);
    }

    #[test]
    fn isolated_flowing_block_dries_up() {
        let mut view = MapView::new();
        // A level-1 block whose only fluid neighbour is at the max level (7 + 1 = 8 > cap, so it
        // cannot feed). With no valid feeder the block dries up. (Neighbour waking is the caller's
        // job and is verified in the integration tests.)
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 1));
        view.set(p(1, 64, 0), fluid_block(FluidKind::Water, 7));
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, water_rules());
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].pos, p(0, 64, 0));
        assert_eq!(changes[0].new_block, block!("air"));
        assert!(!changes[0].reschedule);
    }

    // --- Lava/water interaction. ---
    //
    // Vanilla splits this into two mechanics:
    //   * in-place harden — lava with water above or beside it (never just below): source ->
    //     obsidian, flowing -> cobblestone;
    //   * down-flow — lava directly above water turns that water into stone.

    #[test]
    fn lava_source_with_water_beside_becomes_obsidian() {
        use crate::dimension::Dimension;
        let lava = FluidRules::for_kind(FluidKind::Lava, Dimension::Overworld);

        let mut view = MapView::new();
        view.set(p(0, 64, 0), fluid_block(FluidKind::Lava, 0)); // lava source
        view.set(p(1, 64, 0), fluid_block(FluidKind::Water, 3)); // water beside it

        let changes = compute_fluid_tick(p(0, 64, 0), &view, lava);
        let self_change = changes.iter().find(|c| c.pos == p(0, 64, 0)).unwrap();
        assert_eq!(self_change.new_block, block!("obsidian"));
        assert!(
            self_change.fizz,
            "solidification should flag the fizz effect"
        );
    }

    #[test]
    fn lava_source_with_water_above_becomes_obsidian() {
        use crate::dimension::Dimension;
        let lava = FluidRules::for_kind(FluidKind::Lava, Dimension::Overworld);

        // Water directly above the lava source: the lava (lower cell) hardens to obsidian.
        let mut view = MapView::new();
        view.set(p(0, 65, 0), fluid_block(FluidKind::Water, 0)); // water above
        view.set(p(0, 64, 0), fluid_block(FluidKind::Lava, 0)); // lava source below

        let changes = compute_fluid_tick(p(0, 64, 0), &view, lava);
        let self_change = changes.iter().find(|c| c.pos == p(0, 64, 0)).unwrap();
        assert_eq!(self_change.new_block, block!("obsidian"));
    }

    #[test]
    fn flowing_lava_with_water_beside_becomes_cobblestone() {
        use crate::dimension::Dimension;
        let lava = FluidRules::for_kind(FluidKind::Lava, Dimension::Overworld);

        // Flowing lava beside water always hardens to cobblestone, regardless of the water's level
        // (this is the in-place harden rule, not the down-flow "stone" rule).
        for water_level in [0u8, 5] {
            let mut view = MapView::new();
            view.set(p(0, 64, 0), fluid_block(FluidKind::Lava, 2)); // flowing lava
            view.set(p(1, 64, 0), fluid_block(FluidKind::Water, water_level));

            let changes = compute_fluid_tick(p(0, 64, 0), &view, lava);
            let self_change = changes.iter().find(|c| c.pos == p(0, 64, 0)).unwrap();
            assert_eq!(
                self_change.new_block,
                block!("cobblestone"),
                "flowing lava beside water (level {water_level}) should be cobblestone"
            );
        }
    }

    #[test]
    fn lava_flowing_down_onto_water_turns_the_water_to_stone() {
        use crate::dimension::Dimension;
        let lava = FluidRules::for_kind(FluidKind::Lava, Dimension::Overworld);

        // Lava (upper) directly above water (lower). Ticking the lava must turn the LOWER cell
        // (the water) into stone — not the upper lava.
        let mut view = MapView::new();
        view.set(p(0, 65, 0), fluid_block(FluidKind::Lava, 2)); // flowing lava above
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0)); // water source below

        let changes = compute_fluid_tick(p(0, 65, 0), &view, lava);
        let lower = changes
            .iter()
            .find(|c| c.pos == p(0, 64, 0))
            .expect("the lower (water) cell should change");
        assert_eq!(lower.new_block, block!("stone"));
        assert!(lower.fizz);
        // The upper lava itself is not turned to rock by this rule.
        assert!(changes.iter().all(|c| !(c.pos == p(0, 65, 0) && c.fizz)));
    }

    #[test]
    fn water_touching_lava_is_unaffected_itself() {
        // The reaction only solidifies lava; the water block keeps behaving like water. Ticking
        // the water beside lava should not turn the water into rock, nor flow into the lava cell.
        let mut view = MapView::new();
        view.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0)); // water source
        view.set(p(1, 64, 0), fluid_block(FluidKind::Lava, 2)); // flowing lava beside it
        view.set(p(0, 63, 0), block!("stone"));

        let changes = compute_fluid_tick(p(0, 64, 0), &view, water_rules());
        assert!(changes.iter().all(|c| c.pos != p(0, 64, 0)));
        assert!(
            changes.iter().all(|c| c.pos != p(1, 64, 0)),
            "water should not flow into the opposite fluid; that is the reaction's job"
        );
    }

    #[test]
    fn water_above_lava_hardens_the_lava_not_a_lower_cell() {
        use crate::dimension::Dimension;
        let lava = FluidRules::for_kind(FluidKind::Lava, Dimension::Overworld);

        // Symmetry check for the "下面的流体总是变化" report: water above flowing lava must harden
        // the lava (the cell with water above), here at y=64, into cobblestone.
        let mut view = MapView::new();
        view.set(p(0, 65, 0), fluid_block(FluidKind::Water, 0)); // water above
        view.set(p(0, 64, 0), fluid_block(FluidKind::Lava, 2)); // flowing lava below
        view.set(p(0, 63, 0), block!("stone")); // solid floor so lava can't fall

        let changes = compute_fluid_tick(p(0, 64, 0), &view, lava);
        let self_change = changes.iter().find(|c| c.pos == p(0, 64, 0)).unwrap();
        assert_eq!(self_change.new_block, block!("cobblestone"));
    }
}
