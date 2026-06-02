use bevy_ecs::prelude::Resource;

/// Monotonic game tick counter.
///
/// Incremented once per game tick (see the tick schedule in the binary's game loop).
/// Unlike [`crate::time::WorldTime`], which wraps every Minecraft day, this value never
/// resets for the lifetime of the server process. It is the authoritative clock for any
/// system that needs to schedule work a fixed number of ticks into the future, such as
/// fluid spreading or other scheduled block updates.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TickCounter(u64);

impl TickCounter {
    /// Creates a new counter starting at tick zero.
    pub const fn new() -> Self {
        Self(0)
    }

    /// Advances the counter by one tick. Called once per game tick.
    #[inline]
    pub fn advance(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }

    /// Returns the current tick number.
    #[inline]
    pub fn get(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_zero() {
        let counter = TickCounter::new();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn advance_increments() {
        let mut counter = TickCounter::new();
        counter.advance();
        counter.advance();
        assert_eq!(counter.get(), 2);
    }
}
