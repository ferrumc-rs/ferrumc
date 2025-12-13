use bevy_ecs::prelude::{Entity, MessageReader, MessageWriter, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::NetEncode;
use ferrumc_messages::chunk_calc::ChunkCalc;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_position_sync::TeleportEntityPacket;
use ferrumc_net::packets::outgoing::update_entity_position::UpdateEntityPositionPacket;
use ferrumc_net::packets::outgoing::update_entity_position_and_rotation::UpdateEntityPositionAndRotationPacket;
use ferrumc_net::packets::outgoing::update_entity_rotation::UpdateEntityRotationPacket;
use ferrumc_net::packets::packet_messages::Movement;
use ferrumc_state::GlobalStateResource;
use std::sync::atomic::Ordering;
use tracing::{debug, trace, warn};

/// Maximum delta for position updates before teleport is required.
/// As per Minecraft protocol: "If the movement exceeds these limits, Teleport Entity should be sent instead."
const MAX_DELTA: i16 = (7.5 * 4096f32) as i16;

/// Enum representing all possible movement broadcast packets.
#[derive(NetEncode, Clone)]
enum BroadcastMovementPacket {
    UpdateEntityPosition(UpdateEntityPositionPacket),
    UpdateEntityPositionAndRotation(UpdateEntityPositionAndRotationPacket),
    UpdateEntityRotation(UpdateEntityRotationPacket),
    TeleportEntity(TeleportEntityPacket),
}

/// Unified movement handler that processes all player movement events.
/// This handles position-only, rotation-only, and combined position+rotation updates.
pub fn handle(
    mut movement_events: MessageReader<Movement>,
    mut chunk_calc_messages: MessageWriter<ChunkCalc>,
    mut transform_query: Query<(&mut Position, &mut Rotation, &mut OnGround, &PlayerIdentity)>,
    conn_query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for movement in movement_events.read() {
        let entity = movement.entity;

        // Check if player is still connected
        if !state.0.players.is_connected(entity) {
            trace!(
                "Player {:?} is not connected, skipping movement processing",
                entity
            );
            continue;
        }

        // Get the entity's transform components
        let Ok((mut position, mut rotation, mut on_ground, identity)) =
            transform_query.get_mut(entity)
        else {
            debug!(
                "Failed to get transform components for entity {:?}",
                entity
            );
            continue;
        };

        // Calculate delta position if position is being updated
        let delta_pos = movement.position.as_ref().map(|new_pos| {
            (
                ((new_pos.x * 4096.0) - (position.x * 4096.0)) as i16,
                ((new_pos.y * 4096.0) - (position.y * 4096.0)) as i16,
                ((new_pos.z * 4096.0) - (position.z * 4096.0)) as i16,
            )
        });

        // Check if chunk changed and emit chunk recalculation message
        if let Some(new_pos) = &movement.position {
            let old_chunk = (position.x as i32 >> 4, position.z as i32 >> 4);
            let new_chunk = (new_pos.x as i32 >> 4, new_pos.z as i32 >> 4);
            if old_chunk != new_chunk {
                chunk_calc_messages.write(ChunkCalc(entity));
            }
        }

        // Update position component if provided
        if let Some(new_pos) = &movement.position {
            *position = Position::new(new_pos.x, new_pos.y, new_pos.z);
        }

        // Update rotation component if provided
        if let Some(new_rot) = &movement.rotation {
            *rotation = Rotation::new(new_rot.yaw, new_rot.pitch);
        }

        // Update on_ground component if provided
        if let Some(grounded) = movement.on_ground {
            *on_ground = OnGround(grounded);
        }

        // Build and broadcast the appropriate movement packet to other players
        let packet = build_broadcast_packet(
            delta_pos,
            movement.rotation.as_ref(),
            &position,
            &rotation,
            on_ground.0,
            identity,
        );

        if let Some(packet) = packet {
            broadcast_movement_to_players(entity, &packet, &conn_query, &state);
        }

        trace!(
            "Processed movement for entity {:?}: pos={:?}, rot={:?}, ground={:?}",
            entity,
            movement.position,
            movement.rotation,
            movement.on_ground
        );
    }
}

/// Determines if delta position exceeds the threshold for teleport.
fn delta_exceeds_threshold(delta_pos: Option<(i16, i16, i16)>) -> bool {
    match delta_pos {
        Some((delta_x, delta_y, delta_z)) => {
            // Prevent int overflow, since abs of i16::MIN would overflow
            if delta_x == i16::MIN || delta_y == i16::MIN || delta_z == i16::MIN {
                true
            } else {
                delta_x.abs() > MAX_DELTA || delta_y.abs() > MAX_DELTA || delta_z.abs() > MAX_DELTA
            }
        }
        None => false,
    }
}

/// Builds the appropriate broadcast packet based on what changed.
fn build_broadcast_packet(
    delta_pos: Option<(i16, i16, i16)>,
    new_rot: Option<&Rotation>,
    current_pos: &Position,
    current_rot: &Rotation,
    on_ground: bool,
    identity: &PlayerIdentity,
) -> Option<BroadcastMovementPacket> {
    // If delta exceeds threshold, use teleport packet
    if delta_exceeds_threshold(delta_pos) {
        return Some(BroadcastMovementPacket::TeleportEntity(
            TeleportEntityPacket::new(identity, current_pos, current_rot, on_ground),
        ));
    }

    // Build appropriate packet based on what was updated
    match (delta_pos, new_rot) {
        (Some(delta), Some(rot)) => Some(BroadcastMovementPacket::UpdateEntityPositionAndRotation(
            UpdateEntityPositionAndRotationPacket::new(identity, delta, rot, on_ground),
        )),
        (Some(delta), None) => Some(BroadcastMovementPacket::UpdateEntityPosition(
            UpdateEntityPositionPacket::new(identity, delta, on_ground),
        )),
        (None, Some(rot)) => Some(BroadcastMovementPacket::UpdateEntityRotation(
            UpdateEntityRotationPacket::new(identity, rot, on_ground),
        )),
        (None, None) => None,
    }
}

/// Broadcasts a movement packet to all connected players except the source entity.
fn broadcast_movement_to_players(
    source_entity: Entity,
    packet: &BroadcastMovementPacket,
    conn_query: &Query<(Entity, &StreamWriter)>,
    state: &Res<GlobalStateResource>,
) {
    for (entity, conn) in conn_query.iter() {
        // Don't send to the player who moved
        if entity == source_entity {
            continue;
        }

        // Check if player is still connected
        if !state.0.players.is_connected(entity) || !conn.running.load(Ordering::Relaxed) {
            warn!(
                "Player {:?} is not connected, skipping movement broadcast",
                entity
            );
            state
                .0
                .players
                .disconnect(entity, Some(String::from("Player not connected anymore.")));
            continue;
        }

        if let Err(e) = conn.send_packet_ref(packet) {
            warn!(
                "Failed to send movement packet to player {:?}: {}",
                entity, e
            );
        }
    }
}
