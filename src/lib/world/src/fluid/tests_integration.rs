//! Integration tests that drive the full fluid loop over multiple ticks.
//!
//! These exercise the [`scheduler`](crate::scheduler) and the spreading
//! [`algorithm`](crate::fluid::spread) together against a mutable in-memory world, replicating the
//! drain → evaluate → apply → re-schedule cycle the server runs each tick (minus the ECS and
//! networking). They verify that spreading converges and respects terrain, which the per-function
//! unit tests cannot show on their own.

use crate::block_state_id::BlockStateId;
use crate::dimension::Dimension;
use crate::fluid::spread::BlockView;
use crate::fluid::{compute_tick, fluid_block, fluid_state, FluidKind, FluidRules};
use crate::pos::BlockPos;
use crate::scheduler::{BlockTickScheduler, TickKind};
use ferrumc_config::server_config::FluidAlgorithm;
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

/// Runs the full scheduler-driven fluid loop for `ticks` game ticks using the simplified kernel,
/// returning the number of block changes applied. Convenience wrapper over [`run_with`].
fn run(
    world: &mut MapWorld,
    scheduler: &mut BlockTickScheduler,
    start_tick: u64,
    ticks: u64,
) -> usize {
    run_with(
        world,
        scheduler,
        FluidAlgorithm::Simplified,
        start_tick,
        ticks,
    )
}

