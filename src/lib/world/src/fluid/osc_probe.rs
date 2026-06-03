//! TEMPORARY staircase trace. DELETE after investigation.
#[cfg(test)]
mod probe {
    use crate::block_state_id::BlockStateId;
    use crate::dimension::Dimension;
    use crate::fluid::spread::{fluid_neighbours, BlockView};
    use crate::fluid::{compute_tick, fluid_block, fluid_state, FluidKind, FluidRules};
    use crate::pos::BlockPos;
    use crate::scheduler::{BlockTickScheduler, TickKind};
    use ferrumc_config::server_config::FluidAlgorithm;
    use ferrumc_macros::block;
    use std::collections::HashMap;

    struct W { b: HashMap<(i32, i32, i32), BlockStateId> }
    impl W {
        fn new() -> Self { Self { b: HashMap::new() } }
        fn set(&mut self, p: BlockPos, x: BlockStateId) { self.b.insert((p.pos.x, p.pos.y, p.pos.z), x); }
        fn get(&self, p: BlockPos) -> BlockStateId {
            self.b.get(&(p.pos.x, p.pos.y, p.pos.z)).copied().unwrap_or_else(|| block!("air"))
        }
    }
    impl BlockView for &W { fn block_at(&self, p: BlockPos) -> BlockStateId { self.get(p) } }
    fn pp(x: i32, y: i32, z: i32) -> BlockPos { BlockPos::of(x, y, z) }

    #[test]
    fn trace_staircase() {
        const DIM: Dimension = Dimension::Overworld;
        let mut world = W::new();
        let mut sched = BlockTickScheduler::new();
        for i in 0..4i32 {
            let floor_y = 66 - i;
            let x0 = i * 3;
            for x in x0..x0 + 3 {
                for z in -1..=1 {
                    world.set(pp(x, floor_y, z), block!("stone"));
                }
            }
            for x in x0..x0 + 3 {
                world.set(pp(x, floor_y + 1, -2), block!("stone"));
                world.set(pp(x, floor_y + 1, 2), block!("stone"));
            }
        }
        for y in 63..=66 {
            for z in -2..=2 {
                world.set(pp(12, y, z), block!("stone"));
            }
        }
        world.set(pp(-1, 67, 0), block!("stone"));
        world.set(pp(-1, 66, 0), block!("stone"));
        let source = pp(0, 67, 0);
        world.set(source, fluid_block(FluidKind::Water, 0));
        sched.schedule(source, TickKind::FluidSpread, 0, 0);

        for tick in 0..2000u64 {
            let due = sched.drain_due(tick);
            if due.is_empty() { continue; }
            let mut changes = Vec::new();
            for (_c, ts) in &due {
                for t in ts {
                    let blk = world.get(t.pos);
                    let Some(s) = fluid_state(blk) else { continue };
                    let rules = FluidRules::for_kind(s.kind, DIM);
                    changes.extend(compute_tick(FluidAlgorithm::Vanilla, t.pos, &&world, rules));
                }
            }
            for c in &changes {
                world.set(c.pos, c.new_block);
                if c.reschedule {
                    let dd = fluid_state(c.new_block).map(|s| FluidRules::for_kind(s.kind, DIM).tick_delay).unwrap_or(5);
                    sched.schedule(c.pos, TickKind::FluidSpread, tick, dd);
                }
                for n in fluid_neighbours(c.pos) {
                    if let Some(s) = fluid_state(world.get(n)) {
                        let dd = FluidRules::for_kind(s.kind, DIM).tick_delay;
                        sched.schedule(n, TickKind::FluidSpread, tick, dd);
                    }
                }
            }
        }

        // Dump every water cell.
        let mut cells: Vec<_> = world.b.iter()
            .filter(|(_, &b)| fluid_state(b).is_some())
            .map(|(&k, &b)| (k, fluid_state(b).unwrap().level))
            .collect();
        cells.sort();
        println!("water cells after 2000 ticks ({}):", cells.len());
        for (k, lvl) in &cells {
            println!("  ({},{},{}) level {}", k.0, k.1, k.2, lvl);
        }
        println!("pending: {}", sched.pending_count());
    }
}
