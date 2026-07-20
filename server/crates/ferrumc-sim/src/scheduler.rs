//! Deterministic scheduling internals for the non-authoritative multi-shard
//! shadow path.
//!
//! The scheduler is the simulation tick flow's "run active shards" seam. The
//! current shard still drains its admitted bounded inbox inside
//! [`SimShard::run_tick`]. Cross-shard intents now return with worker results,
//! enter one scheduler-owned bounded queue in canonical order, and become a
//! separate destination prefix at exactly the next tick boundary. Storage
//! overlay output now leaves a successful tick as move-owned, storage-bounded
//! per-shard batches after a full-key alias preflight. Storage completions,
//! plugin tasks, and metrics do not yet have scheduler-owned carriers; later
//! packets add those real values without fake no-op phases.
//!
//! The default mode calls the existing single-shard primitive directly. The
//! non-default shadow mode builds a canonical worker-slot plan, transfers owned
//! shard batches to a fixed group of worker threads through capacity-one
//! channels, then restores ownership and canonical output order before the next
//! tick. Because a batch owns each shard value, plans reject duplicate ids, and
//! the scheduler waits for every batch to return, the same shard can never be
//! entered concurrently. Workers never send directly to one another or borrow a
//! destination shard.

use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroUsize;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread::{self, JoinHandle};

#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(test)]
use std::sync::{Arc, Barrier, Condvar, Mutex};

use ferrumc_core::Tick;
use ferrumc_storage::{ChunkKey, ChunkOverlayRecord, MAX_SAVE_BATCH};

use crate::coordinator::TickCoordinator;
use crate::cross_shard::{
    CrossShardEnvelope, CrossShardIntent, CrossShardPayload, CrossShardQueue, CrossShardRejection,
    CrossShardRejectionReason, PreparedBoundary,
};
use crate::error::{SimError, SimResult};
use crate::message::{GameInput, GameOutput};
use crate::ownership::{ShardId, ShardLifecycle, ShardLifecycleState};
use crate::shard::SimShard;

/// The crate-internal rollout switch for scheduler execution.
///
/// The default stays on the existing inline single-shard path. Shadow workers
/// must be selected explicitly and are not wired into the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum SchedulerMode {
    /// The authoritative, existing `SimShard::run_tick` path.
    #[default]
    AuthoritativeInline,
    /// The non-authoritative deterministic multi-shard worker pool.
    ShadowWorkers {
        /// Fixed number of persistent worker threads.
        worker_slots: NonZeroUsize,
    },
}

impl SchedulerMode {
    /// Builds the explicitly enabled worker mode.
    const fn shadow(worker_slots: NonZeroUsize) -> Self {
        Self::ShadowWorkers { worker_slots }
    }
}

/// A stable zero-based logical worker lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WorkerSlot(usize);

impl WorkerSlot {
    /// Returns the zero-based lane index.
    const fn index(self) -> usize {
        self.0
    }
}

/// One canonical shard visit in a tick execution plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WorkerVisit {
    shard: ShardId,
    /// `None` means the exact authoritative inline path; shadow visits always
    /// carry their stable logical worker assignment.
    slot: Option<WorkerSlot>,
}

/// A duplicate-free, canonically ordered plan for one tick.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ExecutionPlan {
    visits: Vec<WorkerVisit>,
}

impl ExecutionPlan {
    /// Builds a plan independent of registration order.
    ///
    /// Duplicate ids are rejected before any tick or shard state advances.
    /// Canonical `ShardId` ordering remains the semantic execution and output
    /// order regardless of logical worker count.
    fn for_ids(
        mode: SchedulerMode,
        shard_ids: impl IntoIterator<Item = ShardId>,
    ) -> SimResult<Self> {
        let mut ordered = BTreeSet::new();
        for shard in shard_ids {
            if !ordered.insert(shard) {
                return Err(SimError::DuplicateShardDispatch { shard });
            }
        }

        if matches!(mode, SchedulerMode::AuthoritativeInline) && ordered.len() > 1 {
            return Err(SimError::MultipleScheduledShardsDisabled {
                scheduled: ordered.len(),
            });
        }

        let visits = ordered
            .into_iter()
            .enumerate()
            .map(|(ordinal, shard)| {
                let slot = match mode {
                    SchedulerMode::AuthoritativeInline => None,
                    SchedulerMode::ShadowWorkers { worker_slots } => {
                        Some(WorkerSlot(ordinal % worker_slots.get()))
                    }
                };
                WorkerVisit { shard, slot }
            })
            .collect();
        Ok(Self { visits })
    }

    /// Returns the canonical visits in semantic execution order.
    fn visits(&self) -> &[WorkerVisit] {
        &self.visits
    }
}

/// Deterministically forces one worker to publish its completion before another
/// in scheduler tests.
#[cfg(test)]
#[derive(Debug)]
struct TestCompletionOrder {
    first: ShardId,
    first_done: Mutex<bool>,
    changed: Condvar,
    observed: Mutex<Vec<ShardId>>,
}

#[cfg(test)]
impl TestCompletionOrder {
    /// Creates an empty completion trace whose first entry must be `first`.
    fn new(first: ShardId) -> Self {
        Self {
            first,
            first_done: Mutex::new(false),
            changed: Condvar::new(),
            observed: Mutex::new(Vec::new()),
        }
    }

    /// Records `shard`, blocking the non-first worker until the first finished.
    fn finish(&self, shard: ShardId) {
        if shard == self.first {
            self.observed
                .lock()
                .expect("completion trace lock")
                .push(shard);
            let mut first_done = self.first_done.lock().expect("completion gate lock");
            *first_done = true;
            self.changed.notify_all();
            return;
        }

        let mut first_done = self.first_done.lock().expect("completion gate lock");
        while !*first_done {
            first_done = self.changed.wait(first_done).expect("completion gate wait");
        }
        drop(first_done);
        self.observed
            .lock()
            .expect("completion trace lock")
            .push(shard);
    }

    /// Returns the observed worker completion order.
    fn observed(&self) -> Vec<ShardId> {
        self.observed.lock().expect("completion trace lock").clone()
    }
}

/// Test-only per-shard overlap and completion instrumentation.
#[cfg(test)]
#[derive(Debug)]
struct TestWorkerProbe {
    shard: ShardId,
    start: Arc<Barrier>,
    active: AtomicUsize,
    max_active: AtomicUsize,
    global_active: Arc<AtomicUsize>,
    global_max: Arc<AtomicUsize>,
    completion: Arc<TestCompletionOrder>,
    thread_ids: Mutex<Vec<thread::ThreadId>>,
}

#[cfg(test)]
impl TestWorkerProbe {
    /// Builds a probe sharing start/completion coordination with peer shards.
    fn new(
        shard: ShardId,
        start: Arc<Barrier>,
        global_active: Arc<AtomicUsize>,
        global_max: Arc<AtomicUsize>,
        completion: Arc<TestCompletionOrder>,
    ) -> Self {
        Self {
            shard,
            start,
            active: AtomicUsize::new(0),
            max_active: AtomicUsize::new(0),
            global_active,
            global_max,
            completion,
            thread_ids: Mutex::new(Vec::new()),
        }
    }

    /// Marks this shard and the worker group in flight, then synchronizes starts.
    fn enter(self: &Arc<Self>) -> TestWorkerProbeGuard {
        self.thread_ids
            .lock()
            .expect("worker thread trace lock")
            .push(thread::current().id());
        let active = self.active.fetch_add(1, Ordering::SeqCst) + 1;
        self.max_active.fetch_max(active, Ordering::SeqCst);
        let global_active = self.global_active.fetch_add(1, Ordering::SeqCst) + 1;
        self.global_max.fetch_max(global_active, Ordering::SeqCst);
        self.start.wait();
        TestWorkerProbeGuard {
            probe: Arc::clone(self),
        }
    }

    /// Forces and records the worker's completion order.
    fn after_run(&self) {
        self.completion.finish(self.shard);
    }

    /// Returns the maximum simultaneous entries observed for this shard.
    fn max_active(&self) -> usize {
        self.max_active.load(Ordering::SeqCst)
    }

    /// Returns the current number of entries, which must be zero after joining.
    fn active(&self) -> usize {
        self.active.load(Ordering::SeqCst)
    }

    /// Returns whether `expected_runs` all used the same persistent thread.
    fn ran_on_one_persistent_thread(&self, expected_runs: usize) -> bool {
        let ids = self.thread_ids.lock().expect("worker thread trace lock");
        ids.len() == expected_runs
            && ids
                .first()
                .is_some_and(|first| ids.iter().all(|id| id == first))
    }
}

/// Clears a test probe's in-flight counters even if shard execution panics.
#[cfg(test)]
struct TestWorkerProbeGuard {
    probe: Arc<TestWorkerProbe>,
}

#[cfg(test)]
impl Drop for TestWorkerProbeGuard {
    fn drop(&mut self) {
        self.probe.active.fetch_sub(1, Ordering::SeqCst);
        self.probe.global_active.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Unforgeable scheduler-owned inputs for exactly one shard tick.
///
/// The type is visible to the shard implementation, but every constructor and
/// field stays private to this module. Sibling modules therefore cannot invoke
/// the cross-shard application path with an invented mid-tick payload.
#[derive(Debug)]
pub(crate) struct ScheduledTickInputs {
    boundary_inputs: Vec<CrossShardPayload>,
    #[cfg(test)]
    staged_test_emissions: Vec<CrossShardIntent>,
}

impl ScheduledTickInputs {
    /// Creates a scheduled tick with no cross-shard boundary prefix.
    const fn empty() -> Self {
        Self {
            boundary_inputs: Vec::new(),
            #[cfg(test)]
            staged_test_emissions: Vec::new(),
        }
    }

    /// Creates a scheduled tick carrying one validated boundary prefix.
    fn with_boundary(boundary_inputs: Vec<CrossShardPayload>) -> Self {
        Self {
            boundary_inputs,
            #[cfg(test)]
            staged_test_emissions: Vec::new(),
        }
    }

    /// Transfers the validated prefix to the shard's private tick body.
    pub(crate) fn take_boundary_inputs(&mut self) -> Vec<CrossShardPayload> {
        std::mem::take(&mut self.boundary_inputs)
    }

    /// Stages test instrumentation inside the same worker-owned tick call.
    #[cfg(test)]
    fn stage_test_emissions(&mut self, emissions: Vec<CrossShardIntent>) {
        self.staged_test_emissions = emissions;
    }

    /// Transfers test emissions to the shard while its worker tick is active.
    #[cfg(test)]
    pub(crate) fn take_test_emissions(&mut self) -> Vec<CrossShardIntent> {
        std::mem::take(&mut self.staged_test_emissions)
    }
}

/// Unforgeable scheduler-owned rollback of already-drained source intents.
///
/// This is used only when the terminal tick cannot stamp a next-boundary
/// envelope. It restores ownership without exposing an out-of-tick emission API
/// to sibling modules.
#[derive(Debug)]
pub(crate) struct CrossShardOutboxRestore {
    intents: Vec<CrossShardIntent>,
}

impl CrossShardOutboxRestore {
    /// Wraps drained intents for an atomic scheduler rollback.
    fn new(intents: Vec<CrossShardIntent>) -> Self {
        Self { intents }
    }

    /// Returns the number of intents to restore.
    pub(crate) const fn len(&self) -> usize {
        self.intents.len()
    }

    /// Transfers the drained intents back to their source shard.
    pub(crate) fn into_intents(self) -> Vec<CrossShardIntent> {
        self.intents
    }
}

/// One scheduler-owned shard and its eligibility lifecycle.
#[derive(Debug)]
struct ScheduledShard {
    lifecycle: ShardLifecycle,
    shard: SimShard,
    #[cfg(test)]
    probe: Option<Arc<TestWorkerProbe>>,
    #[cfg(test)]
    panic_on_run: bool,
    #[cfg(test)]
    next_tick_cross_shard: Vec<CrossShardIntent>,
}

impl ScheduledShard {
    /// Couples a newly registered shard to its canonical identity.
    const fn new(shard_id: ShardId, shard: SimShard) -> Self {
        Self {
            lifecycle: ShardLifecycle::new(shard_id),
            shard,
            #[cfg(test)]
            probe: None,
            #[cfg(test)]
            panic_on_run: false,
            #[cfg(test)]
            next_tick_cross_shard: Vec::new(),
        }
    }

    /// Returns whether this shard owns no admitted work for a future tick.
    fn is_tick_quiescent(&self) -> bool {
        self.shard.is_tick_quiescent() && {
            #[cfg(test)]
            {
                self.next_tick_cross_shard.is_empty()
            }
            #[cfg(not(test))]
            {
                true
            }
        }
    }

    /// Runs the shard once while activating the test-only overlap probe.
    fn run_tick(&mut self, tick_inputs: ScheduledTickInputs) -> ScheduledShardTick {
        #[cfg(test)]
        assert!(!self.panic_on_run, "injected shard worker panic");

        #[cfg(test)]
        let mut tick_inputs = tick_inputs;

        #[cfg(test)]
        let probe = self.probe.clone();
        #[cfg(test)]
        let _guard = probe.as_ref().map(TestWorkerProbe::enter);

        #[cfg(test)]
        tick_inputs.stage_test_emissions(std::mem::take(&mut self.next_tick_cross_shard));
        let (outputs, cross_shard) = self.shard.run_scheduled_tick(tick_inputs);

        #[cfg(test)]
        if let Some(probe) = probe {
            probe.after_run();
        }
        ScheduledShardTick {
            outputs,
            cross_shard,
        }
    }
}

/// Values one shard returns after its owned tick execution.
#[derive(Debug)]
struct ScheduledShardTick {
    outputs: Vec<GameOutput>,
    cross_shard: Vec<CrossShardIntent>,
}

/// One owned shard assigned to a persistent worker for a single tick.
#[derive(Debug)]
struct ShardWork {
    visit: WorkerVisit,
    scheduled: ScheduledShard,
    tick_inputs: ScheduledTickInputs,
}

impl ShardWork {
    /// Executes the one visit carried by this work item.
    fn run(mut self) -> CompletedShardWork {
        let tick = self.scheduled.run_tick(self.tick_inputs);
        CompletedShardWork {
            visit: self.visit,
            scheduled: self.scheduled,
            outputs: tick.outputs,
            cross_shard: tick.cross_shard,
        }
    }
}

/// A worker's completed shard plus the ownership returned to the scheduler.
#[derive(Debug)]
struct CompletedShardWork {
    visit: WorkerVisit,
    scheduled: ScheduledShard,
    outputs: Vec<GameOutput>,
    cross_shard: Vec<CrossShardIntent>,
}

/// Outputs from one shard, kept grouped under its canonical identity.
#[derive(Debug, Clone, PartialEq)]
struct ShardTickOutput {
    shard: ShardId,
    outputs: Vec<GameOutput>,
}

/// One move-owned, storage-trait-shaped overlay batch from one shard.
///
/// Records remain grouped under the container that exclusively owned their
/// chunks. The scheduler is the only constructor and never places more than
/// `capacity` records in one batch; an overflow tail remains persist-dirty for a
/// later tick.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ShardPersistBatch {
    shard: ShardId,
    capacity: usize,
    has_deferred: bool,
    records: Vec<(ChunkKey, ChunkOverlayRecord)>,
}

impl ShardPersistBatch {
    /// Returns the logical shard container that produced this batch.
    pub(crate) const fn shard(&self) -> ShardId {
        self.shard
    }

    /// Returns the fixed record ceiling applied to this batch.
    pub(crate) const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns whether this shard retained a persist-dirty overflow tail.
    pub(crate) const fn has_deferred(&self) -> bool {
        self.has_deferred
    }

    /// Borrows storage-ready chunk overlay records in canonical chunk order.
    pub(crate) fn records(&self) -> &[(ChunkKey, ChunkOverlayRecord)] {
        &self.records
    }

    /// Transfers the storage-ready records without cloning.
    pub(crate) fn into_records(self) -> Vec<(ChunkKey, ChunkOverlayRecord)> {
        self.records
    }
}

/// Cross-shard payload prefixes grouped by canonical destination.
type CrossShardBoundaryInputs = BTreeMap<ShardId, Vec<CrossShardPayload>>;

/// The result of one scheduler tick.
#[derive(Debug, PartialEq)]
pub(crate) struct SchedulerTickOutcome {
    tick: Tick,
    visits: Vec<WorkerVisit>,
    shards: Vec<ShardTickOutput>,
    cross_shard_rejections: Vec<CrossShardRejection>,
    persist_batches: Vec<ShardPersistBatch>,
}

impl SchedulerTickOutcome {
    /// Returns the globally completed tick.
    pub(crate) const fn tick(&self) -> Tick {
        self.tick
    }

