use bevy_ecs::prelude::*;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::collision::is_solid_block;
use ferrumc_entities::EntityType;
use ferrumc_state::GlobalStateResource;

/// System that checks if entities are on the ground
/// Updates the OnGround component based on the block below the entity
pub fn ground_check_system(
    mut query: Query<(&Position, &mut OnGround), With<EntityType>>,
    state: Res<GlobalStateResource>,
) {
    for (pos, mut on_ground) in query.iter_mut() {
        let block_x = pos.x.floor() as i32;
        let block_y = (pos.y - 0.1).floor() as i32; // Slightly below feet
        let block_z = pos.z.floor() as i32;

        // Use shared collision helper
        // TODO: Check for specific non-solid blocks (water, lava, tall grass, etc.)
        on_ground.0 = is_solid_block(&state.0, block_x, block_y, block_z);
    }
}
