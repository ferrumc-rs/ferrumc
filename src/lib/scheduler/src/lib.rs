//! ferrumc_scheduler
//!
//! A small timed scheduler for Bevy `Schedule`s with tokio-like missed-tick
//! behavior options. Designed to work with the existing game loop style used
//! by FerrumMC.
//!
//! Features:
//! - Fixed-rate periodic schedules
//! - Missed tick behavior (Burst, Skip, Delay), similar to tokio::time::interval
//! - Optional phase offsets and per-schedule max catch-up cap (used by Burst)
//! - Simple plugin registry (register schedules from plugins and drain on startup)
//!
//! Compatibility:
//! - Exposes `Scheduler`, `TimedSchedule`, and `drain_registered_schedules`
//! - Keeps the `pop_next_due()` and `after_run()` API expected by the current
//!   game loop
//! - `TimedSchedule::new` is backward compatible with existing usage

use bevy_ecs::schedule::Schedule;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

/// Controls what happens when a schedule runs late (misses one or more ticks).
///
/// - Burst (default): run missed ticks back-to-back until caught up
///   (bounded by `max_catch_up`).
/// - Skip: drop missed ticks and align to the next future boundary
///   (fixed-rate, drift-free).
/// - Delay: schedule next run at `now + period` (fixed-delay, allows drift).
#[derive(Clone, Copy, Debug)]
pub enum MissedTickBehavior {
    Burst,
    Skip,
    Delay,
}

/// A Bevy schedule that runs periodically with a given period and behavior.
pub struct TimedSchedule {
    /// Human-readable schedule name (used for logging/telemetry).
    pub name: String,

    /// How often this schedule should run.
    pub period: Duration,

    /// The Bevy schedule to execute.
    pub schedule: Schedule,

    // Behavior when the schedule runs late.
    behavior: MissedTickBehavior,

    // Start offset relative to "now" at registration time.
    phase: Duration,

    // Max consecutive catch-ups in Burst mode to avoid starvation.
    max_catch_up: usize,

    // Internal timing state.
    next_due: Instant,

    // Count of consecutive back-to-back runs in Burst mode.
    burst_count: usize,
}

impl TimedSchedule {
    /// Create a new timed schedule.
    ///
    /// Notes:
    /// - The first due time is initialized at registration time to `now + phase`
    ///   (defaults to immediate if `phase` is zero).
    /// - Default missed-tick behavior is `Burst`, similar to tokio::interval.
    pub fn new<N, B>(name: N, period: Duration, mut build: B) -> Self
    where
        N: Into<String>,
        B: FnMut(&mut Schedule),
    {
        let mut schedule = Schedule::default();
        build(&mut schedule);

        Self {
            name: name.into(),
            period,
            schedule,
            behavior: MissedTickBehavior::Burst,
            phase: Duration::ZERO,
            max_catch_up: 8,
            // Will be set properly during Scheduler::register().
            next_due: Instant::now(),
            burst_count: 0,
        }
    }

    /// Set how missed ticks are handled.
    pub fn with_behavior(mut self, behavior: MissedTickBehavior) -> Self {
        self.behavior = behavior;
        self
    }

    /// Set an initial phase offset from registration time.
    pub fn with_phase(mut self, phase: Duration) -> Self {
        self.phase = phase;
        self
    }

    /// Set max consecutive catch-ups for Burst behavior.
    pub fn with_max_catch_up(mut self, n: usize) -> Self {
        self.max_catch_up = n.max(1);
        self
    }

    fn init_deadlines(&mut self, now: Instant) {
        self.next_due = now + self.phase;
        self.burst_count = 0;
    }

    /// Compute the next due time after a run, using the missed-tick behavior.
    fn reschedule_after_run(&mut self, now: Instant) {
        match self.behavior {
            MissedTickBehavior::Delay => {
                // Fixed-delay: next run is based on completion time.
                self.next_due = now + self.period;
                self.burst_count = 0;
            }
            MissedTickBehavior::Burst => {
                // Step forward one period (anchored to previous due).
                self.next_due += self.period;

                // If we're still behind, burst up to max_catch_up times.
                if self.next_due <= now {
                    if self.burst_count + 1 < self.max_catch_up {
                        // We'll allow another immediate "burst" run on the next
                        // loop iteration.
                        self.burst_count += 1;
                    } else {
                        // Too many bursts; skip ahead to the next future boundary.
                        while self.next_due <= now {
                            self.next_due += self.period;
                        }
                        self.burst_count = 0;
                    }
                } else {
                    // We're back on schedule.
                    self.burst_count = 0;
                }
            }
            MissedTickBehavior::Skip => {
                // Fixed-rate: jump to next future boundary (drift-free).
                self.next_due += self.period;
                if self.next_due <= now {
                    while self.next_due <= now {
                        self.next_due += self.period;
                    }
                }
                self.burst_count = 0;
            }
        }
    }

