//! Scheduled block ticks.
//!
//! Some block behaviour does not happen instantly: fluids spread a few ticks after being
//! disturbed, and other mechanics (planned for later) tick on their own cadence. This module
//! provides the bookkeeping for "do something at this block position N ticks from now".
//!
//! # Design
//!
//! Scheduled ticks are partitioned **per chunk**. Each [`ChunkPos`] owns its own queue of pending
//! ticks. This partitioning is deliberate: it lets a future parallel fluid stage process disjoint
//! sets of chunks on separate threads without contending over a single global structure. The
//! scheduler itself does no locking; callers decide how to share it (for example, behind the
//! existing chunk cache or a dedicated resource).
//!
//! Scheduling is **idempotent per `(position, kind)` within a tick bucket**: scheduling the same
//! block for the same work at an already-pending time does not create duplicate entries. This
//! mirrors vanilla, where a block cannot have two identical pending ticks, and keeps the queues
//! from growing without bound when many neighbours re-trigger the same block.

use crate::pos::{BlockPos, ChunkPos};
use std::collections::{HashMap, HashSet};

/// The category of work a scheduled tick performs.
///
/// Kept separate from the fluid module so the scheduler has no dependency on fluid specifics;
/// new tick kinds (redstone, crops, random block ticks) can be added here without touching the
/// queue machinery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TickKind {
    /// A fluid block should re-evaluate its spread.
    FluidSpread,
}

/// A single pending block update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScheduledTick {
    pub pos: BlockPos,
    pub kind: TickKind,
    /// The absolute tick number (from `TickCounter`) at which this should run.
    pub target_tick: u64,
}

/// Per-chunk queue of pending ticks.
///
/// Entries are kept in a flat vector and filtered by due time on drain. A dedup set guards against
/// inserting an identical `(pos, kind, target_tick)` more than once. For the small per-chunk
/// populations expected from fluid spreading this is cheaper and simpler than a binary heap; the
/// structure can be upgraded later if profiling shows it matters.
#[derive(Debug, Default)]
struct ChunkTickQueue {
    pending: Vec<ScheduledTick>,
    seen: HashSet<(BlockPos, TickKind, u64)>,
}

impl ChunkTickQueue {
    fn schedule(&mut self, tick: ScheduledTick) -> bool {
        let key = (tick.pos, tick.kind, tick.target_tick);
        if self.seen.insert(key) {
            self.pending.push(tick);
            true
        } else {
            false
        }
    }

    fn drain_due(&mut self, current_tick: u64, out: &mut Vec<ScheduledTick>) {
        // Partition into due / not-yet-due, retaining the latter.
        let mut remaining = Vec::with_capacity(self.pending.len());
        for tick in self.pending.drain(..) {
            if tick.target_tick <= current_tick {
                self.seen.remove(&(tick.pos, tick.kind, tick.target_tick));
                out.push(tick);
            } else {
                remaining.push(tick);
            }
        }
        self.pending = remaining;
    }

    /// Drains at most `budget` due ticks into `out`, leaving any remaining due ticks queued (they
    /// stay due and will be returned by a later drain). Returns how many were drained.
    fn drain_due_capped(
        &mut self,
        current_tick: u64,
        out: &mut Vec<ScheduledTick>,
        budget: usize,
    ) -> usize {
        let mut taken = 0;
        let mut remaining = Vec::with_capacity(self.pending.len());
        for tick in self.pending.drain(..) {
            if taken < budget && tick.target_tick <= current_tick {
                self.seen.remove(&(tick.pos, tick.kind, tick.target_tick));
                out.push(tick);
                taken += 1;
            } else {
                remaining.push(tick);
            }
        }
        self.pending = remaining;
        taken
    }

    fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

/// Scheduler holding per-chunk tick queues.
///
/// This is a plain data structure with no internal synchronization. It is intended to live in a
/// single owner (e.g. an ECS resource) and be advanced once per game tick.
#[derive(Debug, Default)]
pub struct BlockTickScheduler {
    chunks: HashMap<ChunkPos, ChunkTickQueue>,
}

impl BlockTickScheduler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Schedules `kind` work at `pos` to run `delay` ticks after `current_tick`.
    ///
    /// A `delay` of 0 schedules the work for the current tick (it will be returned by the next
    /// [`drain_due`](Self::drain_due) call for `current_tick`). Returns `true` if a new tick was
    /// added, or `false` if an identical tick was already pending.
    pub fn schedule(
        &mut self,
        pos: BlockPos,
        kind: TickKind,
        current_tick: u64,
        delay: u64,
    ) -> bool {
        let target_tick = current_tick.saturating_add(delay);
        let chunk = pos.chunk();
        self.chunks
            .entry(chunk)
            .or_default()
            .schedule(ScheduledTick {
                pos,
                kind,
                target_tick,
            })
    }

    /// Removes and returns every tick due at or before `current_tick`, grouped by chunk.
    ///
    /// Only chunks that actually have due ticks appear in the result. Chunks whose queues become
    /// empty are dropped to keep the map from accumulating idle entries. Grouping by chunk lets the
    /// caller hand each chunk's work to a separate worker.
    pub fn drain_due(&mut self, current_tick: u64) -> Vec<(ChunkPos, Vec<ScheduledTick>)> {
        let mut result = Vec::new();
        let mut emptied = Vec::new();

        for (chunk_pos, queue) in self.chunks.iter_mut() {
            let mut due = Vec::new();
            queue.drain_due(current_tick, &mut due);
            if !due.is_empty() {
                result.push((*chunk_pos, due));
            }
            if queue.is_empty() {
                emptied.push(*chunk_pos);
            }
        }

        for chunk_pos in emptied {
            self.chunks.remove(&chunk_pos);
        }

        result
    }

    /// Like [`drain_due`](Self::drain_due) but drains at most `max_ticks` due ticks in total this
    /// call, leaving any remaining due ticks queued for a later call. This lets the caller bound how
    /// much work a single game tick performs, so a large fluid cascade is spread across several ticks
    /// (settling a little slower) instead of freezing one tick for hundreds of milliseconds.
    ///
    /// Chunks are visited in map order until the budget is exhausted, so a chunk with a huge backlog
    /// can defer later chunks to subsequent ticks; forward progress is still guaranteed because every
    /// remaining tick stays due. `max_ticks == 0` means unbounded (equivalent to `drain_due`).
    pub fn drain_due_capped(
        &mut self,
        current_tick: u64,
        max_ticks: usize,
    ) -> Vec<(ChunkPos, Vec<ScheduledTick>)> {
        if max_ticks == 0 {
            return self.drain_due(current_tick);
        }
        let mut result = Vec::new();
        let mut emptied = Vec::new();
        let mut budget = max_ticks;

        for (chunk_pos, queue) in self.chunks.iter_mut() {
            if budget > 0 {
                let mut due = Vec::new();
                let taken = queue.drain_due_capped(current_tick, &mut due, budget);
                budget -= taken;
                if !due.is_empty() {
                    result.push((*chunk_pos, due));
                }
            }
            if queue.is_empty() {
                emptied.push(*chunk_pos);
            }
        }

        for chunk_pos in emptied {
            self.chunks.remove(&chunk_pos);
        }

        result
    }

    /// Total number of chunks that currently have pending ticks. Primarily for diagnostics.
    pub fn active_chunk_count(&self) -> usize {
        self.chunks.len()
    }

    /// Total number of pending ticks across all chunks. Primarily for diagnostics.
    pub fn pending_count(&self) -> usize {
        self.chunks.values().map(|q| q.pending.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(x: i32, y: i32, z: i32) -> BlockPos {
        BlockPos::of(x, y, z)
    }

    #[test]
    fn schedule_then_drain_at_target() {
        let mut sched = BlockTickScheduler::new();
        sched.schedule(pos(0, 64, 0), TickKind::FluidSpread, 100, 5);

        // Nothing due before the target tick.
        assert!(sched.drain_due(104).is_empty());
        assert_eq!(sched.pending_count(), 1);

        // Due exactly at the target tick.
        let due = sched.drain_due(105);
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].1.len(), 1);
        assert_eq!(due[0].1[0].pos, pos(0, 64, 0));
        assert_eq!(sched.pending_count(), 0);
    }

