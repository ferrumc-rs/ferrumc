use bevy_ecs::prelude::{Query, Res, With};
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::markers::HasWaterDrag;
use ferrumc_state::GlobalStateResource;

pub fn handle(
    mut query: Query<(&mut Velocity, &mut Position), With<HasWaterDrag>>,
    state: Res<GlobalStateResource>,
) {
    for (mut vel, mut pos) in query.iter_mut() {
        // if state.0.world
        let drag_coefficient = 0.8; // Example drag coefficient for water
        let drag_force = **vel * drag_coefficient;
        // Apply drag force to the entity's velocity
        // This is a placeholder; actual implementation would modify the entity's velocity component
        **vel -= drag_force;
    }
}
