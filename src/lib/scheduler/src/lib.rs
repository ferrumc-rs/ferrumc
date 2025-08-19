use bevy_ecs::schedule::Schedule;
use once_cell::sync::Lazy;
use std::{
    borrow::Cow,
    cmp::Reverse,
    collections::BinaryHeap,
    sync::Mutex,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ScheduleId(pub usize);

pub type ScheduleBuilder = Box<dyn FnOnce(&mut Schedule) + Send + 'static>;

pub struct TimedSchedule {
    pub name: Cow<'static, str>,
    pub period: Duration,
    pub next_run: Instant,
    pub schedule: Schedule,
}

impl TimedSchedule {
    pub fn new<N, F>(name: N, period: Duration, build: F) -> Self
    where
        N: Into<Cow<'static, str>>,
        F: FnOnce(&mut Schedule),
    {
        let mut s = Schedule::default();
        build(&mut s);
        TimedSchedule {
            name: name.into(),
            period,
            next_run: Instant::now(), // run immediately first time
            schedule: s,
        }
    }
}

pub struct Scheduler {
    pub schedules: Vec<TimedSchedule>,
    queue: BinaryHeap<(Reverse<Instant>, usize)>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            schedules: Vec::new(),
            queue: BinaryHeap::new(),
        }
    }

    pub fn register(&mut self, schedule: TimedSchedule) -> ScheduleId {
        let id = ScheduleId(self.schedules.len());
        self.queue.push((Reverse(schedule.next_run), id.0));
        self.schedules.push(schedule);
        id
    }

    pub fn pop_next_due(&mut self) -> Option<(usize, Instant)> {
        self.queue
            .pop()
            .map(|(Reverse(instant), idx)| (idx, instant))
    }

    pub fn after_run(&mut self, idx: usize) {
        // Simple re-schedule policy: schedule again from "now"
        let now = Instant::now();
        let next = now + self.schedules[idx].period;
        self.schedules[idx].next_run = next;
        self.queue.push((Reverse(next), idx));
    }
}

/* ---------- Public registration hub for plugins ---------- */

pub struct PendingSchedule {
    pub name: Cow<'static, str>,
    pub period: Duration,
    pub builder: ScheduleBuilder,
}

static PENDING: Lazy<Mutex<Vec<PendingSchedule>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn register_timed_schedule<N, F>(name: N, period: Duration, builder: F)
where
    N: Into<Cow<'static, str>>,
    F: FnOnce(&mut Schedule) + Send + 'static,
{
    let mut guard = PENDING.lock().expect("pending schedules lock poisoned");
    guard.push(PendingSchedule {
        name: name.into(),
        period,
        builder: Box::new(builder),
    });
}

pub fn drain_registered_schedules() -> Vec<PendingSchedule> {
    let mut guard = PENDING.lock().expect("pending schedules lock poisoned");
    let drained = guard.drain(..).collect::<Vec<_>>();
    drained
}
