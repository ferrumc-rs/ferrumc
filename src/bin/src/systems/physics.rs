use bevy_ecs::prelude::*;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::transform::{
    grounded::OnGround, position::Position, rotation::Rotation, velocity::Velocity,
};
use ferrumc_entities::collision::{check_collision, is_in_water, BoundingBox};
use ferrumc_entities::components::{EntityMetadata, LastSyncedPosition};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::update_entity_position_and_rotation::UpdateEntityPositionAndRotationPacket;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;

const GRAVITY: f64 = -0.08;
const TERMINAL_VELOCITY: f64 = -3.92;
const WATER_BUOYANCY: f64 = 0.09;
const WATER_DRAG: f64 = 0.8;

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
    mut query: Query<(&mut Position, &mut Velocity, &OnGround), With<EntityMetadata>>,
    state: Res<GlobalStateResource>,
) {
    let bbox = BoundingBox::PIG;

    for (mut pos, mut vel, on_ground) in query.iter_mut() {
        let in_water = is_in_water(&state.0, pos.x, pos.y, pos.z, &bbox);

        // Apply gravity and buoyancy
        if in_water {
            vel.y += GRAVITY + WATER_BUOYANCY;
        } else if !on_ground.0 {
            vel.y = (vel.y + GRAVITY).max(TERMINAL_VELOCITY);
        } else if vel.y < 0.0 {
            vel.y = 0.0;
        }

        // Calculate new position
        let new_x = pos.x + vel.x;
        let new_y = pos.y + vel.y;
        let new_z = pos.z + vel.z;

        // Check collision and update position
        if !check_collision(&state.0, new_x, new_y, new_z, &bbox) {
            *pos = Position::new(new_x, new_y, new_z);
        } else {
            // Try each axis separately
            let mut final_x = pos.x;
            let mut final_y = pos.y;
            let mut final_z = pos.z;

            if !check_collision(&state.0, pos.x, new_y, pos.z, &bbox) {
                final_y = new_y;
            } else {
                vel.y = 0.0;
            }

            if !check_collision(&state.0, new_x, final_y, pos.z, &bbox) {
                final_x = new_x;
            } else {
                vel.x = 0.0;
            }

            if !check_collision(&state.0, final_x, final_y, new_z, &bbox) {
                final_z = new_z;
            } else {
                vel.z = 0.0;
            }

            *pos = Position::new(final_x, final_y, final_z);
        }

        // Apply friction
        if in_water {
            vel.x *= WATER_DRAG;
            vel.z *= WATER_DRAG;
            vel.y *= 0.95;
        } else if on_ground.0 {
            vel.x *= 0.85;
            vel.z *= 0.85;
        } else {
            vel.x *= 0.98;
            vel.z *= 0.98;
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