    /// Iterates logical shard ids in canonical dispatch/publication order.
    pub(crate) fn visit_order(&self) -> impl Iterator<Item = ShardId> + '_ {
        self.visits.iter().map(|visit| visit.shard)
    }

    /// Iterates each shard's outputs in canonical shard order.
    pub(crate) fn shard_outputs(&self) -> impl Iterator<Item = (ShardId, &[GameOutput])> + '_ {
        self.shards
            .iter()
            .map(|shard| (shard.shard, shard.outputs.as_slice()))
    }

    /// Returns cross-shard messages rejected after canonical bounded admission.
    ///
    /// Every rejection retains the complete owned envelope so the future
    /// producing system can retry or apply its explicit failure policy without
    /// silent loss.
    pub(crate) fn cross_shard_rejections(&self) -> &[CrossShardRejection] {
        &self.cross_shard_rejections
    }

    /// Transfers every rejected envelope without discarding other move-owned
    /// tick results.
    pub(crate) fn take_cross_shard_rejections(&mut self) -> Vec<CrossShardRejection> {
        std::mem::take(&mut self.cross_shard_rejections)
    }

    /// Borrows nonempty per-shard persist batches in canonical shard order.
    pub(crate) fn persist_batches(&self) -> &[ShardPersistBatch] {
        &self.persist_batches
    }

    /// Transfers every persist batch while retaining this outcome's other
    /// outputs and rejections.
    pub(crate) fn take_persist_batches(&mut self) -> Vec<ShardPersistBatch> {
        std::mem::take(&mut self.persist_batches)
    }
}

/// Internal worker result before cross-shard envelopes are validated and
/// admitted to the next-boundary queue.
#[derive(Debug)]
struct SchedulerExecution {
    outcome: SchedulerTickOutcome,
    cross_shard: Vec<(ShardId, Vec<CrossShardIntent>)>,
}

/// Returns whether a lifecycle still needs tick execution.
///
/// Draining shards reject new admissions but keep ticking work that was already
/// accepted before the transition.
const fn is_runnable(state: ShardLifecycleState) -> bool {
    matches!(
        state,
        ShardLifecycleState::Active | ShardLifecycleState::Draining
    )
}

/// One bounded command accepted by a persistent shard worker.
#[derive(Debug)]
enum ShardWorkerCommand {
    /// Run the owned shard batch for `tick`.
    Run {
        /// Global tick shared by every worker in this dispatch.
        tick: Tick,
        /// Shards owned exclusively by this worker until it returns them.
        work: Vec<ShardWork>,
    },
    /// Exit after all earlier capacity-one commands have completed.
    Shutdown,
}

/// A completed worker batch returned through its bounded result channel.
#[derive(Debug)]
struct ShardWorkerResult {
    tick: Tick,
    work: Vec<CompletedShardWork>,
}

/// Capacity one is a rendezvous with one tick in flight per worker.
///
/// The scheduler waits for every result before it can dispatch again, so a
/// larger queue would only permit accidental tick overlap.
const WORKER_CHANNEL_CAPACITY: usize = 1;

/// Defensive ceiling for the non-default internal worker pool.
///
/// The shadow seam is a correctness path, not an app configuration surface.
/// Capping it at 64 prevents a future caller from turning an unchecked integer
/// into an unbounded OS-thread allocation.
const MAX_SHARD_WORKERS: usize = 64;

/// Default number of accepted cross-shard envelopes awaiting their exact next
/// tick boundary.
///
/// This matches the ordinary shard inbox default. Full admission is
/// nonblocking reject-newest after canonical sorting; the rejected owned
/// envelope is returned in the completed tick outcome.
const DEFAULT_CROSS_SHARD_QUEUE_CAPACITY: usize = 1024;

/// Builds the fixed nonzero default without a production `unwrap`.
const fn nonzero_cross_shard_capacity() -> NonZeroUsize {
    match NonZeroUsize::new(DEFAULT_CROSS_SHARD_QUEUE_CAPACITY) {
        Some(capacity) => capacity,
        None => NonZeroUsize::MIN,
    }
}

/// Default per-shard overlay records transferred in one completed tick.
///
/// This equals [`MAX_SAVE_BATCH`], so every produced vector can be moved
/// directly into [`ferrumc_storage::WorldStore::save_chunk_overlays`] without
/// exceeding the storage trait's atomic batch ceiling. Overflow remains dirty
/// and is emitted by a later fair canonical-order batch.
const DEFAULT_PERSIST_BATCH_CAPACITY: usize = MAX_SAVE_BATCH;

/// Builds the storage-aligned nonzero default without a production `unwrap`.
const fn nonzero_persist_batch_capacity() -> NonZeroUsize {
    match NonZeroUsize::new(DEFAULT_PERSIST_BATCH_CAPACITY) {
        Some(capacity) => capacity,
        None => NonZeroUsize::MIN,
    }
}

/// Caps an internal requested capacity at the downstream trait ceiling.
fn bounded_persist_batch_capacity(capacity: NonZeroUsize) -> NonZeroUsize {
    match NonZeroUsize::new(capacity.get().min(DEFAULT_PERSIST_BATCH_CAPACITY)) {
        Some(capacity) => capacity,
        None => NonZeroUsize::MIN,
    }
}

/// One persistent worker thread and its capacity-one command/result endpoints.
#[derive(Debug)]
struct ShardWorkerHandle {
    slot: WorkerSlot,
    command_tx: SyncSender<ShardWorkerCommand>,
    result_rx: Receiver<ShardWorkerResult>,
    join: Option<JoinHandle<()>>,
}

/// Persistent worker pool used only by the non-default shadow scheduler.
#[derive(Debug)]
struct ShardWorkerPool {
    workers: Vec<ShardWorkerHandle>,
}

impl ShardWorkerPool {
    /// Spawns the fixed worker set before any shard can be registered.
    fn new(worker_slots: NonZeroUsize) -> SimResult<Self> {
        if worker_slots.get() > MAX_SHARD_WORKERS {
            return Err(SimError::TooManyShardWorkers {
                requested: worker_slots.get(),
                maximum: MAX_SHARD_WORKERS,
            });
        }
        let mut pool = Self {
            workers: Vec::with_capacity(worker_slots.get()),
        };
        for index in 0..worker_slots.get() {
            let slot = WorkerSlot(index);
            let (command_tx, command_rx) = mpsc::sync_channel(WORKER_CHANNEL_CAPACITY);
            let (result_tx, result_rx) = mpsc::sync_channel(WORKER_CHANNEL_CAPACITY);
            let name = format!("ferrumc-sim-worker-{index}");
            let join = match thread::Builder::new()
                .name(name)
                .spawn(move || shard_worker_loop(&command_rx, &result_tx))
            {
                Ok(join) => join,
                Err(source) => {
                    pool.shutdown();
                    return Err(SimError::ShardWorkerSpawnFailed {
                        slot: index,
                        kind: source.kind(),
                    });
                }
            };
            pool.workers.push(ShardWorkerHandle {
                slot,
                command_tx,
                result_rx,
                join: Some(join),
            });
        }
        Ok(pool)
    }

    /// Removes every planned shard from the scheduler and groups ownership by
    /// worker slot.
    fn take_work(
        &self,
        shards: &mut BTreeMap<ShardId, ScheduledShard>,
        plan: &ExecutionPlan,
    ) -> SimResult<Vec<Vec<ShardWork>>> {
        let planned_ids: Vec<_> = plan.visits().iter().map(|visit| visit.shard).collect();
        let runnable_ids: Vec<_> = shards
            .iter()
            .filter(|(_, scheduled)| is_runnable(scheduled.lifecycle.state()))
            .map(|(shard, _)| *shard)
            .collect();
        if planned_ids != runnable_ids {
            let shard = planned_ids
                .iter()
                .zip(&runnable_ids)
                .find_map(|(planned, actual)| (planned != actual).then_some(*planned))
                .or_else(|| planned_ids.last().copied())
                .or_else(|| runnable_ids.last().copied());
            if let Some(shard) = shard {
                return Err(SimError::InvalidSchedulerPlan { shard });
            }
        }

        let mut buckets: Vec<Vec<ShardWork>> =
            (0..self.workers.len()).map(|_| Vec::new()).collect();
        for visit in plan.visits().iter().copied() {
            let Some(slot) = visit.slot else {
                Self::restore_work(shards, buckets);
                return Err(SimError::InvalidSchedulerPlan { shard: visit.shard });
            };
            let Some(scheduled) = shards.remove(&visit.shard) else {
                Self::restore_work(shards, buckets);
                return Err(SimError::InvalidSchedulerPlan { shard: visit.shard });
            };
            let Some(bucket) = buckets.get_mut(slot.index()) else {
                shards.insert(visit.shard, scheduled);
                Self::restore_work(shards, buckets);
                return Err(SimError::InvalidSchedulerPlan { shard: visit.shard });
            };
            bucket.push(ShardWork {
                visit,
                scheduled,
                tick_inputs: ScheduledTickInputs::empty(),
            });
        }
        Ok(buckets)
    }

    /// Restores an unexecuted batch to scheduler ownership.
    fn restore_work(shards: &mut BTreeMap<ShardId, ScheduledShard>, buckets: Vec<Vec<ShardWork>>) {
        for work in buckets.into_iter().flatten() {
            shards.insert(work.visit.shard, work.scheduled);
        }
    }

    /// Attaches each prepared next-boundary payload prefix to its exclusively
    /// owned destination work item.
    fn attach_boundary_inputs(
        buckets: &mut [Vec<ShardWork>],
        mut boundary_inputs: CrossShardBoundaryInputs,
    ) -> SimResult<()> {
        for work in buckets.iter_mut().flatten() {
            if let Some(inputs) = boundary_inputs.remove(&work.visit.shard) {
                work.tick_inputs = ScheduledTickInputs::with_boundary(inputs);
            }
        }
        match boundary_inputs.into_keys().next() {
            Some(shard) => Err(SimError::InvalidSchedulerPlan { shard }),
            None => Ok(()),
        }
    }

    /// Restores completed shards and appends their grouped outputs.
    fn restore_completed(
        shards: &mut BTreeMap<ShardId, ScheduledShard>,
        completed: Vec<CompletedShardWork>,
        outputs: &mut Vec<ShardTickOutput>,
        cross_shard: &mut Vec<(ShardId, Vec<CrossShardIntent>)>,
    ) {
        for work in completed {
            outputs.push(ShardTickOutput {
                shard: work.visit.shard,
                outputs: work.outputs,
            });
            if !work.cross_shard.is_empty() {
                cross_shard.push((work.visit.shard, work.cross_shard));
            }
            shards.insert(work.visit.shard, work.scheduled);
        }
    }

    /// Classifies and joins a worker whose channel disconnected.
    fn disconnected_error(&mut self, index: usize, tick: Tick) -> SimError {
        let worker = &mut self.workers[index];
        let panicked = worker.join.take().is_some_and(|join| join.join().is_err());
        if panicked {
            SimError::ShardWorkerPanicked {
                slot: worker.slot.index(),
                tick,
            }
        } else {
            SimError::ShardWorkerStopped {
                slot: worker.slot.index(),
                tick,
            }
        }
    }

    /// Dispatches one owned batch per nonempty worker slot and restores every
    /// returned shard before completing.
    fn execute(
        &mut self,
        coordinator: &mut TickCoordinator,
        shards: &mut BTreeMap<ShardId, ScheduledShard>,
        plan: ExecutionPlan,
        boundary_inputs: CrossShardBoundaryInputs,
    ) -> SimResult<SchedulerExecution> {
        if plan.visits().is_empty() {
            if let Some(shard) = boundary_inputs.into_keys().next() {
                return Err(SimError::InvalidSchedulerPlan { shard });
            }
            let tick = coordinator.advance()?;
            return Ok(SchedulerExecution {
                outcome: SchedulerTickOutcome {
                    tick,
                    visits: Vec::new(),
                    shards: Vec::new(),
                    cross_shard_rejections: Vec::new(),
                    persist_batches: Vec::new(),
                },
                cross_shard: Vec::new(),
            });
        }

        let mut buckets = self.take_work(shards, &plan)?;
        if let Err(error) = Self::attach_boundary_inputs(&mut buckets, boundary_inputs) {
            Self::restore_work(shards, buckets);
            return Err(error);
        }
        let tick = match coordinator.advance() {
            Ok(tick) => tick,
            Err(error) => {
                Self::restore_work(shards, buckets);
                return Err(error);
            }
        };
        let mut sent = Vec::new();
        let mut failure = None;

        for (index, work) in buckets.into_iter().enumerate() {
            if work.is_empty() {
                continue;
            }
            if failure.is_some() {
                Self::restore_work(shards, vec![work]);
                continue;
            }
            let command = ShardWorkerCommand::Run { tick, work };
            if let Err(mpsc::SendError(command)) = self.workers[index].command_tx.send(command) {
                if let ShardWorkerCommand::Run { work, .. } = command {
                    Self::restore_work(shards, vec![work]);
                }
                failure = Some(self.disconnected_error(index, tick));
            } else {
                sent.push(index);
            }
        }

        let mut outputs = Vec::with_capacity(plan.visits().len());
        let mut cross_shard = Vec::with_capacity(plan.visits().len());
        for index in sent {
            if let Ok(result) = self.workers[index].result_rx.recv() {
                if result.tick != tick && failure.is_none() {
                    failure = Some(SimError::ShardWorkerWrongTick {
                        slot: self.workers[index].slot.index(),
                        expected: tick,
                        actual: result.tick,
                    });
                }
                Self::restore_completed(shards, result.work, &mut outputs, &mut cross_shard);
            } else {
                let error = self.disconnected_error(index, tick);
                if failure.is_none() {
                    failure = Some(error);
                }
            }
        }

        if let Some(error) = failure {
            return Err(error);
        }
        outputs.sort_unstable_by_key(|output| output.shard);
        Ok(SchedulerExecution {
            outcome: SchedulerTickOutcome {
                tick,
                visits: plan.visits,
                shards: outputs,
                cross_shard_rejections: Vec::new(),
                persist_batches: Vec::new(),
            },
            cross_shard,
        })
    }

    /// Requests clean shutdown and joins every worker that is still alive.
    fn shutdown(&mut self) {
        for worker in &self.workers {
            if worker.join.is_some() {
                // A disconnected receiver already terminated; Drop has no
                // caller to report that expected fail-stop state to.
                let _closed = worker.command_tx.send(ShardWorkerCommand::Shutdown);
            }
        }
        for worker in &mut self.workers {
            if let Some(join) = worker.join.take() {
                // Any panic was surfaced when the live scheduler observed its
                // disconnected result channel. During Drop there is no safe
                // recovery action beyond joining the thread.
                let _panicked = join.join().is_err();
            }
        }
    }
}

impl Drop for ShardWorkerPool {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Runs one persistent worker until its bounded command endpoint closes.
fn shard_worker_loop(
    command_rx: &Receiver<ShardWorkerCommand>,
    result_tx: &SyncSender<ShardWorkerResult>,
) {
    loop {
        let Ok(command) = command_rx.recv() else {
            return;
        };
        match command {
            ShardWorkerCommand::Run { tick, work } => {
                let work = work.into_iter().map(ShardWork::run).collect();
                if result_tx.send(ShardWorkerResult { tick, work }).is_err() {
                    return;
                }
            }
            ShardWorkerCommand::Shutdown => return,
        }
    }
}

/// Owns registered shards and drives them in a deterministic order.
///
/// This type is intentionally crate-private and has no application wiring. Its
/// immutable mode makes the authoritative-to-shadow transition impossible to
/// toggle accidentally at runtime.
#[derive(Debug)]
pub(crate) struct ShardScheduler {
    coordinator: TickCoordinator,
    mode: SchedulerMode,
    worker_pool: Option<ShardWorkerPool>,
    shards: BTreeMap<ShardId, ScheduledShard>,
    cross_shard: CrossShardQueue,
    persist_batch_capacity: NonZeroUsize,
    /// A worker failure after tick advance is fail-stop: retry could double-run
    /// peers whose state was already returned.
    poisoned_at: Option<Tick>,
}

impl ShardScheduler {
    /// Creates the default scheduler around exactly one active authoritative
    /// shard.
    pub(crate) fn authoritative(coordinator: TickCoordinator, shard: SimShard) -> SimResult<Self> {
        let mut scheduler = Self {
            coordinator,
            mode: SchedulerMode::default(),
            worker_pool: None,
            shards: BTreeMap::new(),
            cross_shard: CrossShardQueue::new(nonzero_cross_shard_capacity()),
            persist_batch_capacity: nonzero_persist_batch_capacity(),
            poisoned_at: None,
        };
        let shard_id = scheduler.register(shard)?;
        scheduler.activate(shard_id)?;
        Ok(scheduler)
    }

