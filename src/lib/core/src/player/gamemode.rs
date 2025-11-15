use bevy_ecs::prelude::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum GameMode {
    #[default]
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

/// The component storing a player's current gamemode.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct GameModeComponent(pub GameMode);