    /// Returns the next time this schedule is due (for introspection).
    pub fn next_due(&self) -> Instant {
        self.next_due
    }
}

/// The main timed scheduler. Holds multiple `TimedSchedule`s and
/// returns whichever one is due next.
pub struct Scheduler {
    /// All registered schedules (index is stable).
    pub schedules: Vec<TimedSchedule>,
    heap: BinaryHeap<HeapEntry>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            schedules: Vec::new(),
            heap: BinaryHeap::new(),
        }
    }

    /// Register a new `TimedSchedule`. The first due time is initialized to
    /// `now + phase`.
    pub fn register(&mut self, mut schedule: TimedSchedule) {
        let now = Instant::now();
        schedule.init_deadlines(now);

        let idx = self.schedules.len();
        let next = schedule.next_due;

        self.schedules.push(schedule);
        self.heap.push(HeapEntry { when: next, idx });
    }

    /// Returns the next due schedule (index and due time) and removes it from
    /// the internal queue. Call `after_run(idx)` once you've executed it to
    /// reschedule it.
    pub fn pop_next_due(&mut self) -> Option<(usize, Instant)> {
        self.heap.pop().map(|e| (e.idx, e.when))
    }

    /// Reschedule the schedule after it has run.
    pub fn after_run(&mut self, idx: usize) {
        let now = Instant::now();
        let sched = &mut self.schedules[idx];
        sched.reschedule_after_run(now);

        let when = sched.next_due;
        self.heap.push(HeapEntry { when, idx });
    }

    /// Peek at the next due schedule without removing it.
    pub fn peek_next_due(&self) -> Option<(usize, Instant)> {
        self.heap.peek().map(|e| (e.idx, e.when))
    }

    /// How long until the next schedule is due.
    pub fn time_until_next_due(&self) -> Option<Duration> {
        self.peek_next_due()
            .map(|(_, when)| when.saturating_duration_since(Instant::now()))
    }
}

/// Heap entry for the min-heap (implemented via BinaryHeap by inverting order).
#[derive(Clone, Copy, Debug)]
struct HeapEntry {
    when: Instant,
    idx: usize,
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.when == other.when && self.idx == other.idx
    }
}
impl Eq for HeapEntry {}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the ordering so the smallest Instant (earliest due) is popped first.
        other
            .when
            .cmp(&self.when)
            .then_with(|| other.idx.cmp(&self.idx))
    }
}

/// Pending schedule registration for plugins.
/// Game startup can drain these and register them into the Scheduler.
pub struct PendingSchedule {
    pub name: String,
    pub period: Duration,
    pub builder: Box<dyn FnMut(&mut Schedule) + Send>,
    // Optional extras with defaults (not used by the current integration,
    // but available to plugin authors if you expose advanced APIs).
    pub behavior: MissedTickBehavior,
    pub phase: Duration,
    pub max_catch_up: usize,
}

impl PendingSchedule {
    pub fn into_timed(self) -> TimedSchedule {
        TimedSchedule::new(self.name, self.period, self.builder)
            .with_behavior(self.behavior)
            .with_phase(self.phase)
            .with_max_catch_up(self.max_catch_up)
    }
}

// Simple global registry to collect schedules from plugins before startup.
static REGISTRY: OnceLock<Mutex<Vec<PendingSchedule>>> = OnceLock::new();

fn registry() -> &'static Mutex<Vec<PendingSchedule>> {
    REGISTRY.get_or_init(|| Mutex::new(Vec::new()))
}

/// Register a schedule from a plugin (basic).
pub fn register_schedule<N, B>(name: N, period: Duration, builder: B)
where
    N: Into<String>,
    B: FnMut(&mut Schedule) + Send + 'static,
{
    let mut reg = registry().lock().expect("registry poisoned");
    reg.push(PendingSchedule {
        name: name.into(),
        period,
        builder: Box::new(builder),
        behavior: MissedTickBehavior::Burst,
        phase: Duration::ZERO,
        max_catch_up: 8,
    });
}

/// Register a schedule from a plugin (advanced), with behavior/phase options.
pub fn register_schedule_advanced<N, B>(
    name: N,
    period: Duration,
    behavior: MissedTickBehavior,
    phase: Duration,
    max_catch_up: usize,
    builder: B,
) where
    N: Into<String>,
    B: FnMut(&mut Schedule) + Send + 'static,
{
    let mut reg = registry().lock().expect("registry poisoned");
    reg.push(PendingSchedule {
        name: name.into(),
        period,
        builder: Box::new(builder),
        behavior,
        phase,
        max_catch_up: max_catch_up.max(1),
    });
}

/// Drain all schedules registered by plugins at startup.
pub fn drain_registered_schedules() -> Vec<PendingSchedule> {
    let mut reg = registry().lock().expect("registry poisoned");
    std::mem::take(&mut *reg)
}