    /// Creates an empty non-authoritative shadow scheduler.
    pub(crate) fn with_shadow_workers(
        coordinator: TickCoordinator,
        worker_slots: NonZeroUsize,
    ) -> SimResult<Self> {
        Self::with_shadow_workers_and_cross_shard_capacity(
            coordinator,
            worker_slots,
            nonzero_cross_shard_capacity(),
        )
    }

    /// Creates an empty shadow scheduler with an explicit bounded cross-shard
    /// queue capacity.
    ///
    /// This remains crate-private rollout plumbing. A tiny capacity is useful
    /// for deterministic backpressure tests; production-neutral shadow
    /// construction uses [`DEFAULT_CROSS_SHARD_QUEUE_CAPACITY`].
    pub(crate) fn with_shadow_workers_and_cross_shard_capacity(
        coordinator: TickCoordinator,
        worker_slots: NonZeroUsize,
        cross_shard_capacity: NonZeroUsize,
    ) -> SimResult<Self> {
        Self::with_shadow_workers_and_capacities(
            coordinator,
            worker_slots,
            cross_shard_capacity,
            nonzero_persist_batch_capacity(),
        )
    }

    /// Creates an empty shadow scheduler with explicit internal queue ceilings.
    ///
    /// Tiny capacities make both reject-newest cross-shard admission and
    /// deferred persist-batch behavior deterministic in tests. Production
    /// shadow construction uses the documented defaults.
    pub(crate) fn with_shadow_workers_and_capacities(
        coordinator: TickCoordinator,
        worker_slots: NonZeroUsize,
        cross_shard_capacity: NonZeroUsize,
        persist_batch_capacity: NonZeroUsize,
    ) -> SimResult<Self> {
        Ok(Self {
            coordinator,
            mode: SchedulerMode::shadow(worker_slots),
            worker_pool: Some(ShardWorkerPool::new(worker_slots)?),
            shards: BTreeMap::new(),
            cross_shard: CrossShardQueue::new(cross_shard_capacity),
            persist_batch_capacity: bounded_persist_batch_capacity(persist_batch_capacity),
            poisoned_at: None,
        })
    }

    /// Rejects every mutating operation after a partially executed worker tick.
    fn ensure_healthy(&self) -> SimResult<()> {
        match self.poisoned_at {
            Some(tick) => Err(SimError::ShardSchedulerPoisoned { tick }),
            None => Ok(()),
        }
    }

    /// Registers a shard in the created state, deriving its id from the shard's
    /// own typed world, dimension, and position.
    pub(crate) fn register(&mut self, shard: SimShard) -> SimResult<ShardId> {
        self.ensure_healthy()?;
        let shard_id = ShardId::try_new(
            shard.loaded_chunks().world(),
            shard.loaded_chunks().dimension(),
            shard.shard_pos(),
        )?;
        if self.shards.contains_key(&shard_id) {
            return Err(SimError::OverlappingShardOwnership {
                region: shard_id.region(),
                shard: shard_id,
            });
        }
        self.shards
            .insert(shard_id, ScheduledShard::new(shard_id, shard));
        Ok(shard_id)
    }

    /// Rejects a chunk storage key resident in more than one shard container.
    ///
    /// Every resident key is checked, not only today's dirty keys, so an alias
    /// cannot become two competing persist records on a later tick. This
    /// read-only preflight runs before coordinator advance and therefore leaves
    /// ticks, inboxes, and dirty masks unchanged on conflict.
    fn validate_unique_persist_residency(&self) -> SimResult<()> {
        if self.shards.len() < 2 {
            return Ok(());
        }
        let mut owners = BTreeMap::new();
        for (&shard_id, scheduled) in &self.shards {
            let chunks = scheduled.shard.loaded_chunks();
            for position in chunks.loaded_positions() {
                let key = ChunkKey::new(chunks.world(), chunks.dimension(), position);
                if let Some(first) = owners.get(&key).copied() {
                    return Err(SimError::PersistChunkAliased {
                        key,
                        first,
                        second: shard_id,
                    });
                }
                owners.insert(key, shard_id);
            }
        }
        Ok(())
    }

    /// Transfers one bounded canonical overlay prefix from every dirty shard.
    ///
    /// This is deliberately the final, infallible scheduler phase. Failures
    /// after workers have returned their shards but before this capture leave
    /// every persist-dirty mask in place. Worker ownership failures remain
    /// independently fail-stop and make no recovery claim.
    fn take_persist_batches(&mut self, tick: Tick) -> Vec<ShardPersistBatch> {
        let capacity = self.persist_batch_capacity;
        let mut batches = Vec::new();
        for (&shard_id, scheduled) in &mut self.shards {
            let records = scheduled
                .shard
                .loaded_chunks_mut()
                .take_persist_dirty_bounded(tick.get(), capacity);
            if records.is_empty() {
                continue;
            }
            batches.push(ShardPersistBatch {
                shard: shard_id,
                capacity: capacity.get(),
                has_deferred: scheduled.shard.loaded_chunks().has_persist_dirty(),
                records,
            });
        }
        batches
    }

    /// Makes a created shard eligible for tick planning.
    pub(crate) fn activate(&mut self, shard: ShardId) -> SimResult<()> {
        self.transition(shard, ShardLifecycleState::Active)
    }

    /// Applies one validated lifecycle transition.
    pub(crate) fn transition(
        &mut self,
        shard: ShardId,
        next: ShardLifecycleState,
    ) -> SimResult<()> {
        self.ensure_healthy()?;
        let current = self
            .shards
            .get(&shard)
            .ok_or(SimError::UnknownScheduledShard { shard })?
            .lifecycle
            .state();
        if current == ShardLifecycleState::Created
            && next == ShardLifecycleState::Active
            && matches!(self.mode, SchedulerMode::AuthoritativeInline)
        {
            let scheduled_count = self
                .shards
                .values()
                .filter(|scheduled| is_runnable(scheduled.lifecycle.state()))
                .count();
            if scheduled_count != 0 {
                return Err(SimError::MultipleScheduledShardsDisabled {
                    scheduled: scheduled_count + 1,
                });
            }
        }
        if current == ShardLifecycleState::Draining
            && next == ShardLifecycleState::Stopped
            && (!self
                .shards
                .get(&shard)
                .ok_or(SimError::UnknownScheduledShard { shard })?
                .is_tick_quiescent()
                || self.cross_shard.has_destination(shard))
        {
            return Err(SimError::ShardDrainIncomplete { shard });
        }

        self.shards
            .get_mut(&shard)
            .ok_or(SimError::UnknownScheduledShard { shard })?
            .lifecycle
            .transition_to(next)
    }

    /// Enqueues a value into an active shard's existing bounded inbox.
    ///
    /// Created shards are not eligible yet; draining and stopped shards reject
    /// new work so admitted inputs cannot become permanently stranded.
    pub(crate) fn enqueue(&mut self, shard: ShardId, input: GameInput) -> SimResult<()> {
        self.ensure_healthy()?;
        let scheduled = self
            .shards
            .get_mut(&shard)
            .ok_or(SimError::UnknownScheduledShard { shard })?;
        let state = scheduled.lifecycle.state();
        if state != ShardLifecycleState::Active {
            return Err(SimError::ShardInputNotAccepted { shard, state });
        }
        scheduled.shard.enqueue(input)
    }

    /// Validates a non-mutating queue snapshot and groups its payloads by
    /// destination for attachment to owned worker batches.
    fn prepare_cross_shard_boundary(
        &self,
        prepared: &PreparedBoundary,
    ) -> SimResult<CrossShardBoundaryInputs> {
        let mut boundary = BTreeMap::new();
        for envelope in prepared.envelopes() {
            let destination = envelope.destination();
            let scheduled = self
                .shards
                .get(&destination)
                .ok_or(SimError::UnknownScheduledShard { shard: destination })?;
            let state = scheduled.lifecycle.state();
            if !is_runnable(state) {
                return Err(SimError::ShardInputNotAccepted {
                    shard: destination,
                    state,
                });
            }
            boundary
                .entry(destination)
                .or_insert_with(Vec::new)
                .push(envelope.payload().clone());
        }
        Ok(boundary)
    }

    /// Stamps worker-produced intents, rejects invalid destinations without
    /// consuming queue capacity, then performs canonical bounded admission.
    fn admit_cross_shard_emissions(
        &mut self,
        completed_tick: Tick,
        mut emissions: Vec<(ShardId, Vec<CrossShardIntent>)>,
    ) -> SimResult<Vec<CrossShardRejection>> {
        emissions.sort_unstable_by_key(|(source, _)| *source);
        if completed_tick.next().is_none() {
            if let Some(first_source) = emissions.first().map(|(source, _)| *source) {
                for (source, intents) in emissions {
                    let scheduled = self
                        .shards
                        .get_mut(&source)
                        .ok_or(SimError::UnknownScheduledShard { shard: source })?;
                    let restore = CrossShardOutboxRestore::new(intents);
                    if scheduled.shard.restore_cross_shard_outbox(restore).is_err() {
                        return Err(SimError::CrossShardOutboxFull {
                            shard: source,
                            capacity: SimShard::cross_shard_outbox_capacity(),
                        });
                    }
                }
                return Err(SimError::CrossShardEnvelopeTickOverflow {
                    shard: first_source,
                    tick: completed_tick,
                });
            }
        }

        let mut candidates = Vec::new();
        for (source, intents) in emissions {
            for (source_sequence, intent) in intents.into_iter().enumerate() {
                let envelope = match CrossShardEnvelope::from_intent(
                    completed_tick,
                    source,
                    source_sequence,
                    intent,
                ) {
                    Ok(envelope) => envelope,
                    Err(intent) => {
                        let scheduled = self
                            .shards
                            .get_mut(&source)
                            .ok_or(SimError::UnknownScheduledShard { shard: source })?;
                        let restore = CrossShardOutboxRestore::new(vec![intent]);
                        if scheduled.shard.restore_cross_shard_outbox(restore).is_err() {
                            return Err(SimError::CrossShardOutboxFull {
                                shard: source,
                                capacity: SimShard::cross_shard_outbox_capacity(),
                            });
                        }
                        return Err(SimError::CrossShardEnvelopeTickOverflow {
                            shard: source,
                            tick: completed_tick,
                        });
                    }
                };
                candidates.push(envelope);
            }
        }
        candidates.sort_unstable_by_key(CrossShardEnvelope::canonical_key);

        let mut valid = Vec::with_capacity(candidates.len());
        let mut rejections = Vec::new();
        for envelope in candidates {
            if envelope.source() == envelope.destination() {
                valid.push(envelope);
                continue;
            }
            let reason = match self.shards.get(&envelope.destination()) {
                None => Some(CrossShardRejectionReason::UnknownDestination),
                Some(destination)
                    if destination.lifecycle.state() != ShardLifecycleState::Active =>
                {
                    Some(CrossShardRejectionReason::DestinationNotActive {
                        state: destination.lifecycle.state(),
                    })
                }
                Some(_) => None,
            };
            if let Some(reason) = reason {
                rejections.push(CrossShardRejection::new(reason, envelope));
            } else {
                valid.push(envelope);
            }
        }

        rejections.extend(self.cross_shard.admit_completed_tick(completed_tick, valid));
        rejections.sort_unstable_by_key(|rejection| rejection.envelope().canonical_key());
        Ok(rejections)
    }

    /// Advances once and runs only active shards according to the validated
    /// plan.
    ///
    /// The plan is built before the coordinator advances, so a disabled
    /// multi-shard configuration or duplicate dispatch cannot drift the tick
    /// counter or drain any inbox. Tick overflow likewise occurs before the
    /// first shard is touched.
    pub(crate) fn tick(&mut self) -> SimResult<SchedulerTickOutcome> {
        self.ensure_healthy()?;
        let boundary_tick = self
            .coordinator
            .current()
            .next()
            .ok_or(SimError::TickOverflow)?;
        self.validate_unique_persist_residency()?;
        let plan = ExecutionPlan::for_ids(
            self.mode,
            self.shards
                .iter()
                .filter(|(_, scheduled)| is_runnable(scheduled.lifecycle.state()))
                .map(|(shard, _)| *shard),
        )?;
        let prepared = self.cross_shard.prepare_boundary(boundary_tick);
        let boundary_inputs = self.prepare_cross_shard_boundary(&prepared)?;
        let result = match self.mode {
            SchedulerMode::AuthoritativeInline => Self::execute_inline(
                &mut self.coordinator,
                &mut self.shards,
                plan,
                boundary_inputs,
            ),
            SchedulerMode::ShadowWorkers { .. } => {
                let result = self
                    .worker_pool
                    .as_mut()
                    .ok_or(SimError::ShardWorkerPoolUnavailable)?
                    .execute(
                        &mut self.coordinator,
                        &mut self.shards,
                        plan,
                        boundary_inputs,
                    );
                result
            }
        };
        if matches!(
            &result,
            Err(SimError::ShardWorkerPanicked { .. }
                | SimError::ShardWorkerStopped { .. }
                | SimError::ShardWorkerWrongTick { .. })
        ) {
            self.poisoned_at = Some(self.coordinator.current());
        }
        let mut execution = result?;
        if execution.outcome.tick() != prepared.apply_at()
            || !self.cross_shard.commit_boundary(&prepared)
        {
            self.poisoned_at = Some(self.coordinator.current());
            return Err(SimError::CrossShardBoundaryCommitFailed {
                tick: prepared.apply_at(),
            });
        }
        let rejections = match self
            .admit_cross_shard_emissions(execution.outcome.tick(), execution.cross_shard)
        {
            Ok(rejections) => rejections,
            Err(error) => {
                self.poisoned_at = Some(self.coordinator.current());
                return Err(error);
            }
        };
        execution.outcome.cross_shard_rejections = rejections;
        execution.outcome.persist_batches = self.take_persist_batches(execution.outcome.tick());
        Ok(execution.outcome)
    }

    /// Executes the exact legacy single-shard path without a worker dispatch.
    fn execute_inline(
        coordinator: &mut TickCoordinator,
        shards: &mut BTreeMap<ShardId, ScheduledShard>,
        plan: ExecutionPlan,
        mut boundary_inputs: CrossShardBoundaryInputs,
    ) -> SimResult<SchedulerExecution> {
        let mut runnable = shards
            .iter_mut()
            .filter(|(_, scheduled)| is_runnable(scheduled.lifecycle.state()));
        let scheduled = match plan.visits().first().copied() {
            Some(visit) => {
                if visit.slot.is_some() {
                    return Err(SimError::InvalidSchedulerPlan { shard: visit.shard });
                }
                let Some((actual_id, scheduled)) = runnable.next() else {
                    return Err(SimError::InvalidSchedulerPlan { shard: visit.shard });
                };
                if *actual_id != visit.shard {
                    return Err(SimError::InvalidSchedulerPlan { shard: visit.shard });
                }
                Some((visit.shard, scheduled))
            }
            None => None,
        };
        if let Some((shard, _)) = runnable.next() {
            return Err(SimError::InvalidSchedulerPlan { shard: *shard });
        }
        let scheduled_tick_inputs = scheduled
            .as_ref()
            .and_then(|(shard, _)| boundary_inputs.remove(shard))
            .map_or_else(
                ScheduledTickInputs::empty,
                ScheduledTickInputs::with_boundary,
            );
        if let Some(shard) = boundary_inputs.into_keys().next() {
            return Err(SimError::InvalidSchedulerPlan { shard });
        }

        let tick = coordinator.advance()?;
        let mut outputs = Vec::with_capacity(usize::from(scheduled.is_some()));
        let mut cross_shard = Vec::with_capacity(usize::from(scheduled.is_some()));
        if let Some((shard, scheduled)) = scheduled {
            let completed = scheduled.run_tick(scheduled_tick_inputs);
            outputs.push(ShardTickOutput {
                shard,
                outputs: completed.outputs,
            });
            if !completed.cross_shard.is_empty() {
                cross_shard.push((shard, completed.cross_shard));
            }
        }
        Ok(SchedulerExecution {
            outcome: SchedulerTickOutcome {
                tick,
                visits: plan.visits,
                shards: outputs,
                cross_shard_rejections: Vec::new(),
                persist_batches: Vec::new(),
            },
            cross_shard,
        })
    }

