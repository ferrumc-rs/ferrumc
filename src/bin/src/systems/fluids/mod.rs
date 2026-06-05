//! Fluid simulation systems.
//!
//! This module wires the decoupled fluid-spreading algorithm in
//! [`ferrumc_world::fluid::spread`] into the running server:
//!
//! 1. Block placements and breaks *seed* the scheduler with fluid ticks (see
//!    [`seed_fluid_tick`] and its callers in the packet handlers).
//! 2. Once per game tick, [`process_fluid_ticks`] drains the ticks due this tick, evaluates each
//!    one against the live world, applies the resulting block changes, re-schedules affected
//!    blocks, and broadcasts the changes to nearby players.
//!
//! The algorithm itself knows nothing about chunks, the ECS, or networking; this module provides
//! the world-access glue ([`WorldBlockView`]) and side effects.

use bevy_ecs::prelude::MessageReader;
use bevy_ecs::prelude::{Entity, Query, Res, ResMut, Resource};
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::tick::TickCounter;
use ferrumc_core::transform::position::Position;
use ferrumc_messages::BlockBrokenEvent;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net::packets::outgoing::level_event::LevelEventPacket;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::{GlobalState, GlobalStateResource};
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::dimension::Dimension;
use ferrumc_world::fluid::is_fluid;
use ferrumc_world::fluid::spread::{fluid_neighbours, would_react, BlockView, FluidChange};
use ferrumc_world::fluid::{fluid_state, FluidKind, FluidRules};
use ferrumc_world::pos::{BlockPos, ChunkPos};
use ferrumc_world::scheduler::{BlockTickScheduler, ScheduledTick, TickKind};
use std::collections::{HashMap, HashSet};
use tracing::{error, trace};

/// Delay (in ticks) before a lava block that has just touched water resolves its solidification.
///
/// Vanilla reacts essentially on the next block update; a 1-tick delay keeps that "instant on
/// contact" feel rather than making lava wait out its slow 30-tick spread cadence.
const REACTION_DELAY: u64 = 1;

/// Minimum number of due ticks in a batch before the parallel evaluation path is used.
///
/// Below this threshold the overhead of dispatching work to the thread pool outweighs the benefit,
/// so the batch is evaluated serially on the calling thread. Small fluid disturbances (a single
/// placed bucket) stay on the fast serial path; large flows fan out across cores.
pub const PARALLEL_THRESHOLD: usize = 64;

/// The dimension the fluid system simulates against.
///
/// The world currently only tracks the overworld; this resource is the single seam that lets
/// [`FluidRules`] pick the right per-dimension parameters (overworld lava is slow and short-range,
/// Nether lava is fast and long-range, etc.). Once multi-dimension support lands, this can become
/// a per-world value, or the systems can iterate dimensions, without changing the algorithm.
#[derive(Resource, Clone, Copy, Debug)]
pub struct ActiveDimension(pub Dimension);

impl Default for ActiveDimension {
    fn default() -> Self {
        Self(Dimension::Overworld)
    }
}

impl ActiveDimension {
    #[inline]
    fn name(self) -> &'static str {
        self.0.as_str()
    }
}

/// Looks up the fluid rules for the block at `pos`, if any. Returns `None` for non-fluid blocks.
#[inline]
fn rules_for_block(block: BlockStateId, dim: Dimension) -> Option<FluidRules> {
    fluid_state(block).map(|s| FluidRules::for_kind(s.kind, dim))
}

/// ECS resource wrapping the per-chunk block tick scheduler.
///
/// The wrapper lives in the binary because the `ferrumc-world` crate intentionally does not depend
/// on `bevy_ecs`; the scheduler itself is a plain data structure.
#[derive(Resource, Default)]
pub struct FluidScheduler(pub BlockTickScheduler);

/// Debug control for the fluid simulation clock.
///
/// Lets `/tick freeze` / `/tick step` / `/tick run` pause and single-step fluid spreading so the
/// (potentially huge) cascade of a single placement can be inspected one game tick at a time
/// without drowning in logs. Only the fluid system consults this; the rest of the server keeps
/// running normally, so players can still move and place blocks while fluids are frozen.
#[derive(Default, Resource)]
pub struct FluidTickControl {
    /// When true, fluid ticks do not advance unless `steps` is positive.
    pub frozen: bool,
    /// Number of single steps queued up while frozen. Each processed tick decrements this.
    pub steps: u32,
}

impl FluidTickControl {
    /// Returns true if a fluid tick is allowed to run this game tick, consuming one queued step if
    /// the simulation is frozen.
    fn allow_tick(&mut self) -> bool {
        if !self.frozen {
            return true;
        }
        if self.steps > 0 {
            self.steps -= 1;
            true
        } else {
            false
        }
    }
}

/// How many freshly loaded chunks [`settle_loaded_fluids`] scans per tick. Bounds the one-time settle
/// cost so a burst of newly loaded chunks (a player joining or flying) cannot overrun the tick; any
/// excess chunks are picked up on later ticks.
const SETTLE_CHUNKS_PER_TICK: usize = 4;

/// Tracks which chunks have already had their generated fluids settled, so each is scanned at most
/// once. Keyed by chunk coordinates, matching [`ChunkReceiver::loaded`].
///
/// The set only grows as the world is explored (bounded by the explored area). Pruning entries when a
/// chunk unloads everywhere is a possible future refinement; re-settling an already-flowed chunk is
/// harmless (its fluids are already in equilibrium) but wasteful, which is what this set avoids.
#[derive(Resource, Default)]
pub struct FluidSettleTracker {
    settled: HashSet<(i32, i32)>,
}