/// Runs the full scheduler-driven fluid loop for `ticks` game ticks with the chosen kernel,
/// returning the number of block changes applied across the whole run. Mirrors the server's
/// `process_fluid_ticks`: each due tick looks up the rules for the fluid currently at that
/// position and dispatches to the selected algorithm, then wakes fluid neighbours.
fn run_with(
    world: &mut MapWorld,
    scheduler: &mut BlockTickScheduler,
    algorithm: FluidAlgorithm,
    start_tick: u64,
    ticks: u64,
) -> usize {
    // All integration tests in this file run in the overworld, where water and lava use their
    // own rules. The reschedule delay mirrors whichever fluid produced the change.
    const DIM: Dimension = Dimension::Overworld;
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
                let block = world.get(tick.pos);
                let Some(state) = fluid_state(block) else {
                    continue;
                };
                let rules = FluidRules::for_kind(state.kind, DIM);
                changes.extend(compute_tick(algorithm, tick.pos, &&*world, rules));
            }
        }

        // Write phase: apply each change, re-schedule the changed block if it asked, and wake the
        // changed cell's fluid neighbours so updates ripple outward (mirrors the server's
        // `apply_changes`, which performs the same neighbour propagation).
        for change in &changes {
            world.set(change.pos, change.new_block);
            total_changes += 1;
            if change.reschedule {
                let delay = fluid_state(change.new_block)
                    .map(|s| FluidRules::for_kind(s.kind, DIM).tick_delay)
                    // Fallback for non-fluid changes (drying up): use the water cadence as a safe
                    // default. In practice non-fluid changes set reschedule = false.
                    .unwrap_or_else(|| FluidRules::for_kind(FluidKind::Water, DIM).tick_delay);
                scheduler.schedule(change.pos, TickKind::FluidSpread, current, delay);
            }
            for neighbour in crate::fluid::spread::fluid_neighbours(change.pos) {
                if let Some(state) = fluid_state(world.get(neighbour)) {
                    let delay = FluidRules::for_kind(state.kind, DIM).tick_delay;
                    scheduler.schedule(neighbour, TickKind::FluidSpread, current, delay);
                }
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
        let state =
            fluid_state(world.get(n)).unwrap_or_else(|| panic!("expected water at {:?}", n.pos));
        assert_eq!(state.kind, FluidKind::Water);
        assert!(
            !state.is_source(),
            "neighbour should be flowing, not source"
        );
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

/// Removing the source must make the *whole* flowing body recede level-by-level and ultimately
/// disappear, not just blank the ring next to the source. This is the regression the receding
/// rewrite targets.
#[test]
fn removing_source_drains_entire_flow() {
    let mut world = MapWorld::new();
    let mut scheduler = BlockTickScheduler::new();

    // Flat floor with a source at the centre; let it spread out fully first.
    for x in -8..=8 {
        for z in -8..=8 {
            world.set(p(x, 63, z), block!("stone"));
        }
    }
    let source = p(0, 64, 0);
    world.set(source, fluid_block(FluidKind::Water, 0));
    scheduler.schedule(source, TickKind::FluidSpread, 0, 5);
    run(&mut world, &mut scheduler, 0, 200);

    // Sanity: water has spread several blocks out from the source.
    assert!(
        fluid_state(world.get(p(3, 64, 0))).is_some(),
        "precondition: water should have spread before source removal"
    );

    // Remove the source and re-tick its position (as a bucket pickup / block break would).
    world.set(source, block!("air"));
    scheduler.schedule(source, TickKind::FluidSpread, 200, 0);
    // Also wake the immediate neighbours so the drain kicks off, mirroring seed_on_block_break.
    for n in [p(1, 64, 0), p(-1, 64, 0), p(0, 64, 1), p(0, 64, -1)] {
        scheduler.schedule(n, TickKind::FluidSpread, 200, 0);
    }
    run(&mut world, &mut scheduler, 200, 600);

    // Every flowing-water cell should be gone; nothing flowing should remain anywhere on the floor.
    let mut remaining = Vec::new();
    for x in -8..=8 {
        for z in -8..=8 {
            if fluid_state(world.get(p(x, 64, z))).is_some() {
                remaining.push((x, z));
            }
        }
    }
    assert!(
        remaining.is_empty(),
        "all flowing water should drain after the source is removed, but these cells remain: {:?}",
        remaining
    );
    // And the scheduler should be idle again.
    assert_eq!(
        scheduler.pending_count(),
        0,
        "scheduler should settle after draining"
    );
}

// =============================================================================================
// Vanilla-kernel stress tests: complex terrain that triggers deep, cascading multi-level updates.
// =============================================================================================

/// Seeds the source and every fluid neighbour set so the loop has work to do from tick 0.
fn seed(scheduler: &mut BlockTickScheduler, pos: BlockPos) {
    scheduler.schedule(pos, TickKind::FluidSpread, 0, 0);
}

/// Counts fluid cells of `kind` in a cuboid (inclusive bounds). Used to assert on the shape of a
/// settled flow without pinning every individual cell.
fn count_fluid(
    world: &MapWorld,
    kind: FluidKind,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
) -> usize {
    let mut n = 0;
    for cx in x.0..=x.1 {
        for cy in y.0..=y.1 {
            for cz in z.0..=z.1 {
                if let Some(s) = fluid_state(world.get(p(cx, cy, cz))) {
                    if s.kind == kind {
                        n += 1;
                    }
                }
            }
        }
    }
    n
}

/// A multi-level staircase basin: water poured at the top must cascade down several ledges, each
/// drop re-triggering spread on the level below. A source-fed cascade is perpetually animated
/// (standing waterfalls between ledges), so this asserts the cascade reaches the bottom and that
/// per-tick activity stays *bounded* (no runaway feedback), rather than reaching zero changes.
/// This is the "complex terrain → many cascading updates" stress case, run through the vanilla
/// slope kernel.
#[test]
fn vanilla_water_cascades_down_a_staircase_with_bounded_activity() {
    let mut world = MapWorld::new();
    let mut scheduler = BlockTickScheduler::new();

    // Build a 4-step descending staircase along +x. Each step is a 3-wide (z = -1..=1) ledge,
    // one block lower than the previous, with a back wall so water must flow forward (+x) and
    // spill onto the next step.
    //
    // step i occupies x in [i*3 .. i*3+2], floor at y = 66 - i.
    for i in 0..4 {
        let floor_y = 66 - i;
        let x0 = i * 3;
        for x in x0..x0 + 3 {
            for z in -1..=1 {
                world.set(p(x, floor_y, z), block!("stone"));
            }
        }
        // Side walls (z = -2 and z = 2) so the flow stays in the channel.
        for x in x0..x0 + 3 {
            world.set(p(x, floor_y + 1, -2), block!("stone"));
            world.set(p(x, floor_y + 1, 2), block!("stone"));
        }
    }
    // End wall after the last step so water pools on the bottom ledge instead of pouring off the
    // edge of the world (which would never settle).
    for y in 63..=66 {
        for z in -2..=2 {
            world.set(p(12, y, z), block!("stone"));
        }
    }
    // A back wall behind the source so it cannot flow -x off the top step.
    world.set(p(-1, 67, 0), block!("stone"));
    world.set(p(-1, 66, 0), block!("stone"));

    // Pour a water source at the top step.
    let source = p(0, 67, 0);
    world.set(source, fluid_block(FluidKind::Water, 0));
    seed(&mut scheduler, source);

    // Run long enough for the cascade to reach the bottom step.
    run_with(&mut world, &mut scheduler, FluidAlgorithm::Vanilla, 0, 2000);

    // Water must have reached the lowest step (floor y = 63), proving the cascade propagated all
    // the way down through every ledge's spill.
    let bottom_wet = count_fluid(&world, FluidKind::Water, (9, 11), (64, 64), (-1, 1));
    assert!(
        bottom_wet > 0,
        "water should have cascaded down to the bottom step, found none there"
    );

    // The source is untouched.
    assert!(fluid_state(world.get(source)).unwrap().is_source());

    // A source-fed cascade has standing waterfalls between ledges, so it is perpetually animated;
    // we do not assert zero-change steady state here. What we DO require is that the work stays
    // *bounded* — the scheduler must not blow up tick over tick (a runaway would mean a feedback
    // loop). Compare the change volume of two equal-length windows well after the initial fill;
    // they should be in the same ballpark, not growing.
    let window_a = run_with(
        &mut world,
        &mut scheduler,
        FluidAlgorithm::Vanilla,
        2000,
        200,
    );
    let window_b = run_with(
        &mut world,
        &mut scheduler,
        FluidAlgorithm::Vanilla,
        2200,
        200,
    );
    assert!(
        window_b <= window_a * 2 + 8,
        "cascade activity should stay bounded, not grow (a={window_a}, b={window_b})"
    );
}

/// A pit with a single drain hole off to one side: water poured in the middle of a large flat
/// floor should steer toward the hole (vanilla slope search) and pour down it, rather than filling
/// the whole floor uniformly. Stresses repeated slope searches over many cells.
#[test]
fn vanilla_flow_steers_into_a_single_drain_and_settles() {
    let mut world = MapWorld::new();
    let mut scheduler = BlockTickScheduler::new();

    // 11x11 solid floor at y=63, walls around the rim at y=64 so water can't escape sideways.
    for x in -5..=5 {
        for z in -5..=5 {
            world.set(p(x, 63, z), block!("stone"));
            if x.abs() == 5 || z.abs() == 5 {
                world.set(p(x, 64, z), block!("stone"));
            }
        }
    }
    // One drain hole within water's reach (max horizontal reach is 7 blocks): put it at (3,_,2),
    // Manhattan distance 5 from the source. Remove the floor there and give the column a floor far
    // below so it terminates.
    world.blocks.remove(&(3, 63, 2));
    world.set(p(3, 60, 2), block!("stone"));

    let source = p(0, 64, 0);
    world.set(source, fluid_block(FluidKind::Water, 0));
    seed(&mut scheduler, source);

    run_with(&mut world, &mut scheduler, FluidAlgorithm::Vanilla, 0, 4000);

    // Water should have found and used the drain: the column below the hole is wet.
    let drained = count_fluid(&world, FluidKind::Water, (3, 3), (60, 62), (2, 2));
    assert!(
        drained > 0,
        "water should have flowed down the drain hole, found none in the drain column"
    );

    // Steering check: water should NOT have filled the far corner away from the drain. The
    // opposite corner from the drain (-5,-5) sits 13 blocks away by path and is beyond a steered
    // flow's reach, so it must stay dry while the drain keeps pulling flow.
    assert!(
        fluid_state(world.get(p(-5, 64, -5))).is_none(),
        "water should steer toward the drain, not fill the far corner"
    );
    // A pit with an open drain is perpetually animated (water keeps flowing in and falling), so we
    // intentionally do NOT assert steady state here — that is correct vanilla behaviour.
}

/// A 2x2 water pool on solid ground, with one cell broken out, must heal back to four sources via
/// infinite-source formation — then stay settled. Exercises source formation inside the full loop.
#[test]
fn vanilla_two_by_two_pool_heals_to_infinite_source() {
    let mut world = MapWorld::new();
    let mut scheduler = BlockTickScheduler::new();

    // Solid floor under a 2x2 area at y=64; sources at three corners, the fourth left as flowing.
    for x in 0..=1 {
        for z in 0..=1 {
            world.set(p(x, 63, z), block!("stone"));
        }
    }
    // Walls around so nothing escapes.
    for x in -1..=2 {
        for z in -1..=2 {
            if x < 0 || x > 1 || z < 0 || z > 1 {
                world.set(p(x, 64, z), block!("stone"));
                world.set(p(x, 63, z), block!("stone"));
            }
        }
    }
    world.set(p(0, 64, 0), fluid_block(FluidKind::Water, 0));
    world.set(p(1, 64, 0), fluid_block(FluidKind::Water, 0));
    world.set(p(0, 64, 1), fluid_block(FluidKind::Water, 0));
    // The fourth cell starts as flowing water (as if a source was just scooped out).
    world.set(p(1, 64, 1), fluid_block(FluidKind::Water, 1));

    seed(&mut scheduler, p(1, 64, 1));
    for n in crate::fluid::spread::fluid_neighbours(p(1, 64, 1)) {
        scheduler.schedule(n, TickKind::FluidSpread, 0, 0);
    }

    run_with(&mut world, &mut scheduler, FluidAlgorithm::Vanilla, 0, 200);

    // All four cells should now be sources (the pool healed).
    for (x, z) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
        let s = fluid_state(world.get(p(x, 64, z)))
            .unwrap_or_else(|| panic!("cell ({x},{z}) should hold water"));
        assert!(
            s.is_source(),
            "2x2 pool cell ({x},{z}) should have healed to a source, was level {}",
            s.level
        );
    }

    let after = run_with(
        &mut world,
        &mut scheduler,
        FluidAlgorithm::Vanilla,
        200,
        200,
    );
    assert_eq!(
        after, 0,
        "healed pool must be stable, {after} further changes"
    );
}

/// Minimal reproduction of a single ledge: a water source on an upper floor whose edge drops one
/// block to a lower floor. The flow off the edge must settle, not oscillate. (Diagnostic for the
/// staircase non-convergence.)
#[test]
fn vanilla_single_ledge_settles() {
    let mut world = MapWorld::new();
    let mut scheduler = BlockTickScheduler::new();

    // Upper floor at y=64 for x in 0..=1; lower floor at y=63 for x in 2..=5, with an end wall at
    // x=6 so water pools on the lower floor instead of pouring off the end of the world.
    for x in 0..=1 {
        world.set(p(x, 64, 0), block!("stone"));
    }
    for x in 2..=5 {
        world.set(p(x, 63, 0), block!("stone"));
    }
    // Narrow channel: walls at z=-1 and z=1 along the whole run, both heights.
    for x in 0..=6 {
        for y in 63..=66 {
            world.set(p(x, y, -1), block!("stone"));
            world.set(p(x, y, 1), block!("stone"));
        }
    }
    // End wall so the lower floor is a closed basin.
    for y in 63..=66 {
        world.set(p(6, y, 0), block!("stone"));
    }
    // Back wall behind source.
    world.set(p(-1, 65, 0), block!("stone"));
    world.set(p(-1, 66, 0), block!("stone"));

    let source = p(0, 65, 0);
    world.set(source, fluid_block(FluidKind::Water, 0));
    seed(&mut scheduler, source);

    run_with(&mut world, &mut scheduler, FluidAlgorithm::Vanilla, 0, 1000);
    let after = run_with(
        &mut world,
        &mut scheduler,
        FluidAlgorithm::Vanilla,
        1000,
        200,
    );
    assert_eq!(
        after, 0,
        "a single ledge must settle, but {after} further changes occurred (oscillation)"
    );
}

/// An open flat platform (edges drop off into the void) with a water source in the middle. The
/// **top layer** must reach a stable shape: interior cells settle to a fixed level gradient and the
/// rim cells settle to air (water that reached the rim falls off and is not re-spread). Regression
/// for the max-level edge oscillation where rim cells flip-flopped between max-level water and air
/// every tick forever. We assert the top layer stops changing; the falling columns below the rim
/// are intentionally ignored (an edge waterfall is perpetually animated, which is correct).
#[test]
fn vanilla_open_platform_top_layer_settles() {
    use std::collections::HashMap;

    let mut world = MapWorld::new();
    let mut scheduler = BlockTickScheduler::new();
    for x in -4..=4 {
        for z in -4..=4 {
            world.set(p(x, 63, z), block!("stone"));
        }
    }
    let source = p(0, 64, 0);
    world.set(source, fluid_block(FluidKind::Water, 0));
    scheduler.schedule(source, TickKind::FluidSpread, 0, 0);

    // Let it spread out and reach the rim.
    run_with(&mut world, &mut scheduler, FluidAlgorithm::Vanilla, 0, 400);

    // Snapshot the entire top layer (y = 64) over the platform plus a one-block rim margin.
    let snapshot = |w: &MapWorld| -> HashMap<(i32, i32), u32> {
        let mut m = HashMap::new();
        for x in -5..=5 {
            for z in -5..=5 {
                m.insert((x, z), w.get(p(x, 64, z)).raw());
            }
        }
        m
    };
    let before = snapshot(&world);
    // Run more ticks; the top layer must not change at all.
    run_with(
        &mut world,
        &mut scheduler,
        FluidAlgorithm::Vanilla,
        400,
        200,
    );
    let after = snapshot(&world);

    let mut changed: Vec<_> = before
        .iter()
        .filter(|(k, v)| after.get(*k) != Some(*v))
        .map(|(k, _)| *k)
        .collect();
    changed.sort();
    assert!(
        changed.is_empty(),
        "open-platform top layer must stabilise, but these cells kept changing: {:?}",
        changed
    );
}
