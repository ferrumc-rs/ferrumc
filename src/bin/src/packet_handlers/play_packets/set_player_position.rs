use bevy_ecs::prelude::{Entity, EventWriter, Query, Res};
use ferrumc_core::chunks::cross_chunk_boundary_event::CrossChunkBoundaryEvent;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::SetPlayerPositionPacketReceiver;

use crate::errors::BinaryError;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::NetEncode;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_position_sync::TeleportEntityPacket;
use ferrumc_net::packets::outgoing::update_entity_position::UpdateEntityPositionPacket;
use ferrumc_net::packets::outgoing::update_entity_position_and_rotation::UpdateEntityPositionAndRotationPacket;
use ferrumc_net::packets::outgoing::update_entity_rotation::UpdateEntityRotationPacket;

pub fn handle(
    events: Res<SetPlayerPositionPacketReceiver>,
    mut pos_query: Query<(&mut Position, &mut OnGround, &Rotation, &PlayerIdentity)>,
    pass_conn_query: Query<&StreamWriter>,
    mut cross_chunk_events: EventWriter<CrossChunkBoundaryEvent>,
) {
    for (event, eid) in events.0.try_iter() {
        let new_rot = None::<Rotation>;

        let new_position = Position::new(event.x, event.feet_y, event.z);

        let (mut position, mut on_ground, _, _) = pos_query
            .get_mut(eid)
            .expect("Failed to get position and on_ground components");

        let delta_pos = Some((
            ((new_position.x * 4096.0) - (position.x * 4096.0)) as i16,
            ((new_position.y * 4096.0) - (position.y * 4096.0)) as i16,
            ((new_position.z * 4096.0) - (position.z * 4096.0)) as i16,
        ));

        let old_chunk = ((position.x as i32 >> 4), (position.z as i32 >> 4));

        let new_chunk = ((new_position.x as i32 >> 4), (new_position.z as i32 >> 4));

        if old_chunk != new_chunk {
            cross_chunk_events.write(CrossChunkBoundaryEvent {
                player: eid,
                old_chunk,
                new_chunk,
            });
        }

        *position = Position::new(new_position.x, new_position.y, new_position.z);

        *on_ground = OnGround(event.on_ground);

        update_pos_for_all(eid, delta_pos, new_rot, &pos_query, &pass_conn_query)
            .expect("Failed to update position for all players");
    }
}

#[derive(NetEncode, Clone)]
enum BroadcastMovementPacket {
    UpdateEntityPosition(UpdateEntityPositionPacket),
    UpdateEntityPositionAndRotation(UpdateEntityPositionAndRotationPacket),
    UpdateEntityRotation(UpdateEntityRotationPacket),
    TeleportEntity(TeleportEntityPacket),
}

fn update_pos_for_all(
    entity_id: Entity,
    delta_pos: Option<(i16, i16, i16)>,
    new_rot: Option<Rotation>,
    pos_query: &Query<(&mut Position, &mut OnGround, &Rotation, &PlayerIdentity)>,
    conn_query: &Query<&StreamWriter>,
) -> Result<(), BinaryError> {
    let (pos, grounded, rot, identity) = pos_query.get(entity_id)?;

    // If any delta of (x|y|z) exceeds 7.5, then it's "not recommended" to use this packet
    // As docs say: "If the movement exceeds these limits, Teleport Entity should be sent instead."
    // "should"????
    const MAX_DELTA: i16 = (7.5 * 4096f32) as i16;
    let delta_exceeds_threshold = match delta_pos {
        Some((delta_x, delta_y, delta_z)) => {
            // Prevent int overflow, since abs of i16::MIN would overflow?
            if delta_x == i16::MIN || delta_y == i16::MIN || delta_z == i16::MIN {
                true
            } else {
                delta_x.abs() > MAX_DELTA || delta_y.abs() > MAX_DELTA || delta_z.abs() > MAX_DELTA
            }
        }
        None => false,
    };

    let packet: BroadcastMovementPacket = if delta_exceeds_threshold {
        BroadcastMovementPacket::TeleportEntity(TeleportEntityPacket::new(
            identity, pos, rot, grounded.0,
        ))
    } else {
        match (delta_pos, new_rot) {
            (Some(delta_pos), Some(new_rot)) => {
                BroadcastMovementPacket::UpdateEntityPositionAndRotation(
                    UpdateEntityPositionAndRotationPacket::new(
                        identity, delta_pos, &new_rot, grounded.0,
                    ),
                )
            }
            (Some(delta_pos), None) => BroadcastMovementPacket::UpdateEntityPosition(
                UpdateEntityPositionPacket::new(identity, delta_pos, grounded.0),
            ),
            (None, Some(new_rot)) => BroadcastMovementPacket::UpdateEntityRotation(
                UpdateEntityRotationPacket::new(identity, &new_rot, grounded.0),
            ),
            _ => {
                return Ok(());
            }
        }
    };

    for writer in conn_query.iter() {
        if !writer.running.load(std::sync::atomic::Ordering::Relaxed) {
            continue;
        }
        writer.send_packet(packet.clone())?;
    }

    Ok(())
}