    /// Returns the current global tick without advancing it.
    pub(crate) const fn current_tick(&self) -> Tick {
        self.coordinator.current()
    }

    /// Returns a registered shard for inspection.
    pub(crate) fn shard(&self, shard: ShardId) -> Option<&SimShard> {
        self.shards.get(&shard).map(|scheduled| &scheduled.shard)
    }

    /// Returns a registered shard's lifecycle state.
    pub(crate) fn lifecycle(&self, shard: ShardId) -> Option<ShardLifecycleState> {
        self.shards
            .get(&shard)
            .map(|scheduled| scheduled.lifecycle.state())
    }

    /// Returns the number of accepted envelopes awaiting a boundary.
    #[cfg(test)]
    fn cross_shard_queue_len(&self) -> usize {
        self.cross_shard.len()
    }

    /// Installs an intent that the named source emits from inside its next
    /// actual worker execution.
    ///
    /// This is test instrumentation for the Packet 42 carrier until a gameplay
    /// system produces transfers. It enters the same bounded shard outbox,
    /// canonical merge, central queue, and destination boundary path as future
    /// production intents.
    #[cfg(test)]
    fn emit_cross_shard_during_next_tick(
        &mut self,
        source: ShardId,
        destination: ShardId,
        payload: CrossShardPayload,
    ) -> SimResult<()> {
        self.ensure_healthy()?;
        let scheduled = self
            .shards
            .get_mut(&source)
            .ok_or(SimError::UnknownScheduledShard { shard: source })?;
        let state = scheduled.lifecycle.state();
        if state != ShardLifecycleState::Active {
            return Err(SimError::ShardInputNotAccepted {
                shard: source,
                state,
            });
        }
        let capacity = SimShard::cross_shard_outbox_capacity();
        if scheduled.next_tick_cross_shard.len() >= capacity {
            return Err(SimError::CrossShardOutboxFull {
                shard: source,
                capacity,
            });
        }
        scheduled
            .next_tick_cross_shard
            .push(CrossShardIntent::new(destination, payload));
        Ok(())
    }

    /// Installs deterministic overlap/completion instrumentation for a test.
    #[cfg(test)]
    fn set_probe(&mut self, shard: ShardId, probe: Arc<TestWorkerProbe>) -> SimResult<()> {
        self.ensure_healthy()?;
        self.shards
            .get_mut(&shard)
            .ok_or(SimError::UnknownScheduledShard { shard })?
            .probe = Some(probe);
        Ok(())
    }

    /// Makes one test worker panic after taking ownership of its shard.
    #[cfg(test)]
    fn set_panic_on_run(&mut self, shard: ShardId) -> SimResult<()> {
        self.ensure_healthy()?;
        self.shards
            .get_mut(&shard)
            .ok_or(SimError::UnknownScheduledShard { shard })?
            .panic_on_run = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};

    use ferrumc_core::{DimensionId, GameMode, PlayerId, Tick, WorldId};
    use ferrumc_items::{ComponentPatch, ComponentValue, ItemId, ItemStack, OpaqueComponent};
    use ferrumc_math::{BlockPos, ChunkPos, LocalBlockPos, ShardPos, Vec3};
    use ferrumc_storage::{ChunkKey, InMemoryStore, MAX_SAVE_BATCH};
    use ferrumc_world::{
        BlockEntity, BlockStateId, Chunk, FlatWorldGenerator, SECTION_COUNT, SECTION_VOLUME,
    };

    use super::{
        ExecutionPlan, SchedulerMode, ShardScheduler, TestCompletionOrder, TestWorkerProbe,
        MAX_SHARD_WORKERS,
    };
    use crate::{
        cross_shard::{CrossShardPayload, CrossShardRejectionReason},
        ChunkTicket, GameInput, GameOutput, MutationCause, ShardId, ShardLifecycleState, SignFace,
        SimError, SimShard, SpawnedEntityKind, TickCoordinator, TickRate, TicketReason,
    };

    fn logical_id(world: WorldId, dimension: DimensionId, position: ShardPos) -> ShardId {
        ShardId::try_new(world, dimension, position).expect("valid test shard position")
    }

    async fn persist_dirty_shard(position: ShardPos, chunks: &[ChunkPos]) -> SimShard {
        persist_dirty_shard_in_dimension(position, WorldId::new(0), DimensionId::new(0), chunks)
            .await
    }

    async fn persist_dirty_shard_in_dimension(
        position: ShardPos,
        world: WorldId,
        dimension: DimensionId,
        chunks: &[ChunkPos],
    ) -> SimShard {
        let store = InMemoryStore::new();
        let generator = FlatWorldGenerator::new();
        let mut shard = SimShard::in_dimension(position, world, dimension);
        for (index, chunk_pos) in chunks.iter().copied().enumerate() {
            shard
                .loaded_chunks_mut()
                .acquire(
                    &store,
                    &generator,
                    chunk_pos,
                    ChunkTicket::of(TicketReason::Forced),
                )
                .await
                .expect("load test chunk");
            let edited = chunk_pos.origin_block(70);
            let state = BlockStateId::new(u32::try_from(index + 5).expect("small test state"));
            let chunk = shard
                .loaded_chunks_mut()
                .get_mut(chunk_pos)
                .expect("resident test chunk");
            chunk
                .set_block(edited, state)
                .expect("test height is in range");
            chunk.mark_persist_dirty(edited);
        }
        shard
    }

    fn append_len(bytes: &mut Vec<u8>, len: usize) {
        let len = u64::try_from(len).expect("test value fits u64");
        bytes.extend_from_slice(&len.to_be_bytes());
    }

    fn append_player(bytes: &mut Vec<u8>, player: PlayerId) {
        bytes.extend_from_slice(player.as_uuid().as_bytes());
    }

    fn append_position(bytes: &mut Vec<u8>, position: Vec3) {
        bytes.extend_from_slice(&position.x.to_bits().to_be_bytes());
        bytes.extend_from_slice(&position.y.to_bits().to_be_bytes());
        bytes.extend_from_slice(&position.z.to_bits().to_be_bytes());
    }

    fn append_block_position(bytes: &mut Vec<u8>, position: BlockPos) {
        bytes.extend_from_slice(&position.x().to_be_bytes());
        bytes.extend_from_slice(&position.y().to_be_bytes());
        bytes.extend_from_slice(&position.z().to_be_bytes());
    }

    fn append_string(bytes: &mut Vec<u8>, value: &str) {
        append_len(bytes, value.len());
        bytes.extend_from_slice(value.as_bytes());
    }

    fn append_sign_face(bytes: &mut Vec<u8>, face: &SignFace) {
        for line in face.lines() {
            append_string(bytes, line);
        }
        append_string(bytes, face.color());
        bytes.push(u8::from(face.has_glowing_text()));
    }

    fn append_mutation_cause(bytes: &mut Vec<u8>, cause: MutationCause) {
        match cause {
            MutationCause::PlayerCreative { player } => {
                bytes.push(0);
                append_player(bytes, player);
            }
            MutationCause::Command => bytes.push(1),
            MutationCause::Plugin => bytes.push(2),
            MutationCause::Test => bytes.push(3),
        }
    }

    /// Encodes every observable output field explicitly, including float bit
    /// patterns, so parity does not depend on `Debug` formatting or Rust memory
    /// layout.
    #[allow(clippy::too_many_lines)] // one match over every GameOutput variant
    fn canonical_output_bytes(outputs: &[GameOutput]) -> Vec<u8> {
        let mut bytes = Vec::new();
        append_len(&mut bytes, outputs.len());
        for output in outputs {
            match output {
                GameOutput::PlayerSpawned { player, position } => {
                    bytes.push(0);
                    append_player(&mut bytes, *player);
                    append_position(&mut bytes, *position);
                }
                GameOutput::PlayerMoved {
                    player,
                    position,
                    yaw,
                    pitch,
                    position_changed,
                } => {
                    bytes.push(1);
                    append_player(&mut bytes, *player);
                    append_position(&mut bytes, *position);
                    bytes.extend_from_slice(&yaw.to_bits().to_be_bytes());
                    bytes.extend_from_slice(&pitch.to_bits().to_be_bytes());
                    bytes.push(u8::from(*position_changed));
                }
                GameOutput::PlayerPositionCorrected { player, position } => {
                    bytes.push(2);
                    append_player(&mut bytes, *player);
                    append_position(&mut bytes, *position);
                }
                GameOutput::PlayerDespawned { player } => {
                    bytes.push(3);
                    append_player(&mut bytes, *player);
                }
                GameOutput::BlockChanged {
                    position,
                    state,
                    sequence,
                    cause,
                } => {
                    bytes.push(4);
                    append_block_position(&mut bytes, *position);
                    bytes.extend_from_slice(&state.as_u32().to_be_bytes());
                    bytes.extend_from_slice(&sequence.to_be_bytes());
                    append_mutation_cause(&mut bytes, *cause);
                }
                GameOutput::BlockChangeRejected {
                    player,
                    position,
                    sequence,
                    requested_state,
                    authoritative_state,
                } => {
                    bytes.push(5);
                    append_player(&mut bytes, *player);
                    append_block_position(&mut bytes, *position);
                    bytes.extend_from_slice(&sequence.to_be_bytes());
                    bytes.extend_from_slice(&requested_state.as_u32().to_be_bytes());
                    bytes.extend_from_slice(&authoritative_state.as_u32().to_be_bytes());
                }
                GameOutput::SignUpdated { position, sign } => {
                    bytes.push(6);
                    append_block_position(&mut bytes, *position);
                    bytes.extend_from_slice(&sign.kind().block_entity_type().to_be_bytes());
                    bytes.push(u8::from(sign.is_waxed()));
                    append_sign_face(&mut bytes, sign.front());
                    append_sign_face(&mut bytes, sign.back());
                }
                GameOutput::OpenSignEditor { player, position } => {
                    bytes.push(7);
                    append_player(&mut bytes, *player);
                    append_block_position(&mut bytes, *position);
                }
                GameOutput::EntitySpawned {
                    entity,
                    position,
                    velocity,
                    kind,
                } => {
                    bytes.push(8);
                    bytes.extend_from_slice(&entity.get().to_be_bytes());
                    append_position(&mut bytes, *position);
                    // Velocity is a `Vec3`, encoded identically to a position.
                    append_position(&mut bytes, *velocity);
                    match kind {
                        SpawnedEntityKind::Simple => bytes.push(0),
                        SpawnedEntityKind::FallingBlock { block } => {
                            bytes.push(1);
                            bytes.extend_from_slice(&block.as_u32().to_be_bytes());
                        }
                    }
                }
                GameOutput::EntityMoved { entity, position } => {
                    bytes.push(9);
                    bytes.extend_from_slice(&entity.get().to_be_bytes());
                    append_position(&mut bytes, *position);
                }
                GameOutput::EntityDespawned { entity } => {
                    bytes.push(10);
                    bytes.extend_from_slice(&entity.get().to_be_bytes());
                }
            }
        }
        bytes
    }

    fn assert_same_observable_shard_state(left: &SimShard, right: &SimShard, player: PlayerId) {
        assert_eq!(format!("{left:#?}"), format!("{right:#?}"));
        assert_eq!(left.shard_pos(), right.shard_pos());
        assert_eq!(left.inbox_len(), right.inbox_len());
        assert_eq!(left.player_count(), right.player_count());
        assert_eq!(left.player_position(player), right.player_position(player));
        assert_eq!(
            left.player_game_mode(player),
            right.player_game_mode(player)
        );
        assert_eq!(
            left.loaded_chunks().loaded_count(),
            right.loaded_chunks().loaded_count()
        );
        assert_eq!(left.has_pending_mutations(), right.has_pending_mutations());
    }

    #[test]
    fn switch_off_tick_output_is_byte_identical_to_legacy_path() {
        let player = PlayerId::offline("scheduler-legacy");
        let mut shard = SimShard::new(ShardPos::new(0, 0));
        shard
            .enqueue(GameInput::PlayerJoin {
                player,
                position: Vec3::new(4.0, 70.0, 8.0),
            })
            .expect("test inbox has capacity");
        shard
            .enqueue(GameInput::PlayerMove {
                player,
                position: Some(Vec3::new(5.0, 70.5, 9.0)),
                yaw: Some(90.0),
                pitch: Some(-12.0),
            })
            .expect("test inbox has capacity");

        let shard_id = logical_id(WorldId::new(0), DimensionId::new(0), shard.shard_pos());
        let mut legacy_shard = shard.clone();
        let mut legacy_coordinator = TickCoordinator::new(TickRate::VANILLA);
        let legacy_tick = legacy_coordinator.advance().expect("tick advances");
        let legacy_outputs = legacy_shard.run_tick();

        assert_eq!(SchedulerMode::default(), SchedulerMode::AuthoritativeInline);
        let mut scheduler =
            ShardScheduler::authoritative(TickCoordinator::new(TickRate::VANILLA), shard)
                .expect("valid authoritative shard");
        let scheduler_tick = scheduler.tick().expect("scheduled tick");

        assert_eq!(scheduler_tick.tick, legacy_tick);
        assert_eq!(scheduler_tick.visits.len(), 1);
        assert_eq!(scheduler_tick.visits[0].shard, shard_id);
        assert_eq!(scheduler_tick.visits[0].slot, None);
        assert_eq!(scheduler_tick.shards.len(), 1);
        assert_eq!(scheduler_tick.shards[0].shard, shard_id);
        assert_eq!(scheduler_tick.shards[0].outputs, legacy_outputs);
        assert_eq!(
            canonical_output_bytes(&scheduler_tick.shards[0].outputs),
            canonical_output_bytes(&legacy_outputs)
        );
        assert_same_observable_shard_state(
            scheduler.shard(shard_id).expect("registered shard"),
            &legacy_shard,
            player,
        );
    }

    #[test]
    fn switch_on_one_shard_is_identical_to_switch_off() {
        let player = PlayerId::offline("scheduler-shadow");
        let shard = SimShard::new(ShardPos::new(-2, 3));
        let shard_id = logical_id(WorldId::new(0), DimensionId::new(0), shard.shard_pos());
        let mut inline =
            ShardScheduler::authoritative(TickCoordinator::new(TickRate::VANILLA), shard.clone())
                .expect("valid authoritative shard");
        let mut shadow = ShardScheduler::with_shadow_workers(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::new(3).expect("nonzero worker slots"),
        )
        .expect("worker pool");
        assert_eq!(shadow.register(shard).expect("register"), shard_id);
        shadow.activate(shard_id).expect("activate");

        let input_schedule = [
            vec![GameInput::PlayerJoin {
                player,
                position: Vec3::new(-17.0, 64.0, 25.0),
            }],
            vec![
                GameInput::PlayerMove {
                    player,
                    position: Some(Vec3::new(-16.5, 65.0, 26.0)),
                    yaw: Some(135.0),
                    pitch: Some(5.0),
                },
                GameInput::SetGameMode {
                    player,
                    mode: GameMode::Creative,
                },
            ],
            vec![GameInput::PlayerLeave { player }],
        ];

        for inputs in input_schedule {
            for input in inputs {
                inline
                    .enqueue(shard_id, input.clone())
                    .expect("inline inbox");
                shadow.enqueue(shard_id, input).expect("shadow inbox");
            }

            let inline_tick = inline.tick().expect("inline tick");
            let shadow_tick = shadow.tick().expect("shadow tick");
            assert_eq!(shadow_tick.tick, inline_tick.tick);
            assert_eq!(inline_tick.visits[0].slot, None);
            assert_eq!(
                shadow_tick.visits[0].slot.map(super::WorkerSlot::index),
                Some(0)
            );
            assert_eq!(
                canonical_output_bytes(&shadow_tick.shards[0].outputs),
                canonical_output_bytes(&inline_tick.shards[0].outputs)
            );
            assert_eq!(shadow_tick.shards[0].outputs, inline_tick.shards[0].outputs);
            assert_same_observable_shard_state(
                shadow.shard(shard_id).expect("shadow shard"),
                inline.shard(shard_id).expect("inline shard"),
                player,
            );
        }
    }