/// Settles "hanging" fluids in chunks that have newly loaded near a player.
///
/// Terrain generation places fluids but never runs the simulation, so a cave that breached an ocean
/// (or, later, a spring perched on a ledge) leaves fluid frozen mid-air until something ticks it.
/// This system scans each newly loaded chunk once for fluid cells bordering open space
/// ([`ferrumc_world::fluid::settle::fluid_frontier_cells`]) and seeds a one-off fluid tick for each,
/// so the fluid flows the first time a player is near — mirroring vanilla's settle-on-load — while
/// leaving the (already-settled) fluid interior untouched. A per-tick chunk budget
/// ([`SETTLE_CHUNKS_PER_TICK`]) keeps the scan from overrunning the tick when many chunks load at
/// once.
pub fn settle_loaded_fluids(
    query: Query<&ChunkReceiver>,
    mut tracker: ResMut<FluidSettleTracker>,
    mut scheduler: ResMut<FluidScheduler>,
    tick: Res<TickCounter>,
    state: Res<GlobalStateResource>,
    dim: Res<ActiveDimension>,
) {
    let current = tick.get();
    let dim = *dim;
    let mut budget = SETTLE_CHUNKS_PER_TICK;

    'outer: for receiver in query.iter() {
        for &coords in receiver.loaded.iter() {
            if budget == 0 {
                break 'outer;
            }
            // Claim the chunk; skip if another player (or an earlier tick) already settled it.
            if !tracker.settled.insert(coords) {
                continue;
            }

            let pos = ChunkPos::new(coords.0, coords.1);
            // Compute the frontier in a tight scope so the chunk read guard is released before
            // seeding (which itself loads chunks through the world view — holding the guard across it
            // could re-enter the same DashMap shard and deadlock).
            let frontier = {
                let chunk =
                    match ferrumc_utils::world::load_or_generate_chunk(&state.0, pos, dim.name()) {
                        Ok(chunk) => chunk,
                        Err(e) => {
                            error!("Failed to load chunk {:?} for fluid settle: {}", coords, e);
                            tracker.settled.remove(&coords); // allow a retry on a later tick
                            continue;
                        }
                    };
                ferrumc_world::fluid::settle::fluid_frontier_cells(&chunk, pos)
            };

            for cell in frontier {
                seed_fluid_tick(&mut scheduler.0, &state.0, dim, current, cell);
            }
            budget -= 1;
        }
    }
}

/// A [`BlockView`] backed by the live world.
///
/// Holds an owned [`GlobalState`] handle (an `Arc` clone) so it can be moved into worker threads
/// during the parallel read phase. Reads go through the chunk cache via a shared (read) lock,
/// which is safe to perform concurrently from multiple threads.
struct WorldBlockView {
    state: GlobalState,
    dim: ActiveDimension,
}

impl BlockView for WorldBlockView {
    fn block_at(&self, pos: BlockPos) -> BlockStateId {
        match ferrumc_utils::world::load_or_generate_chunk(
            &self.state,
            pos.chunk(),
            self.dim.name(),
        ) {
            Ok(chunk) => chunk.get_block(pos.chunk_block_pos()),
            Err(err) => {
                // A failed chunk load is treated as air so the algorithm degrades gracefully
                // rather than panicking inside the tick loop.
                trace!("[fluid] failed to read block at {:?}: {}", pos.pos, err);
                BlockStateId::default()
            }
        }
    }
}

/// Schedules a fluid tick for `pos` if it currently contains fluid.
///
/// The delay comes from [`FluidRules::for_kind`] for whatever fluid is at `pos`, so water and
/// lava (and Nether vs overworld lava) tick at their own cadences.
///
/// Schedules a fluid tick for `pos` if it currently contains fluid, picking the right delay.
///
/// The delay is normally [`FluidRules::for_kind`] for whatever fluid is at `pos` (so water and
/// lava, overworld vs Nether, tick at their own cadences). But if the block is lava that is already
/// in contact with water — i.e. it would solidify this tick — it is scheduled at the fast
/// [`REACTION_DELAY`] instead, so the reaction happens essentially on contact rather than after
/// lava's slow spread cadence.
///
/// Intended to be called from block placement/break handlers and from neighbour propagation. No-op
/// for non-fluid blocks.
pub fn seed_fluid_tick(
    scheduler: &mut BlockTickScheduler,
    state: &GlobalState,
    dim: ActiveDimension,
    current_tick: u64,
    pos: BlockPos,
) {
    let view = WorldBlockView {
        state: state.clone(),
        dim,
    };
    let block = view.block_at(pos);
    let Some(rules) = rules_for_block(block, dim.0) else {
        debug_assert!(!is_fluid(block), "rules_for_block disagreed with is_fluid");
        return;
    };
    // A lava block already touching water solidifies almost immediately rather than waiting out its
    // spread cadence.
    let delay = if would_react(pos, &view) {
        REACTION_DELAY
    } else {
        rules.tick_delay
    };
    scheduler.schedule(pos, TickKind::FluidSpread, current_tick, delay);
}

/// The six axis-aligned neighbours of a block position.
fn neighbours(pos: BlockPos) -> [BlockPos; 6] {
    [
        pos + (0, 1, 0),
        pos + (0, -1, 0),
        pos + (1, 0, 0),
        pos + (-1, 0, 0),
        pos + (0, 0, 1),
        pos + (0, 0, -1),
    ]
}

/// Seeds fluid ticks around blocks that were broken this tick.
///
/// When a block is removed, any fluid bordering the now-empty space should re-evaluate so it can
/// flow into the gap. Listens to [`BlockBrokenEvent`], which is emitted by both creative and
/// survival break paths, so a single system covers both.
pub fn seed_on_block_break(
    mut events: MessageReader<BlockBrokenEvent>,
    mut scheduler: ResMut<FluidScheduler>,
    tick: Res<TickCounter>,
    state: Res<GlobalStateResource>,
    dim: Res<ActiveDimension>,
) {
    let current = tick.get();
    let dim = *dim;
    for event in events.read() {
        let pos = event.position;
        // The broken position itself (in case it is now exposed to a fluid) and its neighbours.
        seed_fluid_tick(&mut scheduler.0, &state.0, dim, current, pos);
        for neighbour in neighbours(pos) {
            seed_fluid_tick(&mut scheduler.0, &state.0, dim, current, neighbour);
        }
    }
}

