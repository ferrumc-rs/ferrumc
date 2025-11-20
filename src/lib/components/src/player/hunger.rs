use bevy_ecs::prelude::Component;
use ferrumc_core::player::hunger::HungerData;
use std::ops::{Deref, DerefMut};

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Hunger(pub HungerData);

impl Hunger {
    pub fn new(level: u8, saturation: f32, exhaustion: f32) -> Self {
        Self(HungerData::new(level, saturation, exhaustion))
    }
}

impl Deref for Hunger {
    type Target = HungerData;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Hunger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
