//! Unified movement broadcast handler.
//!
//! This module handles broadcasting player movement to other players. It receives
//! `Movement` messages from the three movement packet handlers (position only,
//! rotation only, position+rotation) and broadcasts the appropriate packets to
//! all other connected players.
//!
//! ## Packet Types Sent
//! - `UpdateEntityPosition` - For position-only changes within delta threshold
//! - `UpdateEntityRotation` - For rotation-only changes
//! - `UpdateEntityPositionAndRotation` - For combined changes within delta threshold
//! - `TeleportEntity` - For large movements that exceed the delta threshold
//! - `SetHeadRotation` - Sent alongside any rotation update for head tracking

use bevy_ecs::prelude::{Entity, MessageReader, Query};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::NetEncode;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_position_sync::TeleportEntityPacket;
use ferrumc_net::packets::outgoing::set_head_rotation::SetHeadRotationPacket;
use ferrumc_net::packets::outgoing::update_entity_position::UpdateEntityPositionPacket;
use ferrumc_net::packets::outgoing::update_entity_position_and_rotation::UpdateEntityPositionAndRotationPacket;
use ferrumc_net::packets::outgoing::update_entity_rotation::UpdateEntityRotationPacket;
use ferrumc_net::packets::packet_messages::Movement;
use ferrumc_net_codec::net_types::angle::NetAngle;

use tracing::error;

/// Enum to hold all possible movement broadcast packets.
/// Using an enum with `#[derive(NetEncode)]` allows sending any variant
/// through a single `send_packet_ref` call.
#[derive(NetEncode, Clone)]
enum BroadcastMovementPacket {
    UpdateEntityPosition(UpdateEntityPositionPacket),
    UpdateEntityPositionAndRotation(UpdateEntityPositionAndRotationPacket),
    UpdateEntityRotation(UpdateEntityRotationPacket),
    TeleportEntity(TeleportEntityPacket),
}

/// Maximum delta that can be sent with relative position packets (in protocol units).
/// If any axis delta exceeds this threshold, a teleport packet is used instead.
///
/// The Minecraft protocol recommends using teleport packets for movements > 7.5 blocks.
/// Protocol units are 1/4096 of a block, so: 7.5 * 4096 = 30720
const MAX_DELTA: i16 = (7.5 * 4096f32) as i16;

/// Handles `Movement` messages and broadcasts appropriate packets to other players.
///
/// This is the unified broadcast handler that replaces the previous per-packet-type
/// broadcast logic, ensuring consistent handling across all movement packet types.
pub fn handle_player_move(
    mut movement_msgs: MessageReader<Movement>,
    query: Query<(&Position, &Rotation, &PlayerIdentity)>,
    broadcast_query: Query<(Entity, &StreamWriter)>,
) {
    for movement in movement_msgs.read() {
        let sender_entity = movement.entity;

        let Ok((pos, rot, identity)) = query.get(sender_entity) else {
            continue;
        };

        let has_rotation = movement.rotation.is_some();

        // Check if delta exceeds threshold (need to use teleport instead).
        // The Movement message already clamps deltas to i16 range, but we still
        // need to detect large movements to switch to teleport packets.
        let delta_exceeds_threshold = match movement.delta_position {
            Some((delta_x, delta_y, delta_z)) => {
                // Check for clamped values (indicates overflow during calculation)
                // or values exceeding the protocol's recommended threshold
                if delta_x == i16::MIN
                    || delta_y == i16::MIN
                    || delta_z == i16::MIN
                    || delta_x == i16::MAX
                    || delta_y == i16::MAX
                    || delta_z == i16::MAX
                {
                    true
                } else {
                    delta_x.abs() > MAX_DELTA
                        || delta_y.abs() > MAX_DELTA
                        || delta_z.abs() > MAX_DELTA
                }
            }
            None => false,
        };

        // Build the appropriate movement packet
        let movement_packet: Option<BroadcastMovementPacket> = if delta_exceeds_threshold {
            Some(BroadcastMovementPacket::TeleportEntity(
                TeleportEntityPacket::new(identity, pos, rot, movement.on_ground),
            ))
        } else {
            match (movement.delta_position, has_rotation) {
                (Some(delta), true) => {
                    Some(BroadcastMovementPacket::UpdateEntityPositionAndRotation(
                        UpdateEntityPositionAndRotationPacket::new(
                            identity,
                            delta,
                            rot,
                            movement.on_ground,
                        ),
                    ))
                }
                (Some(delta), false) => Some(BroadcastMovementPacket::UpdateEntityPosition(
                    UpdateEntityPositionPacket::new(identity, delta, movement.on_ground),
                )),
                (None, true) => Some(BroadcastMovementPacket::UpdateEntityRotation(
                    UpdateEntityRotationPacket::new(identity, rot, movement.on_ground),
                )),
                (None, false) => None,
            }
        };

        // Build head rotation packet if we have rotation
        let head_rot_packet = if has_rotation {
            Some(SetHeadRotationPacket::new(
                identity.short_uuid,
                NetAngle::from_degrees(rot.yaw as f64),
            ))
        } else {
            None
        };

        // Broadcast to all other connected players
        for (recipient_entity, writer) in broadcast_query.iter() {
            // Skip sending to the sender
            if recipient_entity == sender_entity {
                continue;
            }

            // Skip disconnected players
            if !writer.is_running() {
                continue;
            }

            // Send the movement packet (position and/or rotation)
            if let Some(ref packet) = movement_packet {
                if let Err(err) = writer.send_packet_ref(packet) {
                    error!("Failed to send movement packet: {:?}", err);
                }
            }

            // Send head rotation packet if applicable
            if let Some(ref packet) = head_rot_packet {
                if let Err(err) = writer.send_packet_ref(packet) {
                    error!("Failed to send head rotation packet: {:?}", err);
                }
            }
        }
    }
}
