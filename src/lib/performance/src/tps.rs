use std::time::Duration;

use crate::{
    WINDOW_SECONDS,
    tick::{TickData, TickHistory},
};

pub struct TPSMonitor {
    targetted_tps: u32,
    history: TickHistory,
}

impl TPSMonitor {
    pub fn new(tps: u32) -> Self {
        Self {
            targetted_tps: tps,
            history: TickHistory::new(tps as usize * WINDOW_SECONDS),
        }
    }

    #[inline]
    pub fn record_tick(&mut self, tick: TickData) {
        self.history.record(tick);
    }

    /// Core TPS calculation (time-windowed)
    fn tps_window_ns(&self, window_ns: u128) -> f32 {
        let mut elapsed_ns: u128 = 0;
        let mut ticks: u64 = 0;

        for tick in self.history.iter_rev() {
            elapsed_ns += tick.duration_ns;
            ticks += 1;

            if elapsed_ns >= window_ns {
                break;
            }
        }

        if ticks == 0 || elapsed_ns == 0 {
            return self.targetted_tps as f32;
        }

        let tps = (ticks as f64) / (elapsed_ns as f64 / 1_000_000_000.0);
        tps.clamp(0.0, f64::from(self.targetted_tps)) as f32
    }

    fn collect_window_ns(&self, window_ns: u128) -> Vec<u128> {
        let mut total = 0u128;
        let mut out = Vec::with_capacity(32);

        for tick in self.history.iter_rev() {
            total += tick.duration_ns;
            out.push(tick.duration_ns);

            if total >= window_ns {
                break;
            }
        }

        out
    }

    fn percentile_ns(&self, mut samples: Vec<u128>, percentile: f64) -> Option<u128> {
        if samples.is_empty() {
            return None;
        }

        let len = samples.len();
        let rank = ((percentile * len as f64).ceil() as usize)
            .saturating_sub(1)
            .min(len - 1);

        let (_, value, _) = samples.select_nth_unstable(rank);
        Some(*value)
    }

    fn percentile_ms(&self, percentile: f64, window_ns: u128) -> Option<f64> {
        let samples = self.collect_window_ns(window_ns);
        let ns = self.percentile_ns(samples, percentile)?;
        Some(ns as f64 / 1_000_000.0)
    }

    /// Average tick duration over the last second (ms)
    pub fn avg_tick_ms(&self) -> f64 {
        let mut elapsed_ns = 0u128;
        let mut ticks = 0;

        for tick in self.history.iter_rev() {
            elapsed_ns += tick.duration_ns;
            ticks += 1;

            if elapsed_ns >= 1_000_000_000 {
                break;
            }
        }

        if ticks == 0 {
            return 0.0;
        }

        (elapsed_ns as f64 / f64::from(ticks)) / 1_000_000.0
    }

    pub fn tps(&self, duration: Duration) -> f32 {
        self.tps_window_ns(duration.as_nanos())
    }

    pub fn tick_duration(&self, percentile: f64) -> f64 {
        self.percentile_ms(percentile, 1_000_000_000).unwrap_or(0.0)
    }
}
