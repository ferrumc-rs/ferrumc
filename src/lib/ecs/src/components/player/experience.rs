use bevy_ecs::prelude::Component;
use ferrumc_core::player::experience::ExperienceData;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Experience(pub ExperienceData);
