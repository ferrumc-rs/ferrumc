use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct TeleportTracker {
    pub waiting_for_confirm: bool,
}
