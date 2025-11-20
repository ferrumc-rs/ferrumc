use bevy_ecs::prelude::Resource;
use std::time::Instant;

#[derive(Resource)]
pub struct WorldSyncTracker {
    pub last_synced: Instant,
}
