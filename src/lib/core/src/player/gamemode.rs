use crate::player::abilities::PlayerAbilitiesData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum GameMode {
    #[default]
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

impl GameMode {
    /// Updates a 'PlayerAbilities' component to match the rules of a gamemode
    pub fn update_abilities(&self, abilities: &mut PlayerAbilitiesData) {
        match self {
            GameMode::Survival | GameMode::Adventure => {
                abilities.may_fly = false;
                abilities.flying = false;
                abilities.invulnerable = false;
                abilities.instant_build = false;
            }
            GameMode::Creative => {
                abilities.may_fly = true;
                abilities.invulnerable = true;
                abilities.instant_build = true;
                // We don't force `flying = true`, we just allow it.
            }
            GameMode::Spectator => {
                abilities.may_fly = true;
                abilities.flying = true; // Spectators are always flying
                abilities.invulnerable = true;
                abilities.instant_build = false;
            }
        }
    }
}
