use super::gamemode::GameMode;
use bevy_ecs::prelude::Component;
<<<<<<< HEAD
use ferrumc_config::server_config::get_global_config;
=======
use ferrumc_core::player::abilities::PlayerAbilitiesData;
use std::ops::{Deref, DerefMut};
>>>>>>> origin/master

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PlayerAbilities(pub PlayerAbilitiesData);

<<<<<<< HEAD
impl Default for PlayerAbilities {
    fn default() -> Self {
        let default_gamemode = match get_global_config().default_gamemode.to_lowercase().as_str() {
            "survival" => GameMode::Survival,
            "creative" => GameMode::Creative,
            "adventure" => GameMode::Adventure,
            "spectator" => GameMode::Spectator,
            _ => GameMode::Survival,
        };
        Self::for_game_mode(default_gamemode)
    }
}
impl PlayerAbilities {
    pub fn for_game_mode(game_mode: GameMode) -> Self {
        match game_mode {
            GameMode::Survival => Self {
                invulnerable: false,
                flying: false,
                may_fly: false,
                creative_mode: false,
                may_build: true,
                flying_speed: 0.05,
                walking_speed: 0.1,
            },
            GameMode::Creative => Self {
                invulnerable: true,
                flying: false,
                may_fly: true,
                creative_mode: true,
                may_build: true,
                flying_speed: 0.05,
                walking_speed: 0.1,
            },
            GameMode::Adventure => Self {
                invulnerable: false,
                flying: false,
                may_fly: false,
                creative_mode: false,
                may_build: false,
                flying_speed: 0.05,
                walking_speed: 0.1,
            },
            GameMode::Spectator => Self {
                invulnerable: true,
                flying: true,
                may_fly: true,
                creative_mode: false,
                may_build: false,
                flying_speed: 0.05,
                walking_speed: 0.1,
            },
        }
=======
impl Deref for PlayerAbilities {
    type Target = PlayerAbilitiesData;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PlayerAbilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
>>>>>>> origin/master
    }
}