/// Writes a single block change back to the world. Returns true if the world was actually
/// modified (so the caller knows to broadcast and wake neighbours), false otherwise.
///
/// Guards against two classes of bad writes:
/// * **No-op writes** — if the cell already holds `new_block` there is nothing to do. Skipping
///   these is what stops a settled fluid body from re-broadcasting and re-waking neighbours every
///   tick (the visual "keeps changing shape" symptom).
/// * **Eating solid blocks** — a fluid change must never overwrite a non-fluid, non-air block.
///   The pure kernels never target solids, but a stale read in the parallel phase (or a block
///   placed between evaluation and apply) theoretically could; refusing the write here makes that
///   impossible rather than silently destroying terrain. Reactions (`fizz`, e.g. lava→stone) are
///   allowed to replace fluid with rock, which is their whole purpose.
fn apply_change(state: &GlobalState, dim: ActiveDimension, change: &FluidChange) -> bool {
    let chunk_pos = change.pos.chunk();
    let block_pos = change.pos.chunk_block_pos();

    match ferrumc_utils::world::load_or_generate_mut(state, chunk_pos, dim.name()) {
        Ok(mut chunk) => {
            let current = chunk.get_block(block_pos);
            // Nothing to do if the block is already what we'd write.
            if current == change.new_block {
                return false;
            }
            // Refuse to overwrite a solid block with fluid. A reaction (fizz) replaces fluid with
            // rock and is fine; a plain fluid flow must only ever land on air or fluid.
            if ferrumc_world::fluid::is_fluid(change.new_block)
                && ferrumc_world::fluid::is_solid_obstacle(current)
            {
                trace!(
                    "[fluid] refused to overwrite solid block at {:?} with fluid",
                    change.pos.pos
                );
                return false;
            }
            chunk.set_block(block_pos, change.new_block);
            true
        }
        Err(err) => {
            error!(
                "[fluid] failed to apply change at {:?}: {}",
                change.pos.pos, err
            );
            false
        }
    }
}

/// Broadcasts a block change to every connected player within render distance of it.
fn broadcast_change(
    change: &FluidChange,
    state: &GlobalState,
    players: &Query<(Entity, &StreamWriter, &Position)>,
) {
    let render_distance = get_global_config().chunk_render_distance as i32;
    let target_chunk = change.pos.chunk();
    let packet = BlockUpdate {
        location: NetworkPosition {
            x: change.pos.pos.x,
            y: change.pos.pos.y as i16,
            z: change.pos.pos.z,
        },
        block_state_id: VarInt::from(change.new_block),
    };

    for (eid, conn, position) in players.iter() {
        if !state.players.is_connected(eid) {
            continue;
        }
        let player_chunk = position.chunk();
        if (target_chunk.x() - player_chunk.x).abs() <= render_distance
            && (target_chunk.z() - player_chunk.y).abs() <= render_distance
        {
            if let Err(err) = conn.send_packet_ref(&packet) {
                trace!("[fluid] failed to send block update: {:?}", err);
            }
        }
    }
}

/// Combines fluid changes that target the same block into a deterministic result.
///
/// Multiple due ticks in one batch can produce changes for the same position (for example two
/// sources flowing into the same gap). To make the outcome independent of evaluation/thread order,
/// changes are reduced per position by a fixed priority:
///
/// 1. Fluid of any level beats air/removal (a block being filled wins over being emptied).
/// 2. Between two fluids, the stronger one wins (lower `level`).
/// 3. Any remaining tie is broken by the raw block state id, which is stable.
///
/// `reschedule` is OR-ed across merged changes so a follow-up tick is kept if any contributor
/// wanted one.
fn reduce_changes(changes: Vec<FluidChange>) -> Vec<FluidChange> {
    let mut by_pos: HashMap<BlockPos, FluidChange> = HashMap::new();
    for change in changes {
        by_pos
            .entry(change.pos)
            .and_modify(|existing| {
                if change_priority(&change) > change_priority(existing) {
                    let keep_reschedule = existing.reschedule || change.reschedule;
                    *existing = change;
                    existing.reschedule = keep_reschedule;
                } else {
                    existing.reschedule = existing.reschedule || change.reschedule;
                }
            })
            .or_insert(change);
    }
    by_pos.into_values().collect()
}

/// Priority key for [`reduce_changes`]. Higher wins.
///
/// Lava/water solidifications (`fizz`) outrank everything: once a cell is set to turn into rock,
/// no competing fluid flow at the same position may override it (this is what keeps a down-flow
/// "stone" from being clobbered by the water cell's own recede in the same batch). Below that,
/// fluids outrank non-fluids, and among fluids a lower level (stronger) wins; ties fall back to the
/// raw id for stability.
fn change_priority(change: &FluidChange) -> (u8, i32, u32) {
    if change.fizz {
        return (3, 0, change.new_block.raw());
    }
    match fluid_state(change.new_block) {
        // Fluid: middle tier (2). Stronger (lower level) should win, so negate the level.
        Some(state) => (2, -(state.level as i32), change.new_block.raw()),
        // Non-fluid (air/removal): lowest tier (1).
        None => (1, 0, change.new_block.raw()),
    }
}

/// Evaluates a batch of due ticks against the world, returning the reduced set of changes.
///
/// When the batch is large enough the evaluation (a pure read over the world) is fanned out across
/// the thread pool; otherwise it runs serially. Either way the result is identical because the read
/// phase never mutates the world, so no tick observes another tick's changes within the batch.
fn evaluate_batch(
    state: &GlobalState,
    dim: ActiveDimension,
    due: &[(ferrumc_world::pos::ChunkPos, Vec<ScheduledTick>)],
) -> Vec<FluidChange> {
    // Flatten the per-chunk groups into a single list of positions to evaluate.
    let positions: Vec<BlockPos> = due
        .iter()
        .flat_map(|(_, ticks)| ticks.iter().map(|t| t.pos))
        .collect();

    if positions.len() < PARALLEL_THRESHOLD {
        evaluate_serial(state, dim, &positions)
    } else {
        evaluate_parallel(state, dim, &positions)
    }
}

/// Evaluates a single ticking position against the world, picking the right [`FluidRules`] from
/// whatever fluid currently sits there, and dispatching to the configured spreading kernel.
/// Non-fluid positions produce no changes (the block was removed or replaced between scheduling
/// and now).
#[inline]
fn evaluate_position<V: BlockView>(view: &V, dim: Dimension, pos: BlockPos) -> Vec<FluidChange> {
    let block = view.block_at(pos);
    let Some(state) = fluid_state(block) else {
        return Vec::new();
    };
    let rules = FluidRules::for_kind(state.kind, dim);
    let algorithm = get_global_config().fluids.algorithm;
    ferrumc_world::fluid::compute_tick(algorithm, pos, view, rules)
}

