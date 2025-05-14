use bevy_ecs::prelude::Component;

#[derive(Component, Default)]
pub struct KeepAliveTracker {
    pub last_sent_keep_alive: i64,
}