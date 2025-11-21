use crate::player::abilities::PlayerAbilities;
use bevy_ecs::prelude::Component;
use ferrumc_config::server_config::get_global_config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GameMode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

impl Default for GameMode {
    fn default() -> Self {
        match get_global_config().default_gamemode.to_lowercase().as_str() {
            "survival" => GameMode::Survival,
            "creative" => GameMode::Creative,
            "adventure" => GameMode::Adventure,
            "spectator" => GameMode::Spectator,
            _ => GameMode::Survival,
        }
    }
}

impl GameMode {
    /// Updates a 'PlayerAbilities' component to match the rules of a gamemode
    pub fn update_abilities(&self, abilities: &mut PlayerAbilities) {
        match self {
            GameMode::Survival | GameMode::Adventure => {
                abilities.may_fly = false;
                abilities.flying = false;
                abilities.invulnerable = false;
                abilities.creative_mode = false;
            }
            GameMode::Creative => {
                abilities.may_fly = true;
                abilities.invulnerable = true;
                abilities.creative_mode = true;
                // We don't force `flying = true`, we just allow it.
            }
            GameMode::Spectator => {
                abilities.may_fly = true;
                abilities.flying = true; // Spectators are always flying
                abilities.invulnerable = true;
                abilities.creative_mode = false;
            }
        }
    }
}

/// The component storing a player's current gamemode.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct GameModeComponent(pub GameMode);