/// Serial evaluation: computes changes for every position on the calling thread.
fn evaluate_serial(
    state: &GlobalState,
    dim: ActiveDimension,
    positions: &[BlockPos],
) -> Vec<FluidChange> {
    let view = WorldBlockView {
        state: state.clone(),
        dim,
    };
    let mut changes = Vec::new();
    for &pos in positions {
        changes.extend(evaluate_position(&view, dim.0, pos));
    }
    reduce_changes(changes)
}

/// Parallel evaluation: splits the positions across the thread pool and merges the results.
///
/// Each task gets its own `Arc` handle and its own read-only [`WorldBlockView`]. Because the world
/// is never written during evaluation, concurrent reads are safe and the merged result is
/// independent of task scheduling order (after [`reduce_changes`] applies its deterministic
/// tie-breaking).
///
/// Before fanning out, every chunk that the batch could read (each ticking block's chunk and its
/// six neighbours' chunks) is loaded/generated **serially on the calling thread**. This is
/// essential: `load_or_generate_chunk` writes to the chunk cache and storage backend when a chunk
/// is missing, which is not safe to do concurrently (it would contend on the LMDB writer and the
/// DashMap shard). Pre-warming guarantees the parallel phase only ever hits cached chunks, making
/// it a genuine read-only phase.
fn evaluate_parallel(
    state: &GlobalState,
    dim: ActiveDimension,
    positions: &[BlockPos],
) -> Vec<FluidChange> {
    prewarm_chunks(state, dim, positions);

    let worker_count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let task_size = positions.len().div_ceil(worker_count).max(1);
    let mut batch = state.thread_pool.batch::<Vec<FluidChange>>();
    for slice in positions.chunks(task_size) {
        let slice = slice.to_vec();
        let task_state = state.clone();
        batch.execute(move || {
            let view = WorldBlockView {
                state: task_state,
                dim,
            };
            let mut changes = Vec::new();
            for pos in slice {
                changes.extend(evaluate_position(&view, dim.0, pos));
            }
            changes
        });
    }

    let mut all = Vec::new();
    for partial in batch.wait() {
        all.extend(partial);
    }
    reduce_changes(all)
}

/// Horizontal radius (in blocks) that a single fluid tick may read.
///
/// The current six-neighbour model only needs radius 1, but the vanilla-style spread we are moving
/// toward does a bounded slope search (`getSlopeDistance`, default depth 4) to find the nearest
/// hole. Pre-warming to this radius now means the parallel read phase stays a genuine read-only
/// phase once the new kernel lands: every chunk the search can touch is already cached, so no
/// worker thread triggers chunk generation (which writes, and is unsafe to do concurrently).
///
/// Kept slightly larger than vanilla's depth-4 search so a downward probe at the rim of the search
/// box still lands on a pre-warmed chunk.
const FLUID_SEARCH_RADIUS: i32 = 5;

/// Loads or generates, on the calling thread, every chunk the parallel evaluation phase might read.
///
/// A fluid tick reads a horizontal box of half-width [`FLUID_SEARCH_RADIUS`] around the ticking
/// block (plus one below, for downward probes). The set of chunks touched is therefore every chunk
/// spanned by that box for every ticking position. Doing this serially up front removes all chunk
/// generation/insertion from the parallel phase, leaving only cache reads there.
fn prewarm_chunks(state: &GlobalState, dim: ActiveDimension, positions: &[BlockPos]) {
    use std::collections::HashSet;
    let r = FLUID_SEARCH_RADIUS;
    let mut chunks = HashSet::new();
    for &pos in positions {
        // The search is horizontal, so we only need to span chunks in x/z. Iterate every chunk
        // column the half-width-`r` box covers (robust even if `r` exceeds a chunk width).
        let min = (pos + (-r, 0, -r)).chunk();
        let max = (pos + (r, 0, r)).chunk();
        for cx in min.x()..=max.x() {
            for cz in min.z()..=max.z() {
                chunks.insert(ferrumc_world::pos::ChunkPos::new(cx, cz));
            }
        }
    }
    for chunk_pos in chunks {
        // The result is intentionally discarded; the call's side effect (populating the cache) is
        // what matters. Errors are ignored here and surfaced later when the block is read.
        let _ = ferrumc_utils::world::load_or_generate_chunk(state, chunk_pos, dim.name());
    }
}

/// Applies reduced changes to the world, broadcasts them, and re-schedules follow-up ticks.
///
/// Shared by the serial and parallel processing paths so both behave identically once changes have
/// been computed. Runs on the calling (main) thread, so writes are serial and need no locking
/// beyond the per-chunk guard taken inside [`apply_change`].
///
/// For each applied change this:
/// * broadcasts the new block to nearby players, plus the lava-extinguish "fizz" effect when the
///   change is a solidification ([`FluidChange::fizz`]);
/// * re-ticks the changed flowing block itself when it asked for it;
/// * wakes the fluid **neighbours** of the changed position (via [`seed_fluid_tick`], which also
///   gives a lava-meets-water neighbour the fast reaction cadence). This neighbour propagation is
///   what lets a recede, a fresh spread, or a solidification ripple through the whole body of fluid
///   instead of stopping at the first ring — the analogue of vanilla's neighbour block updates.
fn apply_changes(
    changes: &[FluidChange],
    scheduler: &mut BlockTickScheduler,
    state: &GlobalState,
    dim: ActiveDimension,
    current: u64,
    players: &Query<(Entity, &StreamWriter, &Position)>,
) {
    // Fallback cadence for re-ticking a changed block whose new state is not itself fluid.
    let fallback_delay = FluidRules::for_kind(FluidKind::Water, dim.0).tick_delay;

    for change in changes {
        if !apply_change(state, dim, change) {
            continue;
        }
        broadcast_change(change, state, players);
        if change.fizz {
            // Play the hiss + smoke at the solidified block.
            broadcast_fizz(change.pos, state, players);
        }

        // Re-tick the changed block itself if it is still flowing fluid that may keep evolving.
        if change.reschedule {
            let delay = rules_for_block(change.new_block, dim.0)
                .map(|r| r.tick_delay)
                .unwrap_or(fallback_delay);
            scheduler.schedule(change.pos, TickKind::FluidSpread, current, delay);
        }

        // Wake fluid neighbours so the update propagates outward. `seed_fluid_tick` skips
        // non-fluid neighbours and picks the reaction cadence for lava that now touches water.
        for neighbour in fluid_neighbours(change.pos) {
            seed_fluid_tick(scheduler, state, dim, current, neighbour);
        }
    }
}