    fn scheduler_with_mixed_lifecycles(
        worker_slots: NonZeroUsize,
    ) -> (ShardScheduler, Vec<ShardId>) {
        let entries = [
            (
                WorldId::new(1),
                DimensionId::new(0),
                ShardPos::new(-4, 7),
                ShardLifecycleState::Active,
            ),
            (
                WorldId::new(0),
                DimensionId::new(1),
                ShardPos::new(8, -3),
                ShardLifecycleState::Created,
            ),
            (
                WorldId::new(0),
                DimensionId::new(0),
                ShardPos::new(9, 2),
                ShardLifecycleState::Active,
            ),
            (
                WorldId::new(0),
                DimensionId::new(0),
                ShardPos::new(-9, 5),
                ShardLifecycleState::Stopped,
            ),
            (
                WorldId::new(0),
                DimensionId::new(0),
                ShardPos::new(0, 0),
                ShardLifecycleState::Active,
            ),
            (
                WorldId::new(0),
                DimensionId::new(2),
                ShardPos::new(1, 1),
                ShardLifecycleState::Draining,
            ),
        ];
        let mut scheduler = ShardScheduler::with_shadow_workers(
            TickCoordinator::new(TickRate::VANILLA),
            worker_slots,
        )
        .expect("worker pool");
        let mut expected_active = Vec::new();

        for (world, dimension, position, target) in entries {
            let shard_id = scheduler
                .register(SimShard::in_dimension(position, world, dimension))
                .expect("unique valid shard");
            match target {
                ShardLifecycleState::Created => {}
                ShardLifecycleState::Active => {
                    scheduler.activate(shard_id).expect("activate");
                    expected_active.push(shard_id);
                }
                ShardLifecycleState::Draining => {
                    scheduler.activate(shard_id).expect("activate");
                    scheduler
                        .transition(shard_id, ShardLifecycleState::Draining)
                        .expect("drain");
                    expected_active.push(shard_id);
                }
                ShardLifecycleState::Stopped => {
                    scheduler.activate(shard_id).expect("activate");
                    scheduler
                        .transition(shard_id, ShardLifecycleState::Draining)
                        .expect("drain");
                    scheduler
                        .transition(shard_id, ShardLifecycleState::Stopped)
                        .expect("stop");
                }
            }
        }
        expected_active.sort_unstable();
        (scheduler, expected_active)
    }

    #[test]
    fn active_shards_visit_in_canonical_id_order() {
        for worker_slots in [1, 2, 3] {
            let worker_slots = NonZeroUsize::new(worker_slots).expect("nonzero");
            let (mut scheduler, expected) = scheduler_with_mixed_lifecycles(worker_slots);
            let outcome = scheduler.tick().expect("shadow tick");

            let visited: Vec<_> = outcome.visits.iter().map(|visit| visit.shard).collect();
            let output_order: Vec<_> = outcome.shards.iter().map(|output| output.shard).collect();
            let assigned_slots: Vec<_> = outcome
                .visits
                .iter()
                .map(|visit| visit.slot.expect("shadow dispatch").index())
                .collect();
            let expected_slots: Vec<_> = (0..expected.len())
                .map(|ordinal| ordinal % worker_slots.get())
                .collect();

            assert_eq!(visited, expected);
            assert_eq!(output_order, expected);
            assert_eq!(assigned_slots, expected_slots);
        }
    }

    #[test]
    fn same_shard_is_never_concurrently_scheduled() {
        let worker_slots = NonZeroUsize::new(2).expect("nonzero");
        let first = logical_id(WorldId::new(0), DimensionId::new(0), ShardPos::new(0, 0));
        let second = logical_id(WorldId::new(0), DimensionId::new(0), ShardPos::new(1, 0));

        let duplicate = ExecutionPlan::for_ids(SchedulerMode::shadow(worker_slots), [first, first])
            .expect_err("a shard may appear only once in a tick plan");
        assert_eq!(duplicate, SimError::DuplicateShardDispatch { shard: first });

        let plan = ExecutionPlan::for_ids(SchedulerMode::shadow(worker_slots), [second, first])
            .expect("unique shards");
        let planned: Vec<_> = plan.visits().iter().map(|visit| visit.shard).collect();
        assert_eq!(planned, vec![first, second]);

        let mut scheduler = ShardScheduler::with_shadow_workers(
            TickCoordinator::new(TickRate::VANILLA),
            worker_slots,
        )
        .expect("worker pool");
        let first_player = PlayerId::offline("worker-first");
        let second_player = PlayerId::offline("worker-second");
        for (position, player) in [
            (ShardPos::new(0, 0), first_player),
            (ShardPos::new(1, 0), second_player),
        ] {
            let id = scheduler
                .register(SimShard::new(position))
                .expect("register");
            scheduler.activate(id).expect("activate");
            scheduler
                .enqueue(
                    id,
                    GameInput::PlayerJoin {
                        player,
                        position: Vec3::new(f64::from(position.x()), 64.0, 0.0),
                    },
                )
                .expect("active shard inbox");
        }

        let start = Arc::new(Barrier::new(2));
        let global_active = Arc::new(AtomicUsize::new(0));
        let global_max = Arc::new(AtomicUsize::new(0));
        // Force the higher id to complete first so collation cannot
        // accidentally inherit worker completion order.
        let completion = Arc::new(TestCompletionOrder::new(second));
        let first_probe = Arc::new(TestWorkerProbe::new(
            first,
            Arc::clone(&start),
            Arc::clone(&global_active),
            Arc::clone(&global_max),
            Arc::clone(&completion),
        ));
        let second_probe = Arc::new(TestWorkerProbe::new(
            second,
            start,
            Arc::clone(&global_active),
            Arc::clone(&global_max),
            Arc::clone(&completion),
        ));
        scheduler
            .set_probe(first, Arc::clone(&first_probe))
            .expect("first probe");
        scheduler
            .set_probe(second, Arc::clone(&second_probe))
            .expect("second probe");

        let outcome = scheduler.tick().expect("parallel shadow tick");
        let visited: Vec<_> = outcome.visits.iter().map(|visit| visit.shard).collect();
        let output_order: Vec<_> = outcome.shards.iter().map(|output| output.shard).collect();
        let unique: BTreeSet<_> = visited.iter().copied().collect();

        assert_eq!(outcome.tick, Tick::new(1));
        assert_eq!(completion.observed(), vec![second, first]);
        assert_eq!(visited, vec![first, second]);
        assert_eq!(output_order, vec![first, second]);
        assert_eq!(unique.len(), visited.len());
        assert_eq!(global_max.load(Ordering::SeqCst), 2);
        assert_eq!(global_active.load(Ordering::SeqCst), 0);
        assert_eq!(first_probe.max_active(), 1);
        assert_eq!(second_probe.max_active(), 1);
        assert_eq!(first_probe.active(), 0);
        assert_eq!(second_probe.active(), 0);
        assert!(matches!(
            outcome.shards[0].outputs.as_slice(),
            [GameOutput::PlayerSpawned { player, .. }] if *player == first_player
        ));
        assert!(matches!(
            outcome.shards[1].outputs.as_slice(),
            [GameOutput::PlayerSpawned { player, .. }] if *player == second_player
        ));

        // A second dispatch reuses the same fixed threads; the start barrier is
        // reusable and again proves the two distinct shards overlap.
        let second_tick = scheduler.tick().expect("second parallel shadow tick");
        assert_eq!(second_tick.tick, Tick::new(2));
        assert!(first_probe.ran_on_one_persistent_thread(2));
        assert!(second_probe.ran_on_one_persistent_thread(2));
        assert_eq!(first_probe.max_active(), 1);
        assert_eq!(second_probe.max_active(), 1);
    }

    #[test]
    fn authoritative_mode_rejects_a_second_active_shard_before_tick() {
        let first_shard = SimShard::new(ShardPos::new(0, 0));
        let first = logical_id(
            WorldId::new(0),
            DimensionId::new(0),
            first_shard.shard_pos(),
        );
        let mut scheduler =
            ShardScheduler::authoritative(TickCoordinator::new(TickRate::VANILLA), first_shard)
                .expect("authoritative shard");
        let mut second_shard = SimShard::new(ShardPos::new(1, 0));
        second_shard
            .enqueue(GameInput::PlayerJoin {
                player: PlayerId::offline("not-admitted"),
                position: Vec3::ZERO,
            })
            .expect("staged shard inbox");
        let second = scheduler
            .register(second_shard)
            .expect("created shard may be staged");

        assert_eq!(
            scheduler.activate(second),
            Err(SimError::MultipleScheduledShardsDisabled { scheduled: 2 })
        );
        assert_eq!(scheduler.current_tick(), Tick::ZERO);
        assert_eq!(scheduler.shard(first).expect("first").player_count(), 0);
        assert_eq!(scheduler.shard(second).expect("second").inbox_len(), 1);
        assert_eq!(
            scheduler.lifecycle(second).expect("second lifecycle"),
            ShardLifecycleState::Created
        );
    }

    #[test]
    fn draining_shard_finishes_admitted_work_and_rejects_new_input() {
        let player = PlayerId::offline("draining");
        let mut scheduler = ShardScheduler::with_shadow_workers(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::MIN,
        )
        .expect("worker pool");
        let shard = scheduler
            .register(SimShard::new(ShardPos::new(0, 0)))
            .expect("register");
        scheduler.activate(shard).expect("activate");
        scheduler
            .enqueue(
                shard,
                GameInput::PlayerJoin {
                    player,
                    position: Vec3::ZERO,
                },
            )
            .expect("admit before drain");
        scheduler
            .transition(shard, ShardLifecycleState::Draining)
            .expect("begin draining");

        assert_eq!(
            scheduler.transition(shard, ShardLifecycleState::Stopped),
            Err(SimError::ShardDrainIncomplete { shard })
        );
        assert_eq!(
            scheduler.enqueue(shard, GameInput::PlayerLeave { player }),
            Err(SimError::ShardInputNotAccepted {
                shard,
                state: ShardLifecycleState::Draining,
            })
        );
        let outcome = scheduler.tick().expect("draining tick");
        assert_eq!(outcome.visits[0].shard, shard);
        assert_eq!(
            outcome.shards[0].outputs,
            vec![GameOutput::PlayerSpawned {
                player,
                position: Vec3::ZERO,
            }]
        );
        assert_eq!(scheduler.shard(shard).expect("shard").inbox_len(), 0);
        scheduler
            .transition(shard, ShardLifecycleState::Stopped)
            .expect("quiescent shard stops");
        assert_eq!(
            scheduler.lifecycle(shard),
            Some(ShardLifecycleState::Stopped)
        );
    }

    #[test]
    fn worker_panic_poison_is_fail_stop_and_cannot_be_retried() {
        let player = PlayerId::offline("worker-panic");
        let mut scheduler = ShardScheduler::with_shadow_workers(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::MIN,
        )
        .expect("worker pool");
        let shard = scheduler
            .register(SimShard::new(ShardPos::new(0, 0)))
            .expect("register");
        scheduler.activate(shard).expect("activate");
        scheduler
            .enqueue(
                shard,
                GameInput::PlayerJoin {
                    player,
                    position: Vec3::ZERO,
                },
            )
            .expect("inbox");
        scheduler.set_panic_on_run(shard).expect("panic injection");

        assert_eq!(
            scheduler.tick(),
            Err(SimError::ShardWorkerPanicked {
                slot: 0,
                tick: Tick::new(1),
            })
        );
        assert_eq!(scheduler.current_tick(), Tick::new(1));
        assert_eq!(
            scheduler.tick(),
            Err(SimError::ShardSchedulerPoisoned { tick: Tick::new(1) })
        );
        assert_eq!(
            scheduler.enqueue(shard, GameInput::PlayerLeave { player }),
            Err(SimError::ShardSchedulerPoisoned { tick: Tick::new(1) })
        );
    }

    #[test]
    fn worker_pool_rejects_an_unbounded_thread_count() {
        let requested = MAX_SHARD_WORKERS + 1;
        let result = ShardScheduler::with_shadow_workers(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::new(requested).expect("nonzero"),
        );
        assert!(matches!(
            result,
            Err(SimError::TooManyShardWorkers {
                requested: actual,
                maximum: MAX_SHARD_WORKERS,
            }) if actual == requested
        ));
    }

    #[test]
    fn tick_overflow_leaves_all_shard_inboxes_untouched() {
        let mut scheduler = ShardScheduler::with_shadow_workers(
            TickCoordinator::resuming_at(TickRate::VANILLA, Tick::new(u64::MAX)),
            NonZeroUsize::new(2).expect("nonzero"),
        )
        .expect("worker pool");
        let mut ids = Vec::new();
        for x in [0, 1] {
            let id = scheduler
                .register(SimShard::new(ShardPos::new(x, 0)))
                .expect("register");
            scheduler.activate(id).expect("activate");
            scheduler
                .enqueue(
                    id,
                    GameInput::PlayerJoin {
                        player: PlayerId::offline(&format!("overflow-{x}")),
                        position: Vec3::ZERO,
                    },
                )
                .expect("inbox");
            ids.push(id);
        }

        assert!(matches!(scheduler.tick(), Err(SimError::TickOverflow)));
        assert_eq!(scheduler.current_tick(), Tick::new(u64::MAX));
        for id in ids {
            let shard = scheduler.shard(id).expect("registered");
            assert_eq!(shard.inbox_len(), 1);
            assert_eq!(shard.player_count(), 0);
        }
    }

    #[test]
    fn cross_shard_message_produced_in_tick_n_applies_only_at_n_plus_one() {
        let mut scheduler = ShardScheduler::with_shadow_workers_and_cross_shard_capacity(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::MIN,
            NonZeroUsize::new(4).expect("nonzero queue"),
        )
        .expect("worker pool");
        let source = scheduler
            .register(SimShard::new(ShardPos::new(0, 0)))
            .expect("source");
        let destination = scheduler
            .register(SimShard::with_inbox_capacity(
                ShardPos::new(1, 0),
                NonZeroUsize::MIN,
            ))
            .expect("destination");
        scheduler.activate(source).expect("activate source");
        scheduler
            .activate(destination)
            .expect("activate destination");

        let player = PlayerId::offline("next-boundary");
        let transfer = GameInput::PlayerJoin {
            player,
            position: Vec3::new(130.0, 70.0, 8.0),
        };
        scheduler
            .emit_cross_shard_during_next_tick(
                source,
                destination,
                CrossShardPayload::ApplyInput(transfer),
            )
            .expect("source outbox has room");

        assert_eq!(scheduler.cross_shard_queue_len(), 0);
        assert!(!scheduler
            .shard(destination)
            .expect("destination")
            .contains_player(player));

        let tick_n = scheduler.tick().expect("source production tick");
        assert_eq!(tick_n.tick(), Tick::new(1));
        assert!(tick_n
            .shard_outputs()
            .all(|(_, outputs)| outputs.is_empty()));
        assert!(tick_n.cross_shard_rejections().is_empty());
        assert!(!scheduler
            .shard(destination)
            .expect("destination")
            .contains_player(player));
        assert_eq!(scheduler.cross_shard_queue_len(), 1);

        scheduler
            .enqueue(destination, GameInput::PlayerLeave { player })
            .expect("ordinary inbox has its one slot");
        let tick_n_plus_one = scheduler.tick().expect("destination boundary tick");
        assert_eq!(tick_n_plus_one.tick(), Tick::new(2));
        let destination_outputs = tick_n_plus_one
            .shard_outputs()
            .find_map(|(shard, outputs)| (shard == destination).then_some(outputs))
            .expect("destination outcome");
        assert_eq!(
            destination_outputs,
            &[
                GameOutput::PlayerSpawned {
                    player,
                    position: Vec3::new(130.0, 70.0, 8.0),
                },
                GameOutput::PlayerDespawned { player },
            ]
        );
        assert!(!scheduler
            .shard(destination)
            .expect("destination")
            .contains_player(player));
        assert_eq!(scheduler.cross_shard_queue_len(), 0);

        let tick_n_plus_two = scheduler.tick().expect("following tick");
        assert!(tick_n_plus_two
            .shard_outputs()
            .all(|(_, outputs)| outputs.is_empty()));
        assert_eq!(scheduler.cross_shard_queue_len(), 0);
        assert!(
            !scheduler
                .shard(destination)
                .expect("destination")
                .contains_player(player),
            "a duplicate N+2 application would visibly re-add the player"
        );
    }

