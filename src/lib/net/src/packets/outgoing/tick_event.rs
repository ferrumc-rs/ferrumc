use ferrumc_macros::Event;

#[derive(Event, Clone, Copy)]
pub struct TickEvent {
    pub tick: i64,
}

impl TickEvent {
    pub fn new(tick: i64) -> Self {
        Self { tick }
    }
}

