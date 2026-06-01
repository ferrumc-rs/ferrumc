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

use bevy_ecs::prelude::{Entity, Query, Res, ResMut, Resource};
use bevy_ecs::prelude::MessageReader;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::tick::TickCounter;
use ferrumc_core::transform::position::Position;
use ferrumc_messages::BlockBrokenEvent;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::{GlobalState, GlobalStateResource};
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::fluid::is_fluid;
use ferrumc_world::fluid::spread::{compute_fluid_tick, BlockView, FluidChange};
use ferrumc_world::fluid::fluid_state;
use ferrumc_world::pos::BlockPos;
use ferrumc_world::scheduler::{BlockTickScheduler, ScheduledTick, TickKind};
use std::collections::HashMap;
use tracing::{error, trace};

/// Tick delay before a seeded or re-scheduled fluid block is evaluated.
///
/// Vanilla water updates every 5 ticks; lava is slower, but the simplified model uses a single
/// delay for now. Kept here (rather than in the algorithm) because timing is a scheduling concern.
pub const FLUID_TICK_DELAY: u64 = 5;

/// Minimum number of due ticks in a batch before the parallel evaluation path is used.
///
/// Below this threshold the overhead of dispatching work to the thread pool outweighs the benefit,
/// so the batch is evaluated serially on the calling thread. Small fluid disturbances (a single
/// placed bucket) stay on the fast serial path; large flows fan out across cores.
pub const PARALLEL_THRESHOLD: usize = 64;

/// The dimension fluids are simulated in. The world currently only models the overworld; this
/// constant marks the call sites that will need revisiting once multiple dimensions exist.
const DIMENSION: &str = "overworld";

/// ECS resource wrapping the per-chunk block tick scheduler.
///
/// The wrapper lives in the binary because the `ferrumc-world` crate intentionally does not depend
/// on `bevy_ecs`; the scheduler itself is a plain data structure.
#[derive(Resource, Default)]
pub struct FluidScheduler(pub BlockTickScheduler);

/// A [`BlockView`] backed by the live world.
///
/// Holds an owned [`GlobalState`] handle (an `Arc` clone) so it can be moved into worker threads
/// during the parallel read phase. Reads go through the chunk cache via a shared (read) lock,
/// which is safe to perform concurrently from multiple threads.
struct WorldBlockView {
    state: GlobalState,
}

