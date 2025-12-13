use bevy_ecs::prelude::*;
use bevy_math::DVec3;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::transform::{
    grounded::OnGround, position::Position, rotation::Rotation, velocity::Velocity,
};
use ferrumc_entities::collision::{check_collision, is_in_water, PIG_AABB};
use ferrumc_entities::components::{EntityMetadata, LastSyncedPosition};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::update_entity_position_and_rotation::UpdateEntityPositionAndRotationPacket;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;

type EntitySyncComponents<'a> = (
    &'a EntityIdentity,
    &'a Position,
    &'a Rotation,
    &'a OnGround,
    &'a mut LastSyncedPosition,
);

/// Entity physics system that handles gravity, collision, and movement.
///
/// This system runs every game tick and:
/// - Applies gravity and buoyancy forces
/// - Checks collision with world blocks
/// - Updates entity positions
/// - Applies friction based on environment
pub fn entity_physics_system(
    mut query: Query<(&mut Position, &mut Velocity, &mut OnGround), With<EntityMetadata>>,
    state: Res<GlobalStateResource>,
) {
    for (mut pos, mut vel, mut on_ground) in query.iter_mut() {
        let in_water = is_in_water(&state.0, &pos, &PIG_AABB);

        // Apply gravity and buoyancy
        if in_water {
            vel.y += GRAVITY + WATER_BUOYANCY;
        } else if !on_ground.0 {
            vel.y = (vel.y + GRAVITY).max(TERMINAL_VELOCITY);
        } else if vel.y < 0.0 {
            vel.y = 0.0;
        }

        // Calculate new position
        let new_pos = pos.coords + **vel;

        // Check collision and update position
        let test_pos = Position::from(new_pos);
        if !check_collision(&state.0, &test_pos, &PIG_AABB) {
            pos.coords = new_pos;
            on_ground.0 = false;
        } else {
            // Try each axis separately
            let mut final_pos = pos.coords;

            // Try Y movement first (jumping/falling)
            let test_y = Position::from(DVec3::new(pos.x, new_pos.y, pos.z));
            if !check_collision(&state.0, &test_y, &PIG_AABB) {
                final_pos.y = new_pos.y;
                on_ground.0 = false;
            } else {
                vel.y = 0.0;
                // Check if collision is below (on ground)
                if new_pos.y < pos.y {
                    on_ground.0 = true;
                }
            }

            // Try X movement with updated Y position
            let test_x = Position::from(DVec3::new(new_pos.x, final_pos.y, pos.z));
            if !check_collision(&state.0, &test_x, &PIG_AABB) {
                final_pos.x = new_pos.x;
            } else {
                vel.x = 0.0;
            }

            // Try Z movement with updated X and Y positions
            let test_z = Position::from(DVec3::new(final_pos.x, final_pos.y, new_pos.z));
            if !check_collision(&state.0, &test_z, &PIG_AABB) {
                final_pos.z = new_pos.z;
            } else {
                vel.z = 0.0;
            }

            pos.coords = final_pos;
        }

        // Apply friction
        if in_water {
            vel.x *= WATER_DRAG;
            vel.z *= WATER_DRAG;
            vel.y *= WATER_VERTICAL_DRAG;
        } else if on_ground.0 {
            vel.x *= GROUND_FRICTION;
            vel.z *= GROUND_FRICTION;
        } else {
            vel.x *= AIR_RESISTANCE;
            vel.z *= AIR_RESISTANCE;
        }
    }
}

/// Synchronizes entity movement to all connected clients.
///
/// Uses delta compression to minimize packet size.
pub fn entity_movement_sync(
    mut entity_query: Query<
        EntitySyncComponents<'static>,
        (With<EntityMetadata>, Without<StreamWriter>),
    >,
    client_query: Query<&StreamWriter>,
) {
    for (identity, pos, rot, on_ground, mut last_pos) in entity_query.iter_mut() {
        // Only sync if entity has moved
        if !last_pos.has_moved(pos) {
            continue;
        }

        let delta = last_pos.delta_to(pos);

        // Send update to all connected clients
        for stream_writer in client_query.iter() {
            let packet = UpdateEntityPositionAndRotationPacket {
                entity_id: VarInt::new(identity.entity_id),
                delta_x: delta.0,
                delta_y: delta.1,
                delta_z: delta.2,
                yaw: ferrumc_net_codec::net_types::angle::NetAngle::from_degrees(rot.yaw as f64),
                pitch: ferrumc_net_codec::net_types::angle::NetAngle::from_degrees(
                    rot.pitch as f64,
                ),
                on_ground: on_ground.0,
            };

            if let Err(e) = stream_writer.send_packet(packet) {
                tracing::error!("Failed to send entity movement packet: {:?}", e);
            }
        }

        // Update last synced position
        *last_pos = LastSyncedPosition::from_position(pos);
    }
}
