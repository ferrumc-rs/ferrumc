use bevy_ecs::prelude::{Entity, Query};
use ferrumc_net::SetPlayerPositionPacketReceiver;


use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::NetEncode;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::teleport_entity::TeleportEntityPacket;
use ferrumc_net::packets::outgoing::update_entity_position::UpdateEntityPositionPacket;
use ferrumc_net::packets::outgoing::update_entity_position_and_rotation::UpdateEntityPositionAndRotationPacket;
use ferrumc_net::packets::outgoing::update_entity_rotation::UpdateEntityRotationPacket;
use ferrumc_net::utils::ecs_helpers::EntityExt;
pub fn handle(
    events: SetPlayerPositionPacketReceiver,
    mut pos_query: Query<(&mut Position, &mut OnGround)>,
    pass_pos_query: Query<(&Position, &Rotation, &OnGround)>,
    pass_conn_query: Query<&StreamWriter>,
) {
    for (event, eid) in events.0 {
        let mut delta_pos = None::<(i16, i16, i16)>;
        let mut new_rot = None::<Rotation>;

        let new_position = Position::new(event.x, event.feet_y, event.z);

        let (mut position, mut on_ground) = pos_query.get_mut(eid).expect(
            "Failed to get position and on_ground components",
        );

        delta_pos = Some((
            ((new_position.x * 4096.0) - (position.x * 4096.0)) as i16,
            ((new_position.y * 4096.0) - (position.y * 4096.0)) as i16,
            ((new_position.z * 4096.0) - (position.z * 4096.0)) as i16,
        ));

        *position = Position::new(new_position.x, new_position.y, new_position.z);

        if let Some(new_grounded) = event.on_ground {
            *on_ground = OnGround(new_grounded);
        }

        update_pos_for_all(eid, delta_pos, new_rot, &pass_pos_query, &pass_conn_query).expect(
            "Failed to update position for all players",
        );
    }
}

#[derive(NetEncode)]
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
    pos_query: &Query<(&Position, &Rotation, &OnGround)>,
    conn_query: &Query<&StreamWriter>,
) -> Result<(), NetError> {
    let (pos, rot, grounded) = pos_query.get(entity_id)?;

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
            entity_id, &pos, &rot, grounded.0,
        ))
    } else {
        match (delta_pos, new_rot) {
            (Some(delta_pos), Some(new_rot)) => {
                BroadcastMovementPacket::UpdateEntityPositionAndRotation(
                    UpdateEntityPositionAndRotationPacket::new(
                        entity_id, delta_pos, &new_rot, grounded.0,
                    ),
                )
            }
            (Some(delta_pos), None) => BroadcastMovementPacket::UpdateEntityPosition(
                UpdateEntityPositionPacket::new(entity_id, delta_pos, grounded.0),
            ),
            (None, Some(new_rot)) => BroadcastMovementPacket::UpdateEntityRotation(
                UpdateEntityRotationPacket::new(entity_id, &new_rot, grounded.0),
            ),
            _ => {
                return Ok(());
            }
        }
    };

    for writer in conn_query.iter() {
        writer.write_packet(&packet)?;
    }

    Ok(())
}
