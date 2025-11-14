use bevy_ecs::prelude::Component;

#[derive(Component, Debug)]
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
    // Default abilities for players
    fn default() -> Self {
        Self {
            invulnerable: false,
            flying: false,
            may_fly: false,
            creative_mode: false,
            may_build: true,
            flying_speed: 0.05,
            walking_speed: 0.1,
        }
    }
}