impl BlockView for WorldBlockView {
    fn block_at(&self, pos: BlockPos) -> BlockStateId {
        match ferrumc_utils::world::load_or_generate_chunk(&self.state, pos.chunk(), DIMENSION) {
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
/// Intended to be called from block placement/break handlers (and from neighbour updates) to kick
/// off or continue spreading. No-op for non-fluid blocks.
pub fn seed_fluid_tick(
    scheduler: &mut BlockTickScheduler,
    state: &GlobalState,
    current_tick: u64,
    pos: BlockPos,
) {
    let block = match ferrumc_utils::world::load_or_generate_chunk(state, pos.chunk(), DIMENSION) {
        Ok(chunk) => chunk.get_block(pos.chunk_block_pos()),
        Err(_) => return,
    };
    if is_fluid(block) {
        scheduler.schedule(pos, TickKind::FluidSpread, current_tick, FLUID_TICK_DELAY);
    }
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
) {
    let current = tick.get();
    for event in events.read() {
        let pos = event.position;
        // The broken position itself (in case it is now exposed to a fluid) and its neighbours.
        seed_fluid_tick(&mut scheduler.0, &state.0, current, pos);
        for neighbour in neighbours(pos) {
            seed_fluid_tick(&mut scheduler.0, &state.0, current, neighbour);
        }
    }
}

/// Writes a single block change back to the world. Returns true on success.
fn apply_change(state: &GlobalState, change: &FluidChange) -> bool {
    match ferrumc_utils::world::load_or_generate_mut(state, change.pos.chunk(), DIMENSION) {
        Ok(mut chunk) => {
            chunk.set_block(change.pos.chunk_block_pos(), change.new_block);
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

/// Priority key for [`reduce_changes`]. Higher wins. Fluids outrank non-fluids; among fluids a
/// lower level (stronger) outranks; ties fall back to the raw id for stability.
fn change_priority(change: &FluidChange) -> (u8, i32, u32) {
    match fluid_state(change.new_block) {
        // Fluid: top tier (2). Stronger (lower level) should win, so negate the level.
        Some(state) => (2, -(state.level as i32), change.new_block.raw()),
        // Non-fluid (air/removal): lower tier (1).
        None => (1, 0, change.new_block.raw()),
    }
}

/// Evaluates a batch of due ticks against the world, returning the reduced set of changes.
///
/// When the batch is large enough the evaluation (a pure read over the world) is fanned out across
/// the thread pool; otherwise it runs serially. Either way the result is identical because the read
/// phase never mutates the world, so no tick observes another tick's changes within the batch.
fn evaluate_batch(state: &GlobalState, due: &[(ferrumc_world::pos::ChunkPos, Vec<ScheduledTick>)]) -> Vec<FluidChange> {
    // Flatten the per-chunk groups into a single list of positions to evaluate.
    let positions: Vec<BlockPos> = due
        .iter()
        .flat_map(|(_, ticks)| ticks.iter().map(|t| t.pos))
        .collect();

    if positions.len() < PARALLEL_THRESHOLD {
        evaluate_serial(state, &positions)
    } else {
        evaluate_parallel(state, &positions)
    }
}

/// Serial evaluation: computes changes for every position on the calling thread.
fn evaluate_serial(state: &GlobalState, positions: &[BlockPos]) -> Vec<FluidChange> {
    let view = WorldBlockView {
        state: state.clone(),
    };
    let mut changes = Vec::new();
    for &pos in positions {
        changes.extend(compute_fluid_tick(pos, &view));
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
fn evaluate_parallel(state: &GlobalState, positions: &[BlockPos]) -> Vec<FluidChange> {
    prewarm_chunks(state, positions);

    let worker_count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let task_size = positions.len().div_ceil(worker_count).max(1);
    let mut batch = state.thread_pool.batch::<Vec<FluidChange>>();
    for slice in positions.chunks(task_size) {
        let slice = slice.to_vec();
        let task_state = state.clone();
        batch.execute(move || {
            let view = WorldBlockView { state: task_state };
            let mut changes = Vec::new();
            for pos in slice {
                changes.extend(compute_fluid_tick(pos, &view));
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

/// Loads or generates, on the calling thread, every chunk the parallel evaluation phase might read.
///
/// A fluid tick reads the ticking block and its six axis-aligned neighbours, so the set of chunks
/// touched is each position's chunk plus the chunks of its neighbours. Doing this serially up front
/// removes all chunk generation/insertion from the parallel phase, leaving only cache reads there.
fn prewarm_chunks(state: &GlobalState, positions: &[BlockPos]) {
    use std::collections::HashSet;
    let mut chunks = HashSet::new();
    for &pos in positions {
        chunks.insert(pos.chunk());
        for neighbour in neighbours(pos) {
            chunks.insert(neighbour.chunk());
        }
    }
    for chunk_pos in chunks {
        // The result is intentionally discarded; the call's side effect (populating the cache) is
        // what matters. Errors are ignored here and surfaced later when the block is read.
        let _ = ferrumc_utils::world::load_or_generate_chunk(state, chunk_pos, DIMENSION);
    }
}

/// Applies reduced changes to the world, broadcasts them, and re-schedules follow-up ticks.
///
/// Shared by the serial and parallel processing paths so both behave identically once changes have
/// been computed. Runs on the calling (main) thread, so writes are serial and need no locking
/// beyond the per-chunk guard taken inside [`apply_change`].
fn apply_changes(
    changes: &[FluidChange],
    scheduler: &mut BlockTickScheduler,
    state: &GlobalState,
    current: u64,
    players: &Query<(Entity, &StreamWriter, &Position)>,
) {
    for change in changes {
        if !apply_change(state, change) {
            continue;
        }
        broadcast_change(change, state, players);
        if change.reschedule {
            scheduler.schedule(change.pos, TickKind::FluidSpread, current, FLUID_TICK_DELAY);
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
    players: Query<(Entity, &StreamWriter, &Position)>,
) {
    let current = tick.get();
    let due = scheduler.0.drain_due(current);
    if due.is_empty() {
        return;
    }

    let global = &state.0;
    let changes = evaluate_batch(global, &due);
    if changes.is_empty() {
        return;
    }

    trace!(
        "[fluid] tick {}: {} scheduled tick(s) produced {} change(s)",
        current,
        due.iter().map(|(_, t)| t.len()).sum::<usize>(),
        changes.len(),
    );

    apply_changes(&changes, &mut scheduler.0, global, current, &players);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::{Schedule, World};
    use ferrumc_macros::block;
    use ferrumc_state::create_test_state;
    use ferrumc_world::fluid::{fluid_block, fluid_state, FluidKind};
    use ferrumc_world::pos::BlockPos;

    /// Drives the fluid system end-to-end through the ECS, mirroring the real tick schedule
    /// (advance tick counter, then process fluid ticks). Verifies that a seeded water source
    /// actually spreads into the live world without a player present.
    #[test]
    fn seeded_water_source_spreads_through_ecs() {
        let mut world = World::new();
        let (state, _temp_dir) = create_test_state();

        // Lay a stone floor under the source so water spreads horizontally instead of only falling.
        {
            let global = &state.0;
            let source = BlockPos::of(0, 64, 0);
            for x in -4..=4 {
                for z in -4..=4 {
                    let mut chunk = ferrumc_utils::world::load_or_generate_mut(
                        global,
                        BlockPos::of(x, 63, z).chunk(),
                        DIMENSION,
                    )
                    .expect("load chunk");
                    chunk.set_block(BlockPos::of(x, 63, z).chunk_block_pos(), block!("stone"));
                }
            }
            // Place the water source.
            let mut chunk =
                ferrumc_utils::world::load_or_generate_mut(global, source.chunk(), DIMENSION)
                    .expect("load chunk");
            chunk.set_block(source.chunk_block_pos(), fluid_block(FluidKind::Water, 0));
        }

        world.insert_resource(state.clone());
        world.insert_resource(TickCounter::new());
        let mut sched = FluidScheduler::default();

        // Seed the source as the placement handler would.
        let source = BlockPos::of(0, 64, 0);
        seed_fluid_tick(&mut sched.0, &state.0, 0, source);
        world.insert_resource(sched);

        // Build a schedule that mimics the tick: advance counter, then process fluids.
        let mut schedule = Schedule::default();
        schedule.add_systems(crate::systems::tick_counter::handle);
        schedule.add_systems(process_fluid_ticks);

        // Run enough ticks for the source to spread to its neighbours (delay is 5 ticks).
        for _ in 0..30 {
            schedule.run(&mut world);
        }

        // Verify a horizontal neighbour now holds flowing water in the live world.
        let neighbour = BlockPos::of(1, 64, 0);
        let block = ferrumc_utils::world::load_or_generate_chunk(&state.0, neighbour.chunk(), DIMENSION)
            .expect("load chunk")
            .get_block(neighbour.chunk_block_pos());
        let fluid = fluid_state(block).expect("neighbour should contain water after spreading");
        assert_eq!(fluid.kind, FluidKind::Water);
        assert!(!fluid.is_source(), "spread block should be flowing, not a source");
    }

    /// Whether to force the serial or parallel evaluator in [`run_to_steady_state`].
    #[derive(Clone, Copy)]
    enum EvalMode {
        Serial,
        Parallel,
    }

    /// Drives the full fluid loop (drain → evaluate → apply → re-schedule) directly, without the
    /// ECS, using the chosen evaluator. Returns once the scheduler empties or `max_ticks` elapse.
    fn run_to_steady_state(
        state: &GlobalState,
        scheduler: &mut BlockTickScheduler,
        mode: EvalMode,
        max_ticks: u64,
    ) {
        for tick in 1..=max_ticks {
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
                EvalMode::Serial => evaluate_serial(state, &positions),
                EvalMode::Parallel => evaluate_parallel(state, &positions),
            };
            for change in &changes {
                apply_change(state, change);
                if change.reschedule {
                    scheduler.schedule(change.pos, TickKind::FluidSpread, tick, FLUID_TICK_DELAY);
                }
            }
        }
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
                            global, chunk_pos, DIMENSION,
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
                        .set_block_and_fetch(BlockPos::of(x, 63, z), DIMENSION, block!("stone"))
                        .expect("set floor");
                    if x.abs() == radius || z.abs() == radius {
                        global
                            .world
                            .set_block_and_fetch(BlockPos::of(x, 64, z), DIMENSION, block!("stone"))
                            .expect("set wall");
                    }
                }
            }
            global
                .world
                .set_block_and_fetch(
                    BlockPos::of(0, 64, 0),
                    DIMENSION,
                    fluid_block(FluidKind::Water, 0),
                )
                .expect("set source");
        }
        let mut scheduler = BlockTickScheduler::new();
        seed_fluid_tick(&mut scheduler, &state.0, 0, BlockPos::of(0, 64, 0));
        (state, temp_dir, scheduler)
    }

    /// Snapshots the block states across a cuboid so two worlds can be compared exactly.
    fn snapshot(state: &GlobalState, radius: i32, y_lo: i32, y_hi: i32) -> Vec<(i32, i32, i32, u32)> {
        let mut out = Vec::new();
        for x in -radius..=radius {
            for y in y_lo..=y_hi {
                for z in -radius..=radius {
                    let pos = BlockPos::of(x, y, z);
                    let block =
                        ferrumc_utils::world::load_or_generate_chunk(state, pos.chunk(), DIMENSION)
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
        run_to_steady_state(&serial_state.0, &mut serial_sched, EvalMode::Serial, 400);

        let (par_state, _p_tmp, mut par_sched) = basin_world(radius);
        run_to_steady_state(&par_state.0, &mut par_sched, EvalMode::Parallel, 400);

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
        let strong = FluidChange {
            pos,
            new_block: fluid_block(FluidKind::Water, 1),
            reschedule: true,
        };
        let weak = FluidChange {
            pos,
            new_block: fluid_block(FluidKind::Water, 6),
            reschedule: false,
        };
        let air = FluidChange {
            pos,
            new_block: block!("air"),
            reschedule: false,
        };

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
                    .set_block_and_fetch(pos, DIMENSION, fluid_block(FluidKind::Water, 1))
                    .expect("fill water");
                positions.push(pos);
            }
        }
        println!("batch size: {} positions", positions.len());

        // Warm the cache once so neither path pays generation cost during timing.
        prewarm_chunks(global, &positions);

        let iterations = 20;

        let t0 = Instant::now();
        for _ in 0..iterations {
            let _ = evaluate_serial(global, &positions);
        }
        let serial = t0.elapsed() / iterations;

        let t1 = Instant::now();
        for _ in 0..iterations {
            let _ = evaluate_parallel(global, &positions);
        }
        let parallel = t1.elapsed() / iterations;

        println!("serial avg:   {:?}", serial);
        println!("parallel avg: {:?}", parallel);
        println!(
            "speedup: {:.2}x",
            serial.as_secs_f64() / parallel.as_secs_f64()
        );
    }
}
