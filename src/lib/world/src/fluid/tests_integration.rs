//! Integration tests that drive the full fluid loop over multiple ticks.
//!
//! These exercise the [`scheduler`](crate::scheduler) and the spreading
//! [`algorithm`](crate::fluid::spread) together against a mutable in-memory world, replicating the
//! drain → evaluate → apply → re-schedule cycle the server runs each tick (minus the ECS and
//! networking). They verify that spreading converges and respects terrain, which the per-function
//! unit tests cannot show on their own.

use crate::block_state_id::BlockStateId;
use crate::fluid::spread::{compute_fluid_tick, BlockView};
use crate::fluid::{fluid_block, fluid_state, FluidKind};
use crate::pos::BlockPos;
use crate::scheduler::{BlockTickScheduler, TickKind};
use ferrumc_macros::block;
use std::collections::HashMap;

/// A mutable map-backed world. Missing positions read as air.
struct MapWorld {
    blocks: HashMap<(i32, i32, i32), BlockStateId>,
}

impl MapWorld {
    fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    fn set(&mut self, pos: BlockPos, block: BlockStateId) {
        self.blocks.insert((pos.pos.x, pos.pos.y, pos.pos.z), block);
    }

    fn get(&self, pos: BlockPos) -> BlockStateId {
        self.blocks
            .get(&(pos.pos.x, pos.pos.y, pos.pos.z))
            .copied()
            .unwrap_or_else(|| block!("air"))
    }
}

impl BlockView for &MapWorld {
    fn block_at(&self, pos: BlockPos) -> BlockStateId {
        self.get(pos)
    }
}

fn p(x: i32, y: i32, z: i32) -> BlockPos {
    BlockPos::of(x, y, z)
}

/// Runs the full scheduler-driven fluid loop for `ticks` game ticks, returning the number of block
/// changes applied across the whole run. Mirrors the server's `process_fluid_ticks`.
fn run(world: &mut MapWorld, scheduler: &mut BlockTickScheduler, start_tick: u64, ticks: u64) -> usize {
    const DELAY: u64 = 5;
    let mut total_changes = 0;

    for offset in 0..ticks {
        let current = start_tick + offset;
        let due = scheduler.drain_due(current);
        if due.is_empty() {
            continue;
        }

        // Read phase: evaluate all due ticks against the current world.
        let mut changes = Vec::new();
        for (_chunk, scheduled) in &due {
            for tick in scheduled {
                changes.extend(compute_fluid_tick(tick.pos, &&*world));
            }
        }

        // Write phase: apply changes and re-schedule.
        for change in &changes {
            world.set(change.pos, change.new_block);
            total_changes += 1;
            if change.reschedule {
                scheduler.schedule(change.pos, TickKind::FluidSpread, current, DELAY);
            }
        }
    }

    total_changes
}

#[test]
fn water_source_spreads_across_flat_floor_over_time() {
    let mut world = MapWorld::new();
    let mut scheduler = BlockTickScheduler::new();

    // Build a flat stone floor at y=63 across a small area.
    for x in -8..=8 {
        for z in -8..=8 {
            world.set(p(x, 63, z), block!("stone"));
        }
    }
    // Place a water source at the centre, on the floor.
    let source = p(0, 64, 0);
    world.set(source, fluid_block(FluidKind::Water, 0));
    scheduler.schedule(source, TickKind::FluidSpread, 0, 5);

    // Immediately after seeding, only the source exists.
    assert!(fluid_state(world.get(p(1, 64, 0))).is_none());

    // Run enough ticks for water to spread outward several blocks.
    run(&mut world, &mut scheduler, 0, 60);

    // The four direct neighbours should now hold flowing water.
    for n in [p(1, 64, 0), p(-1, 64, 0), p(0, 64, 1), p(0, 64, -1)] {
        let state = fluid_state(world.get(n))
            .unwrap_or_else(|| panic!("expected water at {:?}", n.pos));
        assert_eq!(state.kind, FluidKind::Water);
        assert!(!state.is_source(), "neighbour should be flowing, not source");
    }
    // The source itself is untouched.
    assert!(fluid_state(world.get(source)).unwrap().is_source());
}

