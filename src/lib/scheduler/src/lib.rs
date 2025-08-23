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
#[derive(Clone, Copy, Debug)]
pub enum MissedTickBehavior {
    Burst,
    Skip,
    Delay,
}

/// A Bevy schedule that runs periodically with a given period and behavior.
pub struct TimedSchedule {
    pub name: String,
    pub period: Duration,
    pub schedule: Schedule,
    behavior: MissedTickBehavior,
    phase: Duration,
    max_catch_up: usize,
    next_due: Instant,
    burst_count: usize,
}

impl TimedSchedule {
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
            next_due: Instant::now(),
            burst_count: 0,
        }
    }

    pub fn with_behavior(mut self, behavior: MissedTickBehavior) -> Self {
        self.behavior = behavior;
        self
    }

    pub fn with_phase(mut self, phase: Duration) -> Self {
        self.phase = phase;
        self
    }

    pub fn with_max_catch_up(mut self, n: usize) -> Self {
        self.max_catch_up = n.max(1);
        self
    }

    fn init_deadlines(&mut self, now: Instant) {
        self.next_due = now + self.phase;
        self.burst_count = 0;
    }

    fn reschedule_after_run(&mut self, now: Instant) {
        match self.behavior {
            MissedTickBehavior::Delay => {
                self.next_due = now + self.period;
                self.burst_count = 0;
            }
            MissedTickBehavior::Burst => {
                self.next_due += self.period;
                if self.next_due <= now {
                    if self.burst_count + 1 < self.max_catch_up {
                        self.burst_count += 1;
                    } else {
                        while self.next_due <= now {
                            self.next_due += self.period;
                        }
                        self.burst_count = 0;
                    }
                } else {
                    self.burst_count = 0;
                }
            }
            MissedTickBehavior::Skip => {
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

    pub fn next_due(&self) -> Instant {
        self.next_due
    }
}

/// The main timed scheduler.
pub struct Scheduler {
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

    pub fn register(&mut self, mut schedule: TimedSchedule) {
        let now = Instant::now();
        schedule.init_deadlines(now);

        let idx = self.schedules.len();
        let next = schedule.next_due;

        self.schedules.push(schedule);
        self.heap.push(HeapEntry { when: next, idx });
    }

    pub fn pop_next_due(&mut self) -> Option<(usize, Instant)> {
        self.heap.pop().map(|e| (e.idx, e.when))
    }

    pub fn after_run(&mut self, idx: usize) {
        let now = Instant::now();
        let sched = &mut self.schedules[idx];
        sched.reschedule_after_run(now);

        let when = sched.next_due;
        self.heap.push(HeapEntry { when, idx });
    }

    pub fn peek_next_due(&self) -> Option<(usize, Instant)> {
        self.heap.peek().map(|e| (e.idx, e.when))
    }

    pub fn time_until_next_due(&self) -> Option<Duration> {
        self.peek_next_due()
            .map(|(_, when)| when.saturating_duration_since(Instant::now()))
    }

    /// ✅ New: Park the current thread until the next schedule is due.
    pub fn park_until_next_due(&self) {
        match self.time_until_next_due() {
            Some(dur) if dur > Duration::ZERO => std::thread::park_timeout(dur),
            Some(_) => {
                // Due now; return immediately
            }
            None => {
                // No schedules registered; avoid spinning
                std::thread::park_timeout(Duration::from_millis(1));
            }
        }
    }
}

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
        other
            .when
            .cmp(&self.when)
            .then_with(|| other.idx.cmp(&self.idx))
    }
}

pub struct PendingSchedule {
    pub name: String,
    pub period: Duration,
    pub builder: Box<dyn FnMut(&mut Schedule) + Send>,
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

static REGISTRY: OnceLock<Mutex<Vec<PendingSchedule>>> = OnceLock::new();

fn registry() -> &'static Mutex<Vec<PendingSchedule>> {
    REGISTRY.get_or_init(|| Mutex::new(Vec::new()))
}

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

pub fn drain_registered_schedules() -> Vec<PendingSchedule> {
    let mut reg = registry().lock().expect("registry poisoned");
    std::mem::take(&mut *reg)
}
