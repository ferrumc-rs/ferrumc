//! World time tracking resource.
//!
//! This module provides the `WorldTime` resource that tracks both the total
//! world age and current time of day for the server's day/night cycle.

use bevy_ecs::prelude::Resource;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};

/// Resource that tracks the world's time state.
///
/// This is a global resource shared across all systems. The time values use
/// atomic operations to allow safe concurrent access without requiring
/// mutable borrows.
///
/// # Time System
///
/// - `world_age`: Total ticks since world creation, always increases
/// - `time_of_day`: Current position in the day/night cycle (0-23,999)
///
/// # Usage
///
/// ```ignore
/// fn my_system(time: Res<WorldTime>) {
///     let current_day = time.time_of_day() / 24_000;
///     if time.is_night() {
///         // Spawn hostile mobs...
///     }
/// }
/// ```
#[derive(Resource)]
pub struct WorldTime {
    /// Total world age in ticks (always increasing).
    world_age: AtomicI64,

    /// Current time of day in ticks (0-23,999).
    time_of_day: AtomicI64,

    /// Whether the day/night cycle is active.
    ///
    /// When false, `time_of_day` doesn't advance automatically.
    /// Corresponds to the `doDaylightCycle` gamerule.
    daylight_cycle_enabled: AtomicBool,
}

impl Default for WorldTime {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldTime {
    /// Creates a new `WorldTime` starting at sunrise (tick 0).
    #[must_use]
    pub fn new() -> Self {
        Self {
            world_age: AtomicI64::new(0),
            time_of_day: AtomicI64::new(0),
            daylight_cycle_enabled: AtomicBool::new(true),
        }
    }

    /// Creates a `WorldTime` with specific initial values.
    #[must_use]
    pub fn with_time(world_age: i64, time_of_day: i64) -> Self {
        Self {
            world_age: AtomicI64::new(world_age),
            // 24,000 = ticks per full day cycle
            time_of_day: AtomicI64::new(time_of_day % 24_000),
            daylight_cycle_enabled: AtomicBool::new(true),
        }
    }

    // ========================================================================
    // Getters
    // ========================================================================

    /// Returns the total world age in ticks.
    #[inline]
    pub fn world_age(&self) -> i64 {
        self.world_age.load(Ordering::Relaxed)
    }

    /// Returns the current time of day (0-23,999).
    #[inline]
    pub fn time_of_day(&self) -> i64 {
        self.time_of_day.load(Ordering::Relaxed)
    }

    /// Returns whether the daylight cycle is enabled.
    #[inline]
    pub fn is_daylight_cycle_enabled(&self) -> bool {
        self.daylight_cycle_enabled.load(Ordering::Relaxed)
    }

    // ========================================================================
    // Time Queries
    // ========================================================================

    /// Returns `true` if it's currently daytime (sunrise to sunset).
    ///
    /// Daytime is defined as time_of_day in [0, 12,000).
    #[inline]
    pub fn is_day(&self) -> bool {
        self.time_of_day() < 12_000
    }

    /// Returns `true` if it's currently nighttime (sunset to sunrise).
    ///
    /// Nighttime is defined as time_of_day in [12,000, 24,000).
    #[inline]
    pub fn is_night(&self) -> bool {
        !self.is_day()
    }

    /// Returns the current Minecraft day number (starting from 0).
    #[inline]
    pub fn day_number(&self) -> i64 {
        self.world_age() / 24_000
    }

    // ========================================================================
    // Modifiers
    // ========================================================================

    /// Advances the world time by the specified number of ticks.
    ///
    /// This increments `world_age` unconditionally, and increments `time_of_day`
    /// only if `daylight_cycle_enabled` is true (wrapping at 24,000).
    pub fn tick(&self, ticks: i64) {
        self.world_age.fetch_add(ticks, Ordering::Relaxed);

        if self.is_daylight_cycle_enabled() {
            let old = self.time_of_day.fetch_add(ticks, Ordering::Relaxed);
            // Wrap around if we exceeded 24,000
            if old + ticks >= 24_000 {
                self.time_of_day
                    .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                        Some(v % 24_000)
                    })
                    .ok();
            }
        }
    }

    /// Sets the time of day directly (e.g., from `/time set` command).
    ///
    /// The value is automatically wrapped to the 0-23,999 range.
    pub fn set_time_of_day(&self, time: i64) {
        let wrapped = ((time % 24_000) + 24_000) % 24_000; // Handle negative values
        self.time_of_day.store(wrapped, Ordering::Relaxed);
    }

    /// Enables or disables the daylight cycle.
    ///
    /// When disabled, `time_of_day` won't advance during `tick()`.
    pub fn set_daylight_cycle_enabled(&self, enabled: bool) {
        self.daylight_cycle_enabled.store(enabled, Ordering::Relaxed);
    }

    /// Adds time to the current time of day (e.g., from `/time add` command).
    ///
    /// The result is wrapped to stay within 0-23,999.
    pub fn add_time(&self, ticks: i64) {
        self.time_of_day
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                Some(((v + ticks) % 24_000 + 24_000) % 24_000)
            })
            .ok();
    }
}
