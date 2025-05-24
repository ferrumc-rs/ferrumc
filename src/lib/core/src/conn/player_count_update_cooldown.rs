use bevy_ecs::prelude::Resource;

#[derive(Resource)]
pub struct PlayerCountUpdateCooldown {
    pub last_update: std::time::Instant,
}
