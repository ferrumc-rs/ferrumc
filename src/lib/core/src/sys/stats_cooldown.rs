use std::time::Instant;
use bevy_ecs::prelude::Resource;

#[derive(Resource)]
pub struct StatsCooldown {
    pub last_update: Instant
}