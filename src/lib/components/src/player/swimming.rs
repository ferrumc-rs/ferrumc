use bevy_ecs::prelude::Component;

/// Component tracking whether a player is currently swimming
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SwimmingState {
    pub is_swimming: bool,
}
