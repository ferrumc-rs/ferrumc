use bevy_ecs::prelude::Component;
use ferrumc_core::player::abilities::PlayerAbilitiesData;
use std::ops::{Deref, DerefMut};

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PlayerAbilities(pub PlayerAbilitiesData);

impl Deref for PlayerAbilities {
    type Target = PlayerAbilitiesData;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PlayerAbilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