    #[test]
    fn drain_due_capped_bounds_and_defers() {
        let mut sched = BlockTickScheduler::new();
        // Five ticks all due at tick 1, spread across two chunks so the budget must span chunks.
        for i in 0..5 {
            sched.schedule(pos(i, 64, 0), TickKind::FluidSpread, 0, 1);
        }
        for i in 0..5 {
            sched.schedule(pos(100 + i, 64, 0), TickKind::FluidSpread, 0, 1);
        }
        assert_eq!(sched.pending_count(), 10);

        // A capped drain returns at most the budget, leaving the rest still due.
        let first: usize = sched
            .drain_due_capped(1, 4)
            .iter()
            .map(|(_, t)| t.len())
            .sum();
        assert_eq!(first, 4, "capped drain must not exceed the budget");
        assert_eq!(sched.pending_count(), 6, "the rest stay queued and due");

        let second: usize = sched
            .drain_due_capped(1, 4)
            .iter()
            .map(|(_, t)| t.len())
            .sum();
        assert_eq!(second, 4);
        let third: usize = sched
            .drain_due_capped(1, 4)
            .iter()
            .map(|(_, t)| t.len())
            .sum();
        assert_eq!(third, 2, "only the remaining due ticks are returned");
        assert_eq!(sched.pending_count(), 0);

        // A budget of 0 means unbounded.
        sched.schedule(pos(0, 64, 0), TickKind::FluidSpread, 0, 1);
        sched.schedule(pos(1, 64, 0), TickKind::FluidSpread, 0, 1);
        let all: usize = sched
            .drain_due_capped(1, 0)
            .iter()
            .map(|(_, t)| t.len())
            .sum();
        assert_eq!(all, 2);
    }

    #[test]
    fn dedup_identical_ticks() {
        let mut sched = BlockTickScheduler::new();
        let first = sched.schedule(pos(1, 64, 1), TickKind::FluidSpread, 0, 5);
        let second = sched.schedule(pos(1, 64, 1), TickKind::FluidSpread, 0, 5);
        assert!(first);
        assert!(!second, "identical pending tick should be deduplicated");
        assert_eq!(sched.pending_count(), 1);
    }

    #[test]
    fn different_target_ticks_not_deduped() {
        let mut sched = BlockTickScheduler::new();
        sched.schedule(pos(1, 64, 1), TickKind::FluidSpread, 0, 5);
        sched.schedule(pos(1, 64, 1), TickKind::FluidSpread, 0, 6);
        assert_eq!(sched.pending_count(), 2);
    }

    #[test]
    fn groups_by_chunk() {
        let mut sched = BlockTickScheduler::new();
        // Two positions in different chunks (16 blocks apart horizontally).
        sched.schedule(pos(0, 64, 0), TickKind::FluidSpread, 0, 1);
        sched.schedule(pos(32, 64, 0), TickKind::FluidSpread, 0, 1);
        sched.schedule(pos(1, 64, 0), TickKind::FluidSpread, 0, 1); // same chunk as first

        let mut due = sched.drain_due(1);
        due.sort_by_key(|(c, _)| c.x());
        assert_eq!(due.len(), 2, "two distinct chunks should be present");
        // First chunk has two ticks, second has one.
        assert_eq!(due[0].1.len(), 2);
        assert_eq!(due[1].1.len(), 1);
    }

    #[test]
    fn drain_leaves_future_ticks() {
        let mut sched = BlockTickScheduler::new();
        sched.schedule(pos(0, 64, 0), TickKind::FluidSpread, 0, 1);
        sched.schedule(pos(0, 65, 0), TickKind::FluidSpread, 0, 10);

        let due = sched.drain_due(1);
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].1.len(), 1);
        assert_eq!(sched.pending_count(), 1);

        // Re-scheduling the drained position is allowed again (dedup entry was cleared).
        assert!(sched.schedule(pos(0, 64, 0), TickKind::FluidSpread, 1, 1));
    }

    #[test]
    fn empty_chunks_are_pruned() {
        let mut sched = BlockTickScheduler::new();
        sched.schedule(pos(0, 64, 0), TickKind::FluidSpread, 0, 1);
        assert_eq!(sched.active_chunk_count(), 1);
        sched.drain_due(1);
        assert_eq!(sched.active_chunk_count(), 0);
    }
}