/// Broadcasts the lava-extinguish effect (sound + smoke) at `pos` to nearby players.
fn broadcast_fizz(
    pos: BlockPos,
    state: &GlobalState,
    players: &Query<(Entity, &StreamWriter, &Position)>,
) {
    let render_distance = get_global_config().chunk_render_distance as i32;
    let target_chunk = pos.chunk();
    let packet = LevelEventPacket::lava_extinguish(NetworkPosition {
        x: pos.pos.x,
        y: pos.pos.y as i16,
        z: pos.pos.z,
    });

    for (eid, conn, position) in players.iter() {
        if !state.players.is_connected(eid) {
            continue;
        }
        let player_chunk = position.chunk();
        if (target_chunk.x() - player_chunk.x).abs() <= render_distance
            && (target_chunk.z() - player_chunk.y).abs() <= render_distance
        {
            if let Err(err) = conn.send_packet_ref(&packet) {
                trace!("[fluid] failed to send lava-fizz level event: {:?}", err);
            }
        }
    }
}

/// Processes all fluid ticks due this game tick.
///
/// The read/evaluate phase ([`evaluate_batch`]) may run in parallel; the apply phase
/// ([`apply_changes`]) is serial on the main thread. Because evaluation is purely read-only, the
/// world state observed by every tick in the batch is the same snapshot, making the parallel result
/// identical to the serial one (verified by the parity test).
pub fn process_fluid_ticks(
    mut scheduler: ResMut<FluidScheduler>,
    tick: Res<TickCounter>,
    state: Res<GlobalStateResource>,
    dim: Res<ActiveDimension>,
    mut control: ResMut<FluidTickControl>,
    players: Query<(Entity, &StreamWriter, &Position)>,
) {
    // Debug freeze/step: when frozen, only advance on an explicitly queued step. Checked before
    // draining so a frozen simulation leaves its pending ticks untouched.
    if !control.allow_tick() {
        return;
    }

    let current = tick.get();
    let due = scheduler.0.drain_due(current);
    if due.is_empty() {
        return;
    }

    let global = &state.0;
    let dim = *dim;
    let changes = evaluate_batch(global, dim, &due);
    if changes.is_empty() {
        return;
    }

    trace!(
        "[fluid] tick {}: {} scheduled tick(s) produced {} change(s)",
        current,
        due.iter().map(|(_, t)| t.len()).sum::<usize>(),
        changes.len(),
    );

    apply_changes(&changes, &mut scheduler.0, global, dim, current, &players);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::{Schedule, World};
    use ferrumc_macros::block;
    use ferrumc_state::create_test_state;
    use ferrumc_world::fluid::{fluid_block, fluid_state, FluidKind};
    use ferrumc_world::pos::BlockPos;

    /// All tests in this module simulate the overworld; this constant keeps the test bodies short
    /// while still exercising the dimension-aware code path.
    const TEST_DIM: ActiveDimension = ActiveDimension(Dimension::Overworld);
    const TEST_DIM_NAME: &str = "overworld";
    /// Water's overworld cadence — what every test here seeds with.
    const TEST_DELAY: u64 = 5;

    /// Drives the fluid system end-to-end through the ECS, mirroring the real tick schedule
    /// (advance tick counter, then process fluid ticks). Verifies that a seeded water source
    /// actually spreads into the live world without a player present.
    #[test]
    fn seeded_water_source_spreads_through_ecs() {
        let mut world = World::new();
        let (state, _temp_dir) = create_test_state();

        // Lay a clean, contained slab: stone floor at y=63 with air above it, large enough that the
        // source's spread (a level-0 source reaches at most 7 blocks) stays entirely on this floor.
        // Clearing the air is what makes the test independent of the generated terrain — otherwise
        // the world generator's surface at these columns can be solid (or ocean) at y=64 and block
        // the spread. A self-contained slab also keeps the simulation tiny and fast.
        const SLAB: i32 = 8;
        {
            let global = &state.0;
            // Pre-generate every chunk the slab spans (each call acquires and releases its own
            // guard), so the subsequent writes hit cached chunks. A fresh test world has no chunks
            // stored, so set_block_and_fetch alone would fail with ChunkNotFound.
            let mut seen = std::collections::HashSet::new();
            for x in -SLAB..=SLAB {
                for z in -SLAB..=SLAB {
                    let chunk_pos = BlockPos::of(x, 63, z).chunk();
                    if seen.insert((chunk_pos.x(), chunk_pos.z())) {
                        let _ = ferrumc_utils::world::load_or_generate_chunk(
                            global,
                            chunk_pos,
                            TEST_DIM_NAME,
                        )
                        .expect("generate chunk");
                    }
                }
            }
            for x in -SLAB..=SLAB {
                for z in -SLAB..=SLAB {
                    global
                        .world
                        .set_block_and_fetch(BlockPos::of(x, 63, z), TEST_DIM_NAME, block!("stone"))
                        .expect("set floor");
                    global
                        .world
                        .set_block_and_fetch(BlockPos::of(x, 64, z), TEST_DIM_NAME, block!("air"))
                        .expect("clear y=64");
                    global
                        .world
                        .set_block_and_fetch(BlockPos::of(x, 65, z), TEST_DIM_NAME, block!("air"))
                        .expect("clear y=65");
                }
            }
            // Place the water source on the cleared floor.
            global
                .world
                .set_block_and_fetch(
                    BlockPos::of(0, 64, 0),
                    TEST_DIM_NAME,
                    fluid_block(FluidKind::Water, 0),
                )
                .expect("set source");
        }

        world.insert_resource(state.clone());
        world.insert_resource(TickCounter::new());
        world.insert_resource(TEST_DIM);
        world.insert_resource(FluidTickControl::default());
        let mut sched = FluidScheduler::default();

        // Seed the source as the placement handler would.
        let source = BlockPos::of(0, 64, 0);
        seed_fluid_tick(&mut sched.0, &state.0, TEST_DIM, 0, source);
        world.insert_resource(sched);

        // Build a schedule that mimics the tick: advance counter, then process fluids.
        let mut schedule = Schedule::default();
        schedule.add_systems(crate::systems::tick_counter::handle);
        schedule.add_systems(process_fluid_ticks);

        // Run ticks until the source has spread to its horizontal neighbour, rather than capping at
        // a fixed tick budget. The elapsed time is printed (visible under `--nocapture`); the loop
        // is bounded only by a generous hang-guard so a real regression fails instead of spinning
        // forever.
        let neighbour = BlockPos::of(1, 64, 0);
        let read_neighbour = || {
            ferrumc_utils::world::load_or_generate_chunk(&state.0, neighbour.chunk(), TEST_DIM_NAME)
                .expect("load chunk")
                .get_block(neighbour.chunk_block_pos())
        };

        let start = std::time::Instant::now();
        const HANG_GUARD: u32 = 10_000;
        let mut ticks = 0u32;
        let fluid = loop {
            schedule.run(&mut world);
            ticks += 1;
            if let Some(fluid) = fluid_state(read_neighbour()) {
                break fluid;
            }
            assert!(
                ticks < HANG_GUARD,
                "neighbour still has no water after {ticks} ticks; spreading appears broken"
            );
        };
        println!(
            "water reached the neighbour after {ticks} ticks in {:?}",
            start.elapsed()
        );

        assert_eq!(fluid.kind, FluidKind::Water);
        assert!(
            !fluid.is_source(),
            "spread block should be flowing, not a source"
        );
    }

    /// Whether to force the serial or parallel evaluator in [`run_to_steady_state`].
    #[derive(Clone, Copy, Debug)]
    enum EvalMode {
        Serial,
        Parallel,
    }

    /// Drives the full fluid loop (drain → evaluate → apply → re-schedule) directly, without the
    /// ECS, using the chosen evaluator. Runs until the scheduler empties — there is no tick budget,
    /// so a slow simulation reports its real cost instead of failing a fixed limit. The elapsed wall
    /// time and tick count are printed (visible under `--nocapture`).
    fn run_to_steady_state(
        state: &GlobalState,
        scheduler: &mut BlockTickScheduler,
        mode: EvalMode,
    ) {
        let start = std::time::Instant::now();
        let mut tick = 0u64;
        // A settling simulation has no fixed tick budget, but it must terminate. This guard turns a
        // non-convergence bug (e.g. a cell oscillating between two states forever) into a fast, clear
        // test failure instead of an unbounded hang. It is far above any legitimate settling time
        // for the small scenarios in this module.
        const HANG_GUARD: u64 = 100_000;
        loop {
            tick += 1;
            assert!(
                tick <= HANG_GUARD,
                "fluid simulation ({mode:?}) did not settle within {HANG_GUARD} ticks; \
                 it is likely oscillating instead of converging"
            );
            let due = scheduler.drain_due(tick);
            if due.is_empty() {
                if scheduler.pending_count() == 0 {
                    break;
                }
                continue;
            }
            let positions: Vec<BlockPos> = due
                .iter()
                .flat_map(|(_, ticks)| ticks.iter().map(|t| t.pos))
                .collect();
            let changes = match mode {
                EvalMode::Serial => evaluate_serial(state, TEST_DIM, &positions),
                EvalMode::Parallel => evaluate_parallel(state, TEST_DIM, &positions),
            };
            for change in &changes {
                apply_change(state, TEST_DIM, change);
                if change.reschedule {
                    // Look up the cadence per change, the same way `apply_changes` does in
                    // production. For the all-water tests in this module this resolves to the
                    // water delay.
                    let delay = rules_for_block(change.new_block, TEST_DIM.0)
                        .map(|r| r.tick_delay)
                        .unwrap_or(TEST_DELAY);
                    scheduler.schedule(change.pos, TickKind::FluidSpread, tick, delay);
                }
                // Wake fluid neighbours so updates propagate, matching production `apply_changes`.
                for neighbour in fluid_neighbours(change.pos) {
                    let block = match ferrumc_utils::world::load_or_generate_chunk(
                        state,
                        neighbour.chunk(),
                        TEST_DIM_NAME,
                    ) {
                        Ok(chunk) => chunk.get_block(neighbour.chunk_block_pos()),
                        Err(_) => continue,
                    };
                    if let Some(rules) = rules_for_block(block, TEST_DIM.0) {
                        scheduler.schedule(
                            neighbour,
                            TickKind::FluidSpread,
                            tick,
                            rules.tick_delay,
                        );
                    }
                }
            }
        }
        println!(
            "run_to_steady_state ({mode:?}) settled after {tick} ticks in {:?}",
            start.elapsed()
        );
    }

    /// Builds a fresh world with a stone floor and walls forming an enclosed basin, plus a water
    /// source at the centre, and seeds the source. Returns the state, a temp dir guard, and the
    /// seeded scheduler.
    fn basin_world(radius: i32) -> (GlobalStateResource, tempfile::TempDir, BlockTickScheduler) {
        let (state, temp_dir) = create_test_state();
        {
            let global = &state.0;
            // Pre-generate every chunk this basin spans (each call acquires and releases its own
            // guard), so the subsequent writes hit cached chunks. A fresh test world has no chunks
            // stored, so set_block_and_fetch alone would fail with ChunkNotFound.
            let mut seen = std::collections::HashSet::new();
            for x in -radius..=radius {
                for z in -radius..=radius {
                    let chunk_pos = BlockPos::of(x, 64, z).chunk();
                    if seen.insert((chunk_pos.x(), chunk_pos.z())) {
                        let _ = ferrumc_utils::world::load_or_generate_chunk(
                            global,
                            chunk_pos,
                            TEST_DIM_NAME,
                        )
                        .expect("generate chunk");
                    }
                }
            }
            // Use set_block_and_fetch so each write acquires and releases its own chunk guard.
            // Holding one chunk guard while loading another position in the *same* chunk (e.g. the
            // floor at y=63 and the wall at y=64 share a chunk) would re-enter the DashMap shard
            // lock and deadlock.
            for x in -radius..=radius {
                for z in -radius..=radius {
                    global
                        .world
                        .set_block_and_fetch(BlockPos::of(x, 63, z), TEST_DIM_NAME, block!("stone"))
                        .expect("set floor");
                    if x.abs() == radius || z.abs() == radius {
                        global
                            .world
                            .set_block_and_fetch(
                                BlockPos::of(x, 64, z),
                                TEST_DIM_NAME,
                                block!("stone"),
                            )
                            .expect("set wall");
                    }
                }
            }
            global
                .world
                .set_block_and_fetch(
                    BlockPos::of(0, 64, 0),
                    TEST_DIM_NAME,
                    fluid_block(FluidKind::Water, 0),
                )
                .expect("set source");
        }
        let mut scheduler = BlockTickScheduler::new();
        seed_fluid_tick(
            &mut scheduler,
            &state.0,
            TEST_DIM,
            0,
            BlockPos::of(0, 64, 0),
        );
        (state, temp_dir, scheduler)
    }

    /// Snapshots the block states across a cuboid so two worlds can be compared exactly.
    fn snapshot(
        state: &GlobalState,
        radius: i32,
        y_lo: i32,
        y_hi: i32,
    ) -> Vec<(i32, i32, i32, u32)> {
        let mut out = Vec::new();
        for x in -radius..=radius {
            for y in y_lo..=y_hi {
                for z in -radius..=radius {
                    let pos = BlockPos::of(x, y, z);
                    let block = ferrumc_utils::world::load_or_generate_chunk(
                        state,
                        pos.chunk(),
                        TEST_DIM_NAME,
                    )
                    .expect("load chunk")
                    .get_block(pos.chunk_block_pos());
                    out.push((x, y, z, block.raw()));
                }
            }
        }
        out
    }

    /// The serial and parallel evaluators must converge to the exact same world state. This is the
    /// core guarantee that parallelisation does not change observable behaviour.
    #[test]
    fn serial_and_parallel_reach_identical_state() {
        let radius = 3;

        let (serial_state, _s_tmp, mut serial_sched) = basin_world(radius);
        run_to_steady_state(&serial_state.0, &mut serial_sched, EvalMode::Serial);

        let (par_state, _p_tmp, mut par_sched) = basin_world(radius);
        run_to_steady_state(&par_state.0, &mut par_sched, EvalMode::Parallel);

        let serial_snap = snapshot(&serial_state.0, radius, 62, 66);
        let par_snap = snapshot(&par_state.0, radius, 62, 66);
        assert_eq!(
            serial_snap, par_snap,
            "serial and parallel fluid evaluation must produce identical world state"
        );
    }

    /// `reduce_changes` must be deterministic: the same set of conflicting changes always collapses
    /// to the same winner regardless of input order.
    #[test]
    fn reduce_changes_is_order_independent() {
        let pos = BlockPos::of(5, 70, 5);
        let strong = FluidChange::flow(pos, fluid_block(FluidKind::Water, 1), true);
        let weak = FluidChange::flow(pos, fluid_block(FluidKind::Water, 6), false);
        let air = FluidChange::flow(pos, block!("air"), false);

        let a = reduce_changes(vec![strong, weak, air]);
        let b = reduce_changes(vec![air, weak, strong]);
        let c = reduce_changes(vec![weak, air, strong]);

        assert_eq!(a.len(), 1);
        assert_eq!(b.len(), 1);
        assert_eq!(c.len(), 1);
        // The strongest (lowest level) fluid wins in every ordering.
        assert_eq!(a[0].new_block, strong.new_block);
        assert_eq!(b[0].new_block, strong.new_block);
        assert_eq!(c[0].new_block, strong.new_block);
        // reschedule is OR-ed across contributors.
        assert!(a[0].reschedule);
    }

    /// End-to-end through the real apply path: flowing lava adjacent to a water source solidifies
    /// into cobblestone.
    #[test]
    fn flowing_lava_beside_water_source_turns_to_cobblestone() {
        let (state, _tmp) = create_test_state();
        let global = &state.0;

        let _ = ferrumc_utils::world::load_or_generate_chunk(
            global,
            BlockPos::of(0, 64, 0).chunk(),
            TEST_DIM_NAME,
        )
        .expect("generate chunk");

        let lava_pos = BlockPos::of(0, 64, 0);
        let water_pos = BlockPos::of(1, 64, 0);
        global
            .world
            .set_block_and_fetch(lava_pos, TEST_DIM_NAME, fluid_block(FluidKind::Lava, 2))
            .expect("set flowing lava");
        global
            .world
            .set_block_and_fetch(water_pos, TEST_DIM_NAME, fluid_block(FluidKind::Water, 0))
            .expect("set water source");

        let mut scheduler = BlockTickScheduler::new();
        scheduler.schedule(lava_pos, TickKind::FluidSpread, 0, 0);
        run_to_steady_state(global, &mut scheduler, EvalMode::Serial);

        let result =
            ferrumc_utils::world::load_or_generate_chunk(global, lava_pos.chunk(), TEST_DIM_NAME)
                .expect("load chunk")
                .get_block(lava_pos.chunk_block_pos());

        assert_eq!(
            result,
            block!("cobblestone"),
            "flowing lava touching a water source horizontally should become cobblestone, got {}",
            result
        );
    }

    /// End-to-end through the real apply path: flowing lava adjacent to a water source solidifies
    /// into stone. This exercises `compute_fluid_tick`'s reaction branch plus the production
    /// `apply_changes` (write + neighbour waking), not just the pure algorithm.
    #[test]
    fn flowing_lava_above_water_source_turns_to_stone() {
        let (state, _tmp) = create_test_state();
        let global = &state.0;

        let _ = ferrumc_utils::world::load_or_generate_chunk(
            global,
            BlockPos::of(0, 64, 0).chunk(),
            TEST_DIM_NAME,
        )
        .expect("generate chunk");

        let lava_pos = BlockPos::of(0, 65, 0);
        let water_pos = BlockPos::of(0, 64, 0);
        global
            .world
            .set_block_and_fetch(lava_pos, TEST_DIM_NAME, fluid_block(FluidKind::Lava, 2))
            .expect("set flowing lava");
        global
            .world
            .set_block_and_fetch(water_pos, TEST_DIM_NAME, fluid_block(FluidKind::Water, 0))
            .expect("set water source");

        let mut scheduler = BlockTickScheduler::new();
        // Tick the lava: it should flow down and turn the water into stone.
        scheduler.schedule(lava_pos, TickKind::FluidSpread, 0, 0);
        run_to_steady_state(global, &mut scheduler, EvalMode::Serial);

        let result =
            ferrumc_utils::world::load_or_generate_chunk(global, water_pos.chunk(), TEST_DIM_NAME)
                .expect("load chunk")
                .get_block(water_pos.chunk_block_pos());

        assert_eq!(
            result,
            block!("stone"),
            "flowing lava falling into water should become stone at the water's position, got {}",
            result
        );
    }

    //obsidian check :3
    // up to 2026-06-04, NCC find these tests are easy to overtime on github runners.
    // So, we need either optimize it or set higher time bar.
    #[test]
    fn flowing_water_above_lava_source_turns_to_obsidian() {
        let (state, _tmp) = create_test_state();
        let global = &state.0;

        let _ = ferrumc_utils::world::load_or_generate_chunk(
            global,
            BlockPos::of(0, 64, 0).chunk(),
            TEST_DIM_NAME,
        )
        .expect("generate chunk");

        let lava_pos = BlockPos::of(0, 64, 0);
        let water_pos = BlockPos::of(0, 65, 0);
        global
            .world
            .set_block_and_fetch(lava_pos, TEST_DIM_NAME, fluid_block(FluidKind::Lava, 0))
            .expect("set lava source");
        global
            .world
            .set_block_and_fetch(water_pos, TEST_DIM_NAME, fluid_block(FluidKind::Water, 0))
            .expect("set flowing water");

        // Seed the placed water *and its neighbours*, exactly as the block-placement handler does
        // (`seed_fluid_tick` wakes neighbours). The lava sits directly below the water, so seeding
        // neighbours is what schedules the lava to tick and harden. Doing this explicitly keeps the
        // test independent of the generated terrain around the origin — it must not rely on water
        // happening to spread onto a block adjacent to the lava to wake it.
        let mut scheduler = BlockTickScheduler::new();
        scheduler.schedule(water_pos, TickKind::FluidSpread, 0, 0);
        for neighbour in fluid_neighbours(water_pos) {
            scheduler.schedule(neighbour, TickKind::FluidSpread, 0, 0);
        }
        run_to_steady_state(global, &mut scheduler, EvalMode::Serial);

        let result =
            ferrumc_utils::world::load_or_generate_chunk(global, lava_pos.chunk(), TEST_DIM_NAME)
                .expect("load chunk")
                .get_block(lava_pos.chunk_block_pos());

        assert_eq!(
            result,
            block!("obsidian"),
            "flowing water falling into lava source should become obsidian at the lava's position, got {}",
            result
        );
    }

    /// Manual benchmark comparing serial vs parallel evaluation on a large batch. Ignored by
    /// default (it only prints timing data and is sensitive to machine load); run explicitly with:
    /// `cargo test -p ferrumc --bin ferrumc bench_serial_vs_parallel -- --ignored --nocapture`.
    #[test]
    #[ignore = "benchmark; prints timing only"]
    fn bench_serial_vs_parallel() {
        use std::time::Instant;

        // Large flat floor so a big batch of fluid positions can be evaluated at once.
        let radius = 48;
        let (state, _tmp, _sched) = basin_world(radius);
        let global = &state.0;

        // Fill the interior with flowing water states to create a large evaluation batch.
        let mut positions = Vec::new();
        for x in -(radius - 1)..=(radius - 1) {
            for z in -(radius - 1)..=(radius - 1) {
                let pos = BlockPos::of(x, 64, z);
                global
                    .world
                    .set_block_and_fetch(pos, TEST_DIM_NAME, fluid_block(FluidKind::Water, 1))
                    .expect("fill water");
                positions.push(pos);
            }
        }
        println!("batch size: {} positions", positions.len());

        // Warm the cache once so neither path pays generation cost during timing.
        prewarm_chunks(global, TEST_DIM, &positions);

        let iterations = 20;

        let t0 = Instant::now();
        for _ in 0..iterations {
            let _ = evaluate_serial(global, TEST_DIM, &positions);
        }
        let serial = t0.elapsed() / iterations;

        let t1 = Instant::now();
        for _ in 0..iterations {
            let _ = evaluate_parallel(global, TEST_DIM, &positions);
        }
        let parallel = t1.elapsed() / iterations;

        println!("serial avg:   {:?}", serial);
        println!("parallel avg: {:?}", parallel);
        println!(
            "speedup: {:.2}x",
            serial.as_secs_f64() / parallel.as_secs_f64()
        );
    }

    /// `settle_loaded_fluids` seeds a fluid tick for a hanging fluid in a newly loaded chunk, and
    /// does so only once: the second run is a no-op because the chunk is already tracked as settled.
    #[test]
    fn settle_seeds_hanging_fluid_once() {
        let mut world = World::new();
        let (state, _tmp) = create_test_state();

        // Pre-generate a chunk, then place a water block high in the air column (air directly below
        // it) — an unambiguous down-flow frontier independent of the generated surface.
        let coords = (3, 5);
        let cpos = ChunkPos::new(coords.0, coords.1);
        let _ = ferrumc_utils::world::load_or_generate_chunk(&state.0, cpos, TEST_DIM_NAME)
            .expect("generate chunk");
        let water_pos = BlockPos::of(coords.0 * 16 + 8, 150, coords.1 * 16 + 8);
        state
            .0
            .world
            .set_block_and_fetch(water_pos, TEST_DIM_NAME, fluid_block(FluidKind::Water, 0))
            .expect("place hanging water");

        world.insert_resource(state.clone());
        world.insert_resource(TickCounter::new());
        world.insert_resource(TEST_DIM);
        world.insert_resource(FluidScheduler::default());
        world.insert_resource(FluidSettleTracker::default());

        // A player whose loaded set includes the chunk.
        let mut receiver = ChunkReceiver::default();
        receiver.loaded.insert(coords);
        world.spawn(receiver);

        let mut schedule = Schedule::default();
        schedule.add_systems(settle_loaded_fluids);

        schedule.run(&mut world);
        let after_first = world.resource::<FluidScheduler>().0.pending_count();
        assert!(
            after_first > 0,
            "settle should have seeded the hanging water cell, pending={after_first}"
        );

        // Running again must not re-scan the chunk (it is already settled), so nothing changes.
        schedule.run(&mut world);
        let after_second = world.resource::<FluidScheduler>().0.pending_count();
        assert_eq!(
            after_first, after_second,
            "an already-settled chunk must not be re-scanned (first={after_first}, second={after_second})"
        );
    }
}
