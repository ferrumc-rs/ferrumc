use bevy_ecs::prelude::{Query, Res, With};
use bevy_math::{IVec3, Vec3A};
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::markers::HasWaterDrag;
use ferrumc_entities::PhysicalProperties;
use ferrumc_macros::match_block;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};
use tracing::debug;

pub fn handle(
    mut query: Query<(&mut Velocity, &Position, &PhysicalProperties), With<HasWaterDrag>>,
    state: Res<GlobalStateResource>,
) {
    for (mut vel, pos, physical) in query.iter_mut() {
        let chunk_pos = ChunkPos::from(pos.coords);
        let chunk = ferrumc_utils::world::load_or_generate_mut(&state.0, chunk_pos, "overworld")
            .expect("Failed to load or generate chunk");

        // Check if the entity's center (middle of body) is in water
        // This makes entities float with their body half-submerged instead of feet out of water
        let feet_pos = pos.coords.as_ivec3();
        let entity_height = physical.bounding_box.height();
        let center_y = pos.coords.y + (entity_height / 2.0);
        let center_pos = IVec3::new(feet_pos.x, center_y as i32, feet_pos.z);

        let is_center_in_water =
            match_block!("water", chunk.get_block(ChunkBlockPos::from(center_pos)));

        if is_center_in_water {
            // Water drag for living entities (not items!)
            // From LivingEntity.travelInWater(): multiply(slowDown, 0.8, slowDown)
            // slowDown = 0.8 for normal water movement (0.9 if sprinting)
            **vel *= Vec3A::splat(0.8);

            // Buoyancy force - makes entities float up to the surface
            // From LivingEntity.floatInWaterWhileRidden(): add(0.0, 0.04, 0.0)
            // This is much stronger than item buoyancy (0.0005) and makes entities rise to surface
            const BUOYANCY_FORCE: f32 = 0.04;
            vel.y += BUOYANCY_FORCE;

            debug!(
                "Water physics - center_y: {:.2}, vel.y after drag+buoyancy: {:.4}",
                center_y, vel.y
            );
        }
    }
}
