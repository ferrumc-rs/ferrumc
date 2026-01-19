use bevy_ecs::prelude::{Query, With};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::markers::entity_types::Pig;

#[expect(dead_code, unused_variables)]
pub fn tick_pig(
    query: Query<&Position, With<Pig>>,
    players: Query<&Position, With<PlayerIdentity>>,
) {
}
