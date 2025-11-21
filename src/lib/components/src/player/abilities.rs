use super::gamemode::GameMode;
use bevy_ecs::prelude::Component;
use ferrumc_config::server_config::get_global_config;

#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerAbilities {
    pub invulnerable: bool,
    pub flying: bool,
    pub may_fly: bool,
    pub creative_mode: bool,
    pub may_build: bool,
    pub flying_speed: f32,
    pub walking_speed: f32,
}

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
    }
}
