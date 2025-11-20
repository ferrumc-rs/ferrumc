use bevy_ecs::prelude::Component;
use ferrumc_core::player::health::HealthData;
use std::ops::{Deref, DerefMut};

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Health(pub HealthData);

impl Health {
    pub fn new(current: f32, max: f32) -> Self {
        Self(HealthData::new(current, max))
    }
}

impl Deref for Health {
    type Target = HealthData;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Health {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
