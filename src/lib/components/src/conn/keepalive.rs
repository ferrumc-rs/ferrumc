use bevy_ecs::prelude::Component;
use std::time::Instant;

#[derive(Component)]
pub struct KeepAliveTracker {
    pub last_sent_keep_alive: i64,
    pub last_received_keep_alive: Instant,
    pub has_received_keep_alive: bool,
}