#[test]
fn water_falls_then_spreads_on_landing() {
    let mut world = MapWorld::new();
    let mut scheduler = BlockTickScheduler::new();

    // Floor at y=60. Source up at y=64 with air between.
    for x in -4..=4 {
        for z in -4..=4 {
            world.set(p(x, 60, z), block!("stone"));
        }
    }
    let source = p(0, 64, 0);
    world.set(source, fluid_block(FluidKind::Water, 0));
    scheduler.schedule(source, TickKind::FluidSpread, 0, 5);

    run(&mut world, &mut scheduler, 0, 120);

    // Water should have fallen down the column to just above the floor.
    for y in 61..=64 {
        let state = fluid_state(world.get(p(0, y, 0)));
        assert!(state.is_some(), "expected water in column at y={}", y);
    }
    // And spread along the floor at y=61.
    let neighbour = fluid_state(world.get(p(1, 61, 0)));
    assert!(
        neighbour.is_some(),
        "water should spread along the floor after falling"
    );
}

#[test]
fn water_does_not_pass_through_walls() {
    let mut world = MapWorld::new();
    let mut scheduler = BlockTickScheduler::new();

    // Floor at y=63.
    for x in -4..=4 {
        for z in -4..=4 {
            world.set(p(x, 63, z), block!("stone"));
        }
    }
    // A wall one block east of the source.
    world.set(p(1, 64, 0), block!("stone"));

    let source = p(0, 64, 0);
    world.set(source, fluid_block(FluidKind::Water, 0));
    scheduler.schedule(source, TickKind::FluidSpread, 0, 5);

    run(&mut world, &mut scheduler, 0, 60);

    // The wall remains stone; water never overwrites it.
    assert_eq!(world.get(p(1, 64, 0)), block!("stone"));
    // Water did spread the other directions.
    assert!(fluid_state(world.get(p(-1, 64, 0))).is_some());
}

#[test]
fn run_eventually_reaches_a_steady_state() {
    let mut world = MapWorld::new();
    let mut scheduler = BlockTickScheduler::new();

    // Fully enclosed basin so water cannot escape and must settle:
    // - 7x7 stone floor at y=63
    // - stone walls one block high around the perimeter (|x|==3 or |z|==3) at y=64
    // This leaves a 5x5 interior (x,z in -2..=2) for water to fill.
    for x in -3..=3 {
        for z in -3..=3 {
            world.set(p(x, 63, z), block!("stone"));
            if x.abs() == 3 || z.abs() == 3 {
                world.set(p(x, 64, z), block!("stone"));
            }
        }
    }
    let source = p(0, 64, 0);
    world.set(source, fluid_block(FluidKind::Water, 0));
    scheduler.schedule(source, TickKind::FluidSpread, 0, 5);

    // Run a long time, then confirm no further changes occur (steady state).
    run(&mut world, &mut scheduler, 0, 200);
    let changes_after_settle = run(&mut world, &mut scheduler, 200, 60);
    assert_eq!(
        changes_after_settle, 0,
        "fluid should reach a steady state with no further changes"
    );
}

#[test]
fn steady_state_stops_rescheduling() {
    // Guards against a performance footgun: once a closed basin settles, the scheduler must
    // empty out rather than re-queueing no-op ticks forever.
    let mut world = MapWorld::new();
    let mut scheduler = BlockTickScheduler::new();

    for x in -3..=3 {
        for z in -3..=3 {
            world.set(p(x, 63, z), block!("stone"));
            if x.abs() == 3 || z.abs() == 3 {
                world.set(p(x, 64, z), block!("stone"));
            }
        }
    }
    let source = p(0, 64, 0);
    world.set(source, fluid_block(FluidKind::Water, 0));
    scheduler.schedule(source, TickKind::FluidSpread, 0, 5);

    run(&mut world, &mut scheduler, 0, 400);
    assert_eq!(
        scheduler.pending_count(),
        0,
        "scheduler should drain to empty once fluid settles"
    );
}