    #[test]
    fn cross_shard_nan_payload_uses_metadata_commit_identity() {
        let mut scheduler = ShardScheduler::with_shadow_workers_and_cross_shard_capacity(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::MIN,
            NonZeroUsize::new(2).expect("nonzero queue"),
        )
        .expect("worker pool");
        let source = scheduler
            .register(SimShard::new(ShardPos::new(0, 0)))
            .expect("source");
        let destination = scheduler
            .register(SimShard::new(ShardPos::new(1, 0)))
            .expect("destination");
        scheduler.activate(source).expect("activate source");
        scheduler
            .activate(destination)
            .expect("activate destination");

        let player = PlayerId::offline("nan-commit-identity");
        scheduler
            .emit_cross_shard_during_next_tick(
                source,
                destination,
                CrossShardPayload::ApplyInput(GameInput::PlayerMove {
                    player,
                    position: None,
                    yaw: Some(f32::NAN),
                    pitch: None,
                }),
            )
            .expect("source outbox has room");

        scheduler.tick().expect("production tick");
        let boundary = scheduler
            .tick()
            .expect("NaN payload must not falsify boundary identity");
        assert_eq!(scheduler.cross_shard_queue_len(), 0);
        assert!(boundary
            .shard_outputs()
            .all(|(_, outputs)| outputs.is_empty()));
        assert!(!scheduler
            .shard(destination)
            .expect("destination")
            .contains_player(player));

        let following = scheduler.tick().expect("scheduler remains healthy");
        assert!(following
            .shard_outputs()
            .all(|(_, outputs)| outputs.is_empty()));
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        reason = "one adversarial scenario pins concurrency, capacity, ownership, and ordering"
    )]
    fn cross_shard_queue_rejects_full_and_applies_in_canonical_order() {
        let capacity = NonZeroUsize::new(3).expect("nonzero queue");
        let mut scheduler = ShardScheduler::with_shadow_workers_and_cross_shard_capacity(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::new(2).expect("nonzero workers"),
            capacity,
        )
        .expect("worker pool");
        let source_a = scheduler
            .register(SimShard::new(ShardPos::new(0, 0)))
            .expect("source a");
        let source_b = scheduler
            .register(SimShard::new(ShardPos::new(1, 0)))
            .expect("source b");
        let destination = scheduler
            .register(SimShard::new(ShardPos::new(2, 0)))
            .expect("destination");
        for shard in [source_a, source_b, destination] {
            scheduler.activate(shard).expect("activate");
        }
        let start = Arc::new(Barrier::new(2));
        let global_active = Arc::new(AtomicUsize::new(0));
        let global_max = Arc::new(AtomicUsize::new(0));
        let completion = Arc::new(TestCompletionOrder::new(source_b));
        let lower_source_probe = Arc::new(TestWorkerProbe::new(
            source_a,
            Arc::clone(&start),
            Arc::clone(&global_active),
            Arc::clone(&global_max),
            Arc::clone(&completion),
        ));
        let higher_source_probe = Arc::new(TestWorkerProbe::new(
            source_b,
            start,
            Arc::clone(&global_active),
            Arc::clone(&global_max),
            Arc::clone(&completion),
        ));
        scheduler
            .set_probe(source_a, lower_source_probe)
            .expect("source a probe");
        scheduler
            .set_probe(source_b, higher_source_probe)
            .expect("source b probe");

        let player_a_first = PlayerId::offline("source-a-first");
        let player_a_second = PlayerId::offline("source-a-second");
        let player_b = PlayerId::offline("source-b");
        let rejected_player = PlayerId::offline("queue-rejected");
        let from_b = GameInput::PlayerJoin {
            player: player_b,
            position: Vec3::new(260.0, 70.0, 8.0),
        };
        let from_a_first = GameInput::PlayerJoin {
            player: player_a_first,
            position: Vec3::new(261.0, 70.0, 8.0),
        };
        let from_a_second = GameInput::PlayerJoin {
            player: player_a_second,
            position: Vec3::new(262.0, 70.0, 8.0),
        };
        let rejected = GameInput::PlayerJoin {
            player: rejected_player,
            position: Vec3::new(263.0, 70.0, 8.0),
        };

        // Stage the higher source first. Boundary application must still sort by
        // destination, source id, then source-local emission order.
        scheduler
            .emit_cross_shard_during_next_tick(
                source_b,
                destination,
                CrossShardPayload::ApplyInput(from_b),
            )
            .expect("source b first");
        scheduler
            .emit_cross_shard_during_next_tick(
                source_a,
                destination,
                CrossShardPayload::ApplyInput(from_a_first),
            )
            .expect("source a first");
        scheduler
            .emit_cross_shard_during_next_tick(
                source_a,
                destination,
                CrossShardPayload::ApplyInput(from_a_second),
            )
            .expect("source a second");
        scheduler
            .emit_cross_shard_during_next_tick(
                source_b,
                destination,
                CrossShardPayload::ApplyInput(rejected.clone()),
            )
            .expect("source b second");

        let mut production = scheduler.tick().expect("production tick");
        assert_eq!(completion.observed(), vec![source_b, source_a]);
        assert_eq!(global_max.load(Ordering::SeqCst), 2);
        assert!(production
            .shard_outputs()
            .all(|(_, outputs)| outputs.is_empty()));
        let mut rejections = production.take_cross_shard_rejections();
        assert_eq!(rejections.len(), 1);
        let rejection = rejections.pop().expect("one owned rejection");
        assert_eq!(
            rejection.reason(),
            CrossShardRejectionReason::QueueFull {
                capacity: capacity.get(),
            }
        );
        assert_eq!(rejection.envelope().source(), source_b);
        assert_eq!(rejection.envelope().destination(), destination);
        let rejected_envelope = rejection.into_envelope();
        assert_eq!(
            rejected_envelope.payload(),
            &CrossShardPayload::ApplyInput(rejected)
        );
        assert_eq!(scheduler.cross_shard_queue_len(), capacity.get());

        let boundary = scheduler.tick().expect("application tick");
        let destination_outputs = boundary
            .shard_outputs()
            .find_map(|(shard, outputs)| (shard == destination).then_some(outputs))
            .expect("destination outcome");
        assert_eq!(
            destination_outputs,
            &[
                GameOutput::PlayerSpawned {
                    player: player_a_first,
                    position: Vec3::new(261.0, 70.0, 8.0),
                },
                GameOutput::PlayerSpawned {
                    player: player_a_second,
                    position: Vec3::new(262.0, 70.0, 8.0),
                },
                GameOutput::PlayerSpawned {
                    player: player_b,
                    position: Vec3::new(260.0, 70.0, 8.0),
                },
            ]
        );
        assert!(!scheduler
            .shard(destination)
            .expect("destination")
            .contains_player(rejected_player));
        assert_eq!(scheduler.cross_shard_queue_len(), 0);
    }

    #[test]
    fn admitted_cross_shard_work_drains_before_destination_stops() {
        let mut scheduler = ShardScheduler::with_shadow_workers_and_cross_shard_capacity(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::new(2).expect("nonzero workers"),
            NonZeroUsize::new(2).expect("nonzero queue"),
        )
        .expect("worker pool");
        let source = scheduler
            .register(SimShard::new(ShardPos::new(0, 0)))
            .expect("source");
        let destination = scheduler
            .register(SimShard::new(ShardPos::new(1, 0)))
            .expect("destination");
        scheduler.activate(source).expect("activate source");
        scheduler
            .activate(destination)
            .expect("activate destination");

        let player = PlayerId::offline("draining-boundary");
        scheduler
            .emit_cross_shard_during_next_tick(
                source,
                destination,
                CrossShardPayload::ApplyInput(GameInput::PlayerJoin {
                    player,
                    position: Vec3::new(132.0, 70.0, 8.0),
                }),
            )
            .expect("source outbox");
        scheduler.tick().expect("production tick");
        assert_eq!(scheduler.cross_shard_queue_len(), 1);

        scheduler
            .transition(destination, ShardLifecycleState::Draining)
            .expect("begin draining");
        assert_eq!(
            scheduler.transition(destination, ShardLifecycleState::Stopped),
            Err(SimError::ShardDrainIncomplete { shard: destination })
        );

        let boundary = scheduler.tick().expect("draining boundary tick");
        let destination_outputs = boundary
            .shard_outputs()
            .find_map(|(shard, outputs)| (shard == destination).then_some(outputs))
            .expect("destination outcome");
        assert_eq!(
            destination_outputs,
            &[GameOutput::PlayerSpawned {
                player,
                position: Vec3::new(132.0, 70.0, 8.0),
            }]
        );
        assert_eq!(scheduler.cross_shard_queue_len(), 0);
        scheduler
            .transition(destination, ShardLifecycleState::Stopped)
            .expect("admitted boundary work drained");
    }

    #[test]
    fn invalid_cross_shard_destinations_reject_without_mutation() {
        let mut scheduler = ShardScheduler::with_shadow_workers_and_cross_shard_capacity(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::new(2).expect("nonzero workers"),
            NonZeroUsize::new(8).expect("nonzero queue"),
        )
        .expect("worker pool");
        let source = scheduler
            .register(SimShard::new(ShardPos::new(0, 0)))
            .expect("source");
        let created = scheduler
            .register(SimShard::new(ShardPos::new(1, 0)))
            .expect("created");
        let draining = scheduler
            .register(SimShard::new(ShardPos::new(2, 0)))
            .expect("draining");
        let stopped = scheduler
            .register(SimShard::new(ShardPos::new(3, 0)))
            .expect("stopped");
        let unknown = logical_id(WorldId::new(0), DimensionId::new(0), ShardPos::new(4, 0));
        scheduler.activate(source).expect("activate source");
        scheduler.activate(draining).expect("activate draining");
        scheduler
            .transition(draining, ShardLifecycleState::Draining)
            .expect("drain");
        scheduler.activate(stopped).expect("activate stopped");
        scheduler
            .transition(stopped, ShardLifecycleState::Draining)
            .expect("drain stopped");
        scheduler
            .transition(stopped, ShardLifecycleState::Stopped)
            .expect("stop");

        for (destination, name) in [
            (source, "same"),
            (created, "created"),
            (draining, "draining"),
            (stopped, "stopped"),
            (unknown, "unknown"),
        ] {
            scheduler
                .emit_cross_shard_during_next_tick(
                    source,
                    destination,
                    CrossShardPayload::ApplyInput(GameInput::PlayerJoin {
                        player: PlayerId::offline(name),
                        position: Vec3::ZERO,
                    }),
                )
                .expect("bounded source outbox");
        }

        let outcome = scheduler.tick().expect("production tick");
        let reasons: Vec<_> = outcome
            .cross_shard_rejections()
            .iter()
            .map(crate::cross_shard::CrossShardRejection::reason)
            .collect();
        assert_eq!(
            reasons,
            vec![
                CrossShardRejectionReason::SameShard,
                CrossShardRejectionReason::DestinationNotActive {
                    state: ShardLifecycleState::Created,
                },
                CrossShardRejectionReason::DestinationNotActive {
                    state: ShardLifecycleState::Draining,
                },
                CrossShardRejectionReason::DestinationNotActive {
                    state: ShardLifecycleState::Stopped,
                },
                CrossShardRejectionReason::UnknownDestination,
            ]
        );
        assert_eq!(scheduler.cross_shard_queue_len(), 0);
        for shard in [source, created, draining, stopped] {
            assert_eq!(
                scheduler.shard(shard).expect("registered").player_count(),
                0
            );
        }
    }

    #[test]
    fn tick_overflow_preserves_a_ready_cross_shard_boundary() {
        let mut scheduler = ShardScheduler::with_shadow_workers_and_cross_shard_capacity(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::MIN,
            NonZeroUsize::MIN,
        )
        .expect("worker pool");
        let source = scheduler
            .register(SimShard::new(ShardPos::new(0, 0)))
            .expect("source");
        let destination = scheduler
            .register(SimShard::new(ShardPos::new(1, 0)))
            .expect("destination");
        scheduler.activate(source).expect("activate source");
        scheduler
            .activate(destination)
            .expect("activate destination");
        let player = PlayerId::offline("overflow-boundary");
        scheduler
            .emit_cross_shard_during_next_tick(
                source,
                destination,
                CrossShardPayload::ApplyInput(GameInput::PlayerJoin {
                    player,
                    position: Vec3::new(133.0, 70.0, 8.0),
                }),
            )
            .expect("source outbox");
        scheduler.tick().expect("production tick");
        assert_eq!(scheduler.cross_shard_queue_len(), 1);

        scheduler.coordinator =
            TickCoordinator::resuming_at(TickRate::VANILLA, Tick::new(u64::MAX));
        assert_eq!(scheduler.tick(), Err(SimError::TickOverflow));
        assert_eq!(scheduler.cross_shard_queue_len(), 1);
        assert!(!scheduler
            .shard(destination)
            .expect("destination")
            .contains_player(player));

        scheduler.coordinator = TickCoordinator::resuming_at(TickRate::VANILLA, Tick::new(1));
        let boundary = scheduler.tick().expect("retry exact boundary");
        assert_eq!(boundary.tick(), Tick::new(2));
        assert!(scheduler
            .shard(destination)
            .expect("destination")
            .contains_player(player));
        assert_eq!(scheduler.cross_shard_queue_len(), 0);
    }

    #[tokio::test]
    async fn single_shard_persist_batch_matches_legacy_drain() {
        let chunks = [
            ChunkPos::new(0, 0),
            // Current authoritative shards intentionally retain arbitrary
            // view-distance chunks outside their nominal 8x8 region.
            ChunkPos::new(8, 0),
        ];
        let mut legacy = persist_dirty_shard(ShardPos::new(0, 0), &chunks).await;
        let scheduled = legacy.clone();
        let expected = legacy.loaded_chunks_mut().take_persist_dirty(1);
        let expected_shard = logical_id(
            scheduled.loaded_chunks().world(),
            scheduled.loaded_chunks().dimension(),
            scheduled.shard_pos(),
        );
        let mut runtime =
            ShardScheduler::authoritative(TickCoordinator::new(TickRate::VANILLA), scheduled)
                .expect("authoritative scheduler");

        let mut outcome = runtime.tick().expect("single-shard tick");
        assert_eq!(outcome.persist_batches().len(), 1);
        let batch = &outcome.persist_batches()[0];
        assert_eq!(batch.shard(), expected_shard);
        assert_eq!(batch.capacity(), MAX_SAVE_BATCH);
        assert_eq!(batch.records(), expected.as_slice());
        assert!(!batch.has_deferred());
        assert!(!runtime
            .shard(expected_shard)
            .expect("scheduled shard")
            .loaded_chunks()
            .has_persist_dirty());
        let mut owned_batches = outcome.take_persist_batches();
        assert_eq!(owned_batches.len(), 1);
        assert_eq!(
            owned_batches.pop().expect("one owned batch").into_records(),
            expected
        );
    }

    #[tokio::test]
    async fn per_shard_persist_batches_are_canonical_and_non_overlapping() {
        let definitions = [
            (
                ShardPos::new(1, 0),
                [ChunkPos::new(9, 0), ChunkPos::new(8, 0)],
            ),
            (
                ShardPos::new(-1, 0),
                [ChunkPos::new(-1, 0), ChunkPos::new(-8, 0)],
            ),
            (
                ShardPos::new(0, 0),
                [ChunkPos::new(0, 0), ChunkPos::new(7, 0)],
            ),
        ];
        let mut scheduler = ShardScheduler::with_shadow_workers(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::new(2).expect("nonzero workers"),
        )
        .expect("worker pool");
        let mut expected_batches = BTreeMap::new();
        for (position, chunks) in definitions {
            let id = scheduler
                .register(persist_dirty_shard(position, &chunks).await)
                .expect("unique shard");
            scheduler.activate(id).expect("activate");
            let mut keys = chunks
                .map(|chunk| ChunkKey::new(WorldId::new(0), DimensionId::new(0), chunk))
                .to_vec();
            keys.sort_unstable_by_key(|key| key.pos());
            assert!(expected_batches.insert(id, keys).is_none());
        }

        let outcome = scheduler.tick().expect("multi-shard tick");
        let batches = outcome.persist_batches();
        assert_eq!(
            batches
                .iter()
                .map(super::ShardPersistBatch::shard)
                .collect::<Vec<_>>(),
            expected_batches.keys().copied().collect::<Vec<_>>()
        );

        let mut seen = BTreeSet::new();
        for batch in batches {
            assert_eq!(batch.records().len(), 2);
            assert!(!batch.has_deferred());
            let actual_keys = batch
                .records()
                .iter()
                .map(|(key, _)| *key)
                .collect::<Vec<_>>();
            assert_eq!(
                actual_keys,
                expected_batches
                    .remove(&batch.shard())
                    .expect("registered shard has an expected batch")
            );
            for (key, _) in batch.records() {
                assert!(seen.insert(*key), "a chunk key may occur in one batch only");
            }
        }
        assert!(expected_batches.is_empty());
        assert_eq!(seen.len(), 6);
    }

