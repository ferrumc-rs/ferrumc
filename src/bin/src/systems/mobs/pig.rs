use bevy_ecs::prelude::{Query, With};
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::markers::entity_types::Pig;

#[expect(dead_code, unused_variables)]
pub fn tick_pig(query: Query<(&Position, &Velocity), With<Pig>>) {
    // Pig AI logic would go here
}
