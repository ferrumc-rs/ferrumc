use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerAbilitiesData {
    pub flying: bool,
    pub flying_speed: f32,
    pub instant_build: bool,
    pub invulnerable: bool,
    pub may_build: bool,
    pub may_fly: bool,
    pub walking_speed: f32,
}

impl Default for PlayerAbilitiesData {
    fn default() -> Self {
        Self {
            invulnerable: false,
            flying: false,
            may_fly: false,
            instant_build: false,
            may_build: true,
            flying_speed: 0.05,
            walking_speed: 0.1,
        }
    }
}
