use bevy_ecs::component::Component;
use bevy_ecs::prelude::Resource;
use std::ops::Range;
use std::time::Instant;

#[derive(Resource, Debug, Default)]
pub struct WorldTime(u16);

#[derive(Component)]
pub struct LastSentTimeUpdate(Instant);

impl WorldTime {
    pub const MAX_TIME: u16 = 24000;

    pub const DAY: Range<u16> = 0..12000;
    pub const DUSK: Range<u16> = 12000..13000;
    pub const NIGHT: Range<u16> = 13000..23000;
    pub const DAWN: Range<u16> = 23000..24000;

    #[inline]
    pub fn advance_tick(&mut self) {
        self.0 = (self.0 + 1) % Self::MAX_TIME;
    }

    #[inline]
    pub fn current_time(&self) -> u16 {
        self.0
    }

    #[inline]
    pub fn set_time(&mut self, time: u16) {
        self.0 = time % Self::MAX_TIME;
    }

    pub fn set_time_to_start(&mut self, range: Range<u16>) {
        self.set_time(range.start);
    }

    pub fn set_time_to_middle(&mut self, range: Range<u16>) {
        let time = (range.end - range.start) / 2 + range.start;
        self.set_time(time);
    }
}

impl LastSentTimeUpdate {
    pub fn reset(&mut self) {
        self.0 = Instant::now();
    }

    pub fn should_resend(&self) -> bool {
        self.0.elapsed().as_secs() >= 5
    }
}

impl Default for LastSentTimeUpdate {
    fn default() -> Self {
        Self(Instant::now())
    }
}
