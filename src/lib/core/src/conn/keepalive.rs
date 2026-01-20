use bevy_ecs::prelude::Component;
use std::time::Instant;

#[derive(Component)]
pub struct KeepAliveTracker {
    pub last_sent_keep_alive_id: i64,
    pub last_received_keep_alive: Instant,
    pub last_sent_keep_alive: Instant,
    pub has_received_keep_alive: bool,
}

impl KeepAliveTracker {
    pub fn ping(&self) -> i64 {
        self.last_received_keep_alive
            .duration_since(self.last_sent_keep_alive)
            .as_millis() as i64
    }
}
