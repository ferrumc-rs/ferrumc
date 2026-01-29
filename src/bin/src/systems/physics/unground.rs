use bevy_ecs::message::MessageReader;
use bevy_ecs::prelude::{Query, Res, With};
use bevy_math::IVec3;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::components::{Baby, EntityMetadata, PhysicalRegistry};
use ferrumc_entities::markers::HasCollisions;
use ferrumc_messages::BlockBrokenEvent;
use ferrumc_state::GlobalStateResource;
use tracing::debug;

use super::collisions::is_solid_block;

/// System that ungrounds entities when blocks are broken beneath them.
/// This runs only when BlockBrokenEvent messages are received, avoiding
/// the performance cost of checking every entity every tick.
/// The main purpose is to re-enable gravity for entities that lose their ground support.
pub fn handle(
    mut events: MessageReader<BlockBrokenEvent>,
    mut entities: Query<
        (&Position, &EntityMetadata, Option<&Baby>, &mut OnGround),
        With<HasCollisions>,
    >,
    state: Res<GlobalStateResource>,
    registry: Res<PhysicalRegistry>,
) {
    for event in events.read() {
        let broken_block_pos = event.position;
        debug!(
            "Block broken at {:?}, checking entities for un-grounding",
            broken_block_pos.pos
        );

        // Check all entities with collisions
        for (pos, metadata, baby, mut grounded) in entities.iter_mut() {
            // Skip entities that aren't grounded
            if !grounded.0 {
                continue;
            }

            // Get physical properties from registry
            let is_baby = baby.is_some();
            let Some(physical) = registry.get(metadata.protocol_id(), is_baby) else {
                continue;
            };

            // Calculate entity's feet position
            let entity_pos = pos.coords.as_vec3a();
            let feet_min = physical.bounding_box.min + entity_pos;

            // Check if the broken block was supporting this entity
            // We check blocks just below the entity's feet
            let feet_y = (feet_min.y - 0.01).floor() as i32;

            // Get the horizontal range of blocks the entity occupies
            let min_x = feet_min.x.floor() as i32;
            let max_x = (physical.bounding_box.max.x + entity_pos.x).floor() as i32;
            let min_z = feet_min.z.floor() as i32;
            let max_z = (physical.bounding_box.max.z + entity_pos.z).floor() as i32;

            debug!(
                "Entity at {:?}, feet_y={}, support area: x=[{}, {}], z=[{}, {}]",
                pos.coords, feet_y, min_x, max_x, min_z, max_z
            );

            // Check if the broken block is in the entity's support area
            if broken_block_pos.pos.y == feet_y
                && broken_block_pos.pos.x >= min_x
                && broken_block_pos.pos.x <= max_x
                && broken_block_pos.pos.z >= min_z
                && broken_block_pos.pos.z <= max_z
            {
                // Block was broken beneath this entity
                // Now check if there are ANY other solid blocks supporting it
                let mut has_support = false;

                for x in min_x..=max_x {
                    for z in min_z..=max_z {
                        // Skip the broken block
                        if x == broken_block_pos.pos.x && z == broken_block_pos.pos.z {
                            continue;
                        }

                        let check_pos = IVec3::new(x, feet_y, z);
                        if is_solid_block(&state.0, check_pos) {
                            has_support = true;
                            break;
                        }
                    }
                    if has_support {
                        break;
                    }
                }

                // If no support found, unground the entity
                if !has_support {
                    debug!(
                        "Un-grounding entity at {:?} - no support remaining",
                        pos.coords
                    );
                    grounded.0 = false;
                } else {
                    debug!(
                        "Entity at {:?} still has support after block break",
                        pos.coords
                    );
                }
            } else {
                debug!(
                    "Broken block {:?} not in entity's support area (feet_y={}, x=[{}, {}], z=[{}, {}])",
                    broken_block_pos.pos, feet_y, min_x, max_x, min_z, max_z
                );
            }
        }
    }
}