    #[tokio::test]
    async fn persist_batch_capacity_defers_overflow_without_clearing_it() {
        let chunks = [ChunkPos::new(0, 0), ChunkPos::new(1, 0)];
        let mut scheduler = ShardScheduler::with_shadow_workers_and_capacities(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::MIN,
            NonZeroUsize::new(4).expect("nonzero cross-shard capacity"),
            NonZeroUsize::MIN,
        )
        .expect("worker pool");
        let shard = scheduler
            .register(persist_dirty_shard(ShardPos::new(0, 0), &chunks).await)
            .expect("shard");
        scheduler.activate(shard).expect("activate");

        let first = scheduler.tick().expect("first bounded drain");
        let first_batch = &first.persist_batches()[0];
        assert_eq!(first_batch.capacity(), 1);
        assert_eq!(first_batch.records().len(), 1);
        assert_eq!(first_batch.records()[0].0.pos(), chunks[0]);
        assert!(first_batch.has_deferred());
        assert!(scheduler
            .shard(shard)
            .expect("scheduled shard")
            .loaded_chunks()
            .has_persist_dirty());
        scheduler
            .shards
            .get_mut(&shard)
            .expect("scheduled shard")
            .shard
            .loaded_chunks_mut()
            .get_mut(chunks[0])
            .expect("first chunk remains resident")
            .mark_persist_dirty(chunks[0].origin_block(70));
        scheduler
            .transition(shard, ShardLifecycleState::Draining)
            .expect("begin drain");
        scheduler
            .transition(shard, ShardLifecycleState::Stopped)
            .expect("persist output does not keep a shard runnable");

        let second = scheduler.tick().expect("deferred drain");
        assert_eq!(second.visit_order().count(), 0);
        let second_batch = &second.persist_batches()[0];
        assert_eq!(second_batch.shard(), shard);
        assert_eq!(second_batch.records().len(), 1);
        assert_eq!(second_batch.records()[0].0.pos(), chunks[1]);
        assert!(second_batch.has_deferred());
        assert!(scheduler
            .shard(shard)
            .expect("scheduled shard")
            .loaded_chunks()
            .has_persist_dirty());

        let third = scheduler.tick().expect("wrapped re-dirty drain");
        assert_eq!(third.visit_order().count(), 0);
        let third_batch = &third.persist_batches()[0];
        assert_eq!(third_batch.records().len(), 1);
        assert_eq!(third_batch.records()[0].0.pos(), chunks[0]);
        assert!(!third_batch.has_deferred());
        assert!(!scheduler
            .shard(shard)
            .expect("scheduled shard")
            .loaded_chunks()
            .has_persist_dirty());
    }

    #[tokio::test]
    async fn aliased_resident_chunk_is_rejected_before_tick_or_batch_drain() {
        let aliased = ChunkPos::new(0, 0);
        let mut scheduler = ShardScheduler::with_shadow_workers(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::new(2).expect("nonzero workers"),
        )
        .expect("worker pool");
        let owner = scheduler
            .register(persist_dirty_shard(ShardPos::new(0, 0), &[aliased]).await)
            .expect("owner");
        let mut clean_alias = persist_dirty_shard(ShardPos::new(1, 0), &[aliased]).await;
        assert_eq!(
            clean_alias.loaded_chunks_mut().take_persist_dirty(0).len(),
            1
        );
        let foreign = scheduler.register(clean_alias).expect("foreign");
        scheduler.activate(owner).expect("activate owner");
        scheduler.activate(foreign).expect("activate foreign");
        let player = PlayerId::offline("alias-preflight");
        scheduler
            .enqueue(
                owner,
                GameInput::PlayerJoin {
                    player,
                    position: Vec3::ZERO,
                },
            )
            .expect("owner inbox");

        assert_eq!(
            scheduler.tick(),
            Err(SimError::PersistChunkAliased {
                key: ChunkKey::new(WorldId::new(0), DimensionId::new(0), aliased),
                first: owner,
                second: foreign,
            })
        );
        assert_eq!(scheduler.current_tick(), Tick::ZERO);
        assert!(scheduler
            .shard(owner)
            .expect("owner")
            .loaded_chunks()
            .has_persist_dirty());
        assert!(!scheduler
            .shard(foreign)
            .expect("foreign")
            .loaded_chunks()
            .has_persist_dirty());
        assert_eq!(scheduler.shard(owner).expect("owner").inbox_len(), 1);
        assert!(!scheduler
            .shard(owner)
            .expect("owner")
            .contains_player(player));
        assert_eq!(
            scheduler.tick(),
            Err(SimError::PersistChunkAliased {
                key: ChunkKey::new(WorldId::new(0), DimensionId::new(0), aliased),
                first: owner,
                second: foreign,
            })
        );
    }

    #[tokio::test]
    async fn equal_chunk_coordinates_in_distinct_worlds_are_not_aliases() {
        let chunk = ChunkPos::new(0, 0);
        let mut scheduler = ShardScheduler::with_shadow_workers(
            TickCoordinator::new(TickRate::VANILLA),
            NonZeroUsize::new(2).expect("nonzero workers"),
        )
        .expect("worker pool");
        for world in [WorldId::new(1), WorldId::new(2)] {
            let shard = persist_dirty_shard_in_dimension(
                ShardPos::new(0, 0),
                world,
                DimensionId::new(0),
                &[chunk],
            )
            .await;
            let id = scheduler.register(shard).expect("world-scoped shard");
            scheduler.activate(id).expect("activate");
        }

        let outcome = scheduler.tick().expect("world-scoped keys are distinct");
        assert_eq!(outcome.persist_batches().len(), 2);
        let keys: BTreeSet<_> = outcome
            .persist_batches()
            .iter()
            .flat_map(|batch| batch.records().iter().map(|(key, _)| *key))
            .collect();
        assert_eq!(
            keys,
            BTreeSet::from([
                ChunkKey::new(WorldId::new(1), DimensionId::new(0), chunk),
                ChunkKey::new(WorldId::new(2), DimensionId::new(0), chunk),
            ])
        );
    }

    #[tokio::test]
    async fn post_worker_failure_keeps_persist_dirty_ownership_in_the_shard() {
        let terminal = Tick::new(u64::MAX);
        let mut scheduler = ShardScheduler::with_shadow_workers(
            TickCoordinator::resuming_at(TickRate::VANILLA, Tick::new(u64::MAX - 1)),
            NonZeroUsize::MIN,
        )
        .expect("worker pool");
        let source = scheduler
            .register(persist_dirty_shard(ShardPos::new(0, 0), &[ChunkPos::new(0, 0)]).await)
            .expect("source");
        let destination = scheduler
            .register(SimShard::new(ShardPos::new(1, 0)))
            .expect("destination");
        scheduler.activate(source).expect("activate source");
        scheduler
            .activate(destination)
            .expect("activate destination");
        scheduler
            .emit_cross_shard_during_next_tick(
                source,
                destination,
                CrossShardPayload::ApplyInput(GameInput::PlayerLeave {
                    player: PlayerId::offline("terminal-persist"),
                }),
            )
            .expect("source outbox");

        assert_eq!(
            scheduler.tick(),
            Err(SimError::CrossShardEnvelopeTickOverflow {
                shard: source,
                tick: terminal,
            })
        );
        assert_eq!(scheduler.current_tick(), terminal);
        assert!(scheduler
            .shard(source)
            .expect("source")
            .loaded_chunks()
            .has_persist_dirty());
        assert_eq!(
            scheduler.tick(),
            Err(SimError::ShardSchedulerPoisoned { tick: terminal })
        );
    }

    /// Exact, topology-independent canonical bytes for one completed tick.
    ///
    /// Equality compares the bytes rather than a hash, so the regression cannot
    /// pass because of a digest collision. The stable FNV-1a fingerprint exists
    /// only to keep divergence diagnostics compact.
    #[derive(Clone, PartialEq, Eq)]
    struct ShadowStateDigest(Vec<u8>);

    impl ShadowStateDigest {
        /// Returns a stable compact fingerprint for diagnostics.
        fn fingerprint(&self) -> u128 {
            const FNV_128_OFFSET_BASIS: u128 = 0x6c62_272e_07bb_0142_62b8_2175_6295_c58d;
            const FNV_128_PRIME: u128 = 0x0000_0000_0100_0000_0000_0000_0000_013b;
            self.0.iter().fold(FNV_128_OFFSET_BASIS, |hash, byte| {
                (hash ^ u128::from(*byte)).wrapping_mul(FNV_128_PRIME)
            })
        }
    }

