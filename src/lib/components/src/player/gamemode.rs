use bevy_ecs::prelude::Component;
use ferrumc_core::player::gamemode::GameMode;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct GameModeComponent(pub GameMode);
