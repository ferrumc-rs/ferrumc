use crate::player::abilities::PlayerAbilities;
use bevy_ecs::prelude::Component;
use ferrumc_core::player::gamemode::GameMode;
use std::ops::{Deref, DerefMut};

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct GameModeComponent(pub GameMode);

// 1. Deref Magic: Allows you to access GameMode variants directly
impl Deref for GameModeComponent {
    type Target = GameMode;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GameModeComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// 2. Helper Method: Bridges the Component gap
impl GameModeComponent {
    pub fn update_abilities(&self, abilities: &mut PlayerAbilities) {
        // We call the core logic on the inner data types
        self.0.update_abilities(&mut abilities.0);
    }
}