    impl std::fmt::Debug for ShadowStateDigest {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                formatter,
                "ShadowStateDigest({:032x}, {} bytes)",
                self.fingerprint(),
                self.0.len()
            )
        }
    }

    impl std::fmt::Display for ShadowStateDigest {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "{:032x}", self.fingerprint())
        }
    }

    /// First tick where the one-owner and partitioned shadow states differed.
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct ShadowDigestMismatch {
        tick: Tick,
        one_shard: ShadowStateDigest,
        n_shard: ShadowStateDigest,
    }

    impl ShadowDigestMismatch {
        /// Returns the first divergent completed tick.
        const fn tick(&self) -> Tick {
            self.tick
        }

        /// Returns the digest from the authoritative one-owner configuration.
        const fn one_shard(&self) -> &ShadowStateDigest {
            &self.one_shard
        }

        /// Returns the digest from the partitioned shadow configuration.
        const fn n_shard(&self) -> &ShadowStateDigest {
            &self.n_shard
        }
    }

    impl std::fmt::Display for ShadowDigestMismatch {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                formatter,
                "shadow state diverged at tick {}: one-shard={}, n-shard={}",
                self.tick, self.one_shard, self.n_shard
            )
        }
    }

    /// Drives one authoritative owner and an N-owner shadow from one semantic
    /// shard-addressed input log.
    struct ShadowDeterminismHarness {
        one_shard: ShardScheduler,
        n_shard: ShardScheduler,
        one_shard_id: ShardId,
        shadow_routes: BTreeMap<ShardPos, ShardId>,
    }

    impl ShadowDeterminismHarness {
        /// Creates both configurations at tick zero with the same logical
        /// chunks but deliberately different ownership and registration order.
        async fn new<const N: usize>(
            shard_positions: [ShardPos; N],
            worker_slots: NonZeroUsize,
        ) -> Result<Self, SimError> {
            let authoritative_position = shard_positions
                .first()
                .copied()
                .expect("the fixed shadow topology has at least one shard");
            let store = InMemoryStore::new();
            let generator = FlatWorldGenerator::new();

            let mut authoritative_shard = SimShard::new(authoritative_position);
            for position in shard_positions {
                authoritative_shard
                    .loaded_chunks_mut()
                    .acquire(
                        &store,
                        &generator,
                        position.origin_chunk(),
                        ChunkTicket::of(TicketReason::Forced),
                    )
                    .await?;
            }
            let one_shard_id = logical_id(
                authoritative_shard.loaded_chunks().world(),
                authoritative_shard.loaded_chunks().dimension(),
                authoritative_position,
            );
            let one_shard = ShardScheduler::authoritative(
                TickCoordinator::new(TickRate::VANILLA),
                authoritative_shard,
            )?;

            let mut n_shard = ShardScheduler::with_shadow_workers(
                TickCoordinator::new(TickRate::VANILLA),
                worker_slots,
            )?;
            let mut shadow_routes = BTreeMap::new();
            for position in shard_positions.into_iter().rev() {
                let mut shard = SimShard::new(position);
                shard
                    .loaded_chunks_mut()
                    .acquire(
                        &store,
                        &generator,
                        position.origin_chunk(),
                        ChunkTicket::of(TicketReason::Forced),
                    )
                    .await?;
                let id = n_shard.register(shard)?;
                n_shard.activate(id)?;
                assert!(
                    shadow_routes.insert(position, id).is_none(),
                    "the fixed topology has unique logical positions"
                );
            }

            Ok(Self {
                one_shard,
                n_shard,
                one_shard_id,
                shadow_routes,
            })
        }

        /// Replays the fixed log, optionally injecting one shadow-only input.
        ///
        /// The injected path exists solely for the negative control proving the
        /// comparator reports the first divergent tick and both digests.
        fn replay(
            &mut self,
            input_log: &[Vec<(ShardPos, GameInput)>],
            shadow_only: Option<&(Tick, ShardPos, GameInput)>,
        ) -> Result<Vec<(Tick, ShadowStateDigest)>, ShadowDigestMismatch> {
            let mut digests = Vec::with_capacity(input_log.len());
            for (index, inputs) in input_log.iter().enumerate() {
                let tick = Tick::new(
                    u64::try_from(index + 1).expect("the fixed input log length fits u64"),
                );
                for (target, input) in inputs {
                    let shadow_shard = *self
                        .shadow_routes
                        .get(target)
                        .expect("the fixed input log names a registered logical shard");
                    self.one_shard
                        .enqueue(self.one_shard_id, input.clone())
                        .expect("authoritative test inbox has capacity");
                    self.n_shard
                        .enqueue(shadow_shard, input.clone())
                        .expect("shadow test inbox has capacity");
                }
                if let Some((divergent_tick, target, input)) = shadow_only {
                    if *divergent_tick == tick {
                        let shadow_shard = *self
                            .shadow_routes
                            .get(target)
                            .expect("the deliberate divergence names a logical shard");
                        self.n_shard
                            .enqueue(shadow_shard, input.clone())
                            .expect("shadow test inbox has capacity");
                    }
                }

                let one_outcome = self.one_shard.tick().expect("authoritative test tick");
                let n_outcome = self.n_shard.tick().expect("shadow test tick");
                assert_eq!(one_outcome.tick(), tick);
                assert_eq!(n_outcome.tick(), tick);
                let one_digest = semantic_state_digest(&self.one_shard, &one_outcome, tick);
                let n_digest = semantic_state_digest(&self.n_shard, &n_outcome, tick);
                if one_digest != n_digest {
                    return Err(ShadowDigestMismatch {
                        tick,
                        one_shard: one_digest,
                        n_shard: n_digest,
                    });
                }
                digests.push((tick, one_digest));
            }
            Ok(digests)
        }
    }

    /// Appends one full storage key without relying on `Debug` or layout.
    fn append_chunk_key(bytes: &mut Vec<u8>, key: ChunkKey) {
        bytes.extend_from_slice(&key.world().get().to_be_bytes());
        bytes.extend_from_slice(&key.dimension().get().to_be_bytes());
        bytes.extend_from_slice(&key.pos().x().to_be_bytes());
        bytes.extend_from_slice(&key.pos().z().to_be_bytes());
    }

    /// Converts a fixed-width dirty set into its stable low-bit mask.
    fn dirty_mask(dirty: ferrumc_world::DirtySections) -> u32 {
        dirty
            .dirty_indices()
            .fold(0u32, |mask, index| mask | (1u32 << index))
    }

    /// Maps the world's documented YZX flat order back to a local coordinate.
    fn shadow_local_position(flat: usize) -> LocalBlockPos {
        let x = u8::try_from(flat & 0x0f).expect("local x fits u8");
        let z = u8::try_from((flat >> 4) & 0x0f).expect("local z fits u8");
        let y = u8::try_from((flat >> 8) & 0x0f).expect("local y fits u8");
        LocalBlockPos::new(x, y, z).expect("masked local position is valid")
    }

    /// Appends an untruncated UTF-8 value for exact semantic comparison.
    fn append_exact_string(bytes: &mut Vec<u8>, value: &str) {
        append_len(bytes, value.len());
        bytes.extend_from_slice(value.as_bytes());
    }

    /// Appends every modeled field of one sign face without storage-codec
    /// truncation.
    fn append_exact_sign_face(bytes: &mut Vec<u8>, face: &SignFace) {
        for line in face.lines() {
            append_exact_string(bytes, line);
        }
        append_exact_string(bytes, face.color());
        bytes.push(u8::from(face.has_glowing_text()));
    }

    /// Appends the complete item model, including variant identity that the
    /// trusted wire encoding intentionally does not distinguish.
    fn append_exact_item_stack(bytes: &mut Vec<u8>, stack: &ItemStack) {
        match stack.item() {
            Some(item) => {
                bytes.push(1);
                bytes.extend_from_slice(&item.id().to_be_bytes());
            }
            None => bytes.push(0),
        }
        bytes.push(stack.count());
        append_len(bytes, stack.components().added().len());
        for component in stack.components().added() {
            match component {
                ComponentValue::MaxStackSize(value) => {
                    bytes.push(0);
                    bytes.push(*value);
                }
                ComponentValue::Damage(value) => {
                    bytes.push(1);
                    bytes.extend_from_slice(&value.to_be_bytes());
                }
                ComponentValue::MaxDamage(value) => {
                    bytes.push(2);
                    bytes.extend_from_slice(&value.to_be_bytes());
                }
                ComponentValue::Unbreakable => bytes.push(3),
                ComponentValue::CustomName(_) => bytes.push(4),
                ComponentValue::CustomData(_) => bytes.push(5),
                ComponentValue::Opaque(component) => {
                    bytes.push(6);
                    bytes.extend_from_slice(&component.type_id().to_be_bytes());
                    append_len(bytes, component.raw().len());
                    bytes.extend_from_slice(component.raw());
                }
            }
        }
        append_len(bytes, stack.components().removed().len());
        for removed in stack.components().removed() {
            bytes.extend_from_slice(&removed.get().to_be_bytes());
        }

        // The slot body supplies the exact bounded NBT bytes for CustomName and
        // CustomData. The explicit shape above makes typed and opaque variants
        // distinct even when their trusted wire payload happens to be identical.
        let mut slot = Vec::new();
        stack
            .encode_slot(&mut slot)
            .expect("modeled chest stack is valid and encodable");
        append_len(bytes, slot.len());
        bytes.extend_from_slice(&slot);
    }

    /// Appends one block entity with an encoding that is injective over the
    /// currently modeled semantic fields.
    fn append_exact_block_entity(bytes: &mut Vec<u8>, entity: &BlockEntity) {
        match entity {
            BlockEntity::Sign(sign) => {
                bytes.push(0);
                bytes.extend_from_slice(&sign.kind().block_entity_type().to_be_bytes());
                bytes.push(u8::from(sign.is_waxed()));
                append_exact_sign_face(bytes, sign.front());
                append_exact_sign_face(bytes, sign.back());
            }
            BlockEntity::Chest(chest) => {
                bytes.push(1);
                append_len(bytes, chest.slots().len());
                for stack in chest.slots() {
                    append_exact_item_stack(bytes, stack);
                }
            }
            _ => panic!("new block-entity variants require shadow digest encoding"),
        }
    }

    /// Appends every modelled field of a live chunk, independent of palette
    /// representation.
    fn append_live_chunk(
        bytes: &mut Vec<u8>,
        key: ChunkKey,
        schema_version: u32,
        chunk: &Chunk,
        tickets: &[u8],
    ) {
        assert_eq!(
            chunk.pos(),
            key.pos(),
            "resident chunk position must match its full storage key"
        );
        append_chunk_key(bytes, key);
        bytes.extend_from_slice(&chunk.pos().x().to_be_bytes());
        bytes.extend_from_slice(&chunk.pos().z().to_be_bytes());
        bytes.extend_from_slice(&schema_version.to_be_bytes());
        bytes.extend_from_slice(&dirty_mask(*chunk.dirty_sections()).to_be_bytes());
        bytes.extend_from_slice(&dirty_mask(*chunk.persist_dirty_sections()).to_be_bytes());
        bytes.extend_from_slice(&dirty_mask(*chunk.persist_edited_sections()).to_be_bytes());
        bytes.push(u8::from(chunk.light().is_some()));
        append_len(bytes, SECTION_COUNT);
        for (section_index, section) in chunk.sections().iter().enumerate() {
            append_len(bytes, section_index);
            for flat in 0..SECTION_VOLUME {
                bytes.extend_from_slice(
                    &section
                        .get(shadow_local_position(flat))
                        .as_u32()
                        .to_be_bytes(),
                );
            }
        }

        append_len(bytes, chunk.block_entity_count());
        for (position, entity) in chunk.block_entities() {
            append_block_position(bytes, position);
            append_exact_block_entity(bytes, entity);
        }
        append_len(bytes, tickets.len());
        bytes.extend_from_slice(tickets);
    }

    /// Appends the exact semantic payload of one emitted overlay.
    fn canonical_overlay_record(
        key: ChunkKey,
        record: &ferrumc_storage::ChunkOverlayRecord,
    ) -> Vec<u8> {
        assert_eq!(
            key.pos(),
            record.pos(),
            "persist record position must match its full storage key"
        );
        let mut bytes = Vec::new();
        append_chunk_key(&mut bytes, key);
        bytes.extend_from_slice(&record.schema_version().get().to_be_bytes());
        bytes.extend_from_slice(&record.dirty_section_mask().to_be_bytes());
        bytes.extend_from_slice(&record.updated_at_tick().to_be_bytes());
        append_len(&mut bytes, record.section_count());
        append_len(&mut bytes, record.block_entity_count());

        let mut reconstructed = Chunk::new(record.pos());
        record
            .apply_to_chunk(&mut reconstructed)
            .expect("scheduler-created overlay applies to its own chunk");
        for section_index in record.section_indices() {
            append_len(&mut bytes, section_index);
            let section = reconstructed
                .section(section_index)
                .expect("overlay section index is validated");
            for flat in 0..SECTION_VOLUME {
                bytes.extend_from_slice(
                    &section
                        .get(shadow_local_position(flat))
                        .as_u32()
                        .to_be_bytes(),
                );
            }
        }
        append_len(&mut bytes, reconstructed.block_entity_count());
        for (position, entity) in reconstructed.block_entities() {
            append_block_position(&mut bytes, position);
            append_exact_block_entity(&mut bytes, entity);
        }
        bytes
    }

    /// Builds exact canonical bytes for topology-neutral state and move-owned
    /// persistence produced by the completed tick.
    fn semantic_state_digest(
        scheduler: &ShardScheduler,
        outcome: &super::SchedulerTickOutcome,
        tick: Tick,
    ) -> ShadowStateDigest {
        let mut players = Vec::new();
        let mut chunks = BTreeMap::new();
        let mut mutations: BTreeMap<ShardId, Vec<Vec<u8>>> = BTreeMap::new();

        for scheduled in scheduler.shards.values() {
            assert!(
                scheduled.shard.is_tick_quiescent(),
                "a completed replay tick must drain every admitted operation"
            );
            assert!(
                !scheduled.shard.has_undo_history(),
                "the fixed replay intentionally creates no topology-local undo state"
            );
            players.extend(scheduled.shard.canonical_player_state_records());

            let loaded = scheduled.shard.loaded_chunks();
            for position in loaded.loaded_positions() {
                let key = ChunkKey::new(loaded.world(), loaded.dimension(), position);
                let chunk = loaded
                    .get(position)
                    .expect("loaded position remains resident");
                let tickets = loaded
                    .canonical_ticket_state_record(position)
                    .expect("loaded chunk has resident ticket state");
                let mut record = Vec::new();
                append_live_chunk(
                    &mut record,
                    key,
                    loaded.schema_version().get(),
                    chunk,
                    &tickets,
                );
                assert!(
                    chunks.insert(key, record).is_none(),
                    "a semantic chunk key has exactly one owner"
                );
            }

            for (position, record) in scheduled.shard.canonical_pending_mutation_records() {
                let logical_owner = logical_id(
                    loaded.world(),
                    loaded.dimension(),
                    position.to_chunk_pos().to_shard_pos(),
                );
                mutations.entry(logical_owner).or_default().push(record);
            }
        }

        players.sort_unstable();
        let mut overlays = BTreeMap::new();
        for batch in outcome.persist_batches() {
            for (key, record) in batch.records() {
                let canonical = canonical_overlay_record(*key, record);
                assert!(
                    overlays.insert(*key, canonical).is_none(),
                    "a completed tick emits at most one overlay per semantic chunk"
                );
            }
        }

        let mut canonical = Vec::new();
        canonical.extend_from_slice(&tick.get().to_be_bytes());
        append_len(&mut canonical, players.len());
        for player in players {
            canonical.push(0);
            append_len(&mut canonical, player.len());
            canonical.extend_from_slice(&player);
        }
        append_len(&mut canonical, chunks.len());
        for chunk in chunks.into_values() {
            canonical.push(1);
            append_len(&mut canonical, chunk.len());
            canonical.extend_from_slice(&chunk);
        }
        append_len(&mut canonical, mutations.len());
        for (logical_owner, records) in mutations {
            canonical.push(2);
            canonical.extend_from_slice(&logical_owner.world().get().to_be_bytes());
            canonical.extend_from_slice(&logical_owner.dimension().get().to_be_bytes());
            canonical.extend_from_slice(&logical_owner.position().x().to_be_bytes());
            canonical.extend_from_slice(&logical_owner.position().z().to_be_bytes());
            append_len(&mut canonical, records.len());
            for record in records {
                append_len(&mut canonical, record.len());
                canonical.extend_from_slice(&record);
            }
        }
        append_len(&mut canonical, overlays.len());
        for overlay in overlays.into_values() {
            canonical.push(3);
            append_len(&mut canonical, overlay.len());
            canonical.extend_from_slice(&overlay);
        }
        ShadowStateDigest(canonical)
    }

    #[allow(
        clippy::too_many_lines,
        reason = "keeping the fixed replay in one literal makes its exact tick schedule auditable"
    )]
    fn fixed_shadow_input_log() -> Vec<Vec<(ShardPos, GameInput)>> {
        let first = PlayerId::offline("shadow-digest-first");
        let second = PlayerId::offline("shadow-digest-second");
        let third = PlayerId::offline("shadow-digest-third");
        let west = ShardPos::new(-1, 0);
        let center = ShardPos::new(0, 0);
        let east = ShardPos::new(1, 0);
        vec![
            vec![
                (
                    east,
                    GameInput::PlayerJoin {
                        player: third,
                        position: Vec3::new(136.0, 70.0, 8.0),
                    },
                ),
                (
                    west,
                    GameInput::PlayerJoin {
                        player: first,
                        position: Vec3::new(-120.0, 64.0, 8.0),
                    },
                ),
                (
                    center,
                    GameInput::PlayerJoin {
                        player: second,
                        position: Vec3::new(8.0, 68.0, 8.0),
                    },
                ),
            ],
            vec![
                (
                    west,
                    GameInput::PlayerMove {
                        player: first,
                        position: Some(Vec3::new(-119.5, 64.5, 8.5)),
                        yaw: Some(45.0),
                        pitch: Some(-10.0),
                    },
                ),
                (
                    center,
                    GameInput::PlayerMove {
                        player: second,
                        position: None,
                        yaw: Some(180.0),
                        pitch: Some(15.0),
                    },
                ),
                (
                    east,
                    GameInput::SetGameMode {
                        player: third,
                        mode: GameMode::Creative,
                    },
                ),
            ],
            vec![
                (
                    center,
                    GameInput::PlayerMove {
                        player: second,
                        position: Some(Vec3::new(9.0, 68.0, 8.0)),
                        yaw: None,
                        pitch: None,
                    },
                ),
                (
                    west,
                    GameInput::SetGameMode {
                        player: first,
                        mode: GameMode::Adventure,
                    },
                ),
                (
                    east,
                    GameInput::PlayerMove {
                        player: third,
                        position: Some(Vec3::new(137.0, 70.5, 9.0)),
                        yaw: Some(-90.0),
                        pitch: None,
                    },
                ),
                (
                    west,
                    GameInput::SetBlockExact {
                        player: first,
                        position: BlockPos::new(-120, 65, 8),
                        sequence: 31,
                        state: BlockStateId::new(5),
                    },
                ),
                (
                    center,
                    GameInput::SetBlockExact {
                        player: second,
                        position: BlockPos::new(8, 68, 8),
                        sequence: 32,
                        state: BlockStateId::new(6),
                    },
                ),
                (
                    center,
                    GameInput::SetBlockExact {
                        player: second,
                        position: BlockPos::new(9, 68, 8),
                        sequence: 34,
                        state: BlockStateId::new(8),
                    },
                ),
                (
                    east,
                    GameInput::SetBlockExact {
                        player: third,
                        position: BlockPos::new(136, 70, 8),
                        sequence: 33,
                        state: BlockStateId::new(7),
                    },
                ),
            ],
            vec![
                (center, GameInput::PlayerLeave { player: second }),
                (
                    center,
                    GameInput::PlayerMove {
                        player: second,
                        position: Some(Vec3::new(99.0, 99.0, 99.0)),
                        yaw: Some(12.0),
                        pitch: Some(34.0),
                    },
                ),
                (
                    west,
                    GameInput::PlayerMove {
                        player: first,
                        position: None,
                        yaw: Some(90.0),
                        pitch: Some(0.0),
                    },
                ),
                (
                    east,
                    GameInput::SetGameMode {
                        player: third,
                        mode: GameMode::Spectator,
                    },
                ),
            ],
            vec![
                (
                    center,
                    GameInput::PlayerJoin {
                        player: second,
                        position: Vec3::new(10.0, 69.0, 8.0),
                    },
                ),
                (
                    west,
                    GameInput::PlayerMove {
                        player: first,
                        position: Some(Vec3::new(-118.0, 65.0, 9.0)),
                        yaw: None,
                        pitch: None,
                    },
                ),
                (east, GameInput::PlayerLeave { player: third }),
            ],
            Vec::new(),
        ]
    }

    #[test]
    fn shadow_item_digest_distinguishes_typed_and_opaque_components() {
        let item = ItemId::from_name("stone").expect("stone item exists");
        let count = std::num::NonZeroU8::new(1).expect("one is nonzero");
        let typed = ItemStack::new(
            item,
            count,
            ComponentPatch::new(vec![ComponentValue::MaxStackSize(1)], Vec::new()),
        );
        let opaque = ItemStack::new(
            item,
            count,
            ComponentPatch::new(
                vec![ComponentValue::Opaque(
                    OpaqueComponent::new(1, vec![1]).expect("bounded component"),
                )],
                Vec::new(),
            ),
        );
        let mut typed_bytes = Vec::new();
        append_exact_item_stack(&mut typed_bytes, &typed);
        let mut opaque_bytes = Vec::new();
        append_exact_item_stack(&mut opaque_bytes, &opaque);
        assert_ne!(typed, opaque);
        assert_ne!(typed_bytes, opaque_bytes);
    }

    #[tokio::test(start_paused = true)]
    async fn shadow_determinism_one_vs_n_shard_identical_digests() {
        let schedule = fixed_shadow_input_log();
        let mut harness = ShadowDeterminismHarness::new(
            [
                ShardPos::new(-1, 0),
                ShardPos::new(0, 0),
                ShardPos::new(1, 0),
            ],
            NonZeroUsize::new(2).expect("nonzero worker slots"),
        )
        .await
        .expect("determinism harness");

        let digests = harness
            .replay(&schedule, None)
            .expect("identical schedule must not diverge");
        assert_eq!(digests.len(), schedule.len());
        assert_eq!(
            digests.iter().map(|(tick, _)| *tick).collect::<Vec<_>>(),
            (1..=u64::try_from(schedule.len()).expect("fixed schedule fits u64"))
                .map(Tick::new)
                .collect::<Vec<_>>()
        );
    }

    #[tokio::test(start_paused = true)]
    async fn shadow_determinism_reports_the_first_deliberate_divergence() {
        let schedule = fixed_shadow_input_log();
        let second = PlayerId::offline("shadow-digest-second");
        let mut harness = ShadowDeterminismHarness::new(
            [
                ShardPos::new(-1, 0),
                ShardPos::new(0, 0),
                ShardPos::new(1, 0),
            ],
            NonZeroUsize::new(2).expect("nonzero worker slots"),
        )
        .await
        .expect("determinism harness");

        let divergence = (
            Tick::new(3),
            ShardPos::new(0, 0),
            GameInput::PlayerMove {
                player: second,
                position: None,
                yaw: Some(270.0),
                pitch: None,
            },
        );
        let error = harness
            .replay(&schedule, Some(&divergence))
            .expect_err("shadow-only state mutation must be detected");
        assert_eq!(error.tick(), Tick::new(3));
        assert_ne!(error.one_shard(), error.n_shard());
        assert!(error.to_string().contains("diverged at tick 3"));
        assert!(error.to_string().contains("one-shard="));
        assert!(error.to_string().contains("n-shard="));
    }
}
