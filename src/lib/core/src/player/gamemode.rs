use bevy_ecs::prelude::Component;

use crate::player::abilities::PlayerAbilities;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum GameMode {
    #[default]
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
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
