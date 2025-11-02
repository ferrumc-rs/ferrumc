use bevy_ecs::prelude::*;
use ferrumc_entities::types::passive::pig_data::PigData;
use ferrumc_entities::GameEntity;
use ferrumc_state::GlobalStateResource;

/// System that ticks pig entities to update their AI/behavior
pub fn pig_tick_system(
    mut pigs: Query<&mut PigData>,
    state: Res<GlobalStateResource>,
    mut commands: Commands,
) {
    for mut pig_data in pigs.iter_mut() {
        pig_data.tick(&state.0, &mut commands);
    }
}
