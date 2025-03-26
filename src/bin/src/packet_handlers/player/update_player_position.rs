use crate::chunk_sending::send_chunks;
use ferrumc_core::chunks::chunk_receiver::{ChunkReceiver, VIEW_DISTANCE};
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_ecs::entities::Entity;
use ferrumc_macros::{event_handler, NetEncode};
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::teleport_entity::TeleportEntityPacket;
use ferrumc_net::packets::outgoing::update_entity_position::UpdateEntityPositionPacket;
use ferrumc_net::packets::outgoing::update_entity_position_and_rotation::UpdateEntityPositionAndRotationPacket;
use ferrumc_net::packets::outgoing::update_entity_rotation::UpdateEntityRotationPacket;
use ferrumc_net::packets::packet_events::TransformEvent;
use ferrumc_net::utils::broadcast::{broadcast, BroadcastOptions};
use ferrumc_net::utils::ecs_helpers::EntityExt;
use ferrumc_net::NetResult;
use ferrumc_state::GlobalState;
use tracing::{trace, warn};

#[event_handler(priority = "fastest")]
fn handle_player_move(
    event: TransformEvent,
    state: GlobalState,
) -> Result<TransformEvent, NetError> {
    let conn_id = event.conn_id;

    let mut delta_pos = None::<(i16, i16, i16)>;
    let mut new_rot = None::<Rotation>;

    if let Some(ref new_position) = event.position {
        trace!("Getting chunk_recv 1 for player move");
        let mut chunks_need_sending = false;
        {
            let mut chunk_recv = state.universe.get_mut::<ChunkReceiver>(conn_id)?;
            if (new_position.x / 16.0).floor() != chunk_recv.last_chunk.0 as f64
                || (new_position.z / 16.0).floor() != chunk_recv.last_chunk.1 as f64
            {
                let (old_x, old_z) = (chunk_recv.last_chunk.0, chunk_recv.last_chunk.1);
                let (new_x, new_z) = (
                    (new_position.x / 16.0).floor() as i32,
                    (new_position.z / 16.0).floor() as i32,
                );
                {
                    if (old_x, old_z) != (new_x, new_z) {
                        chunk_recv.last_chunk = (new_x, new_z, String::from("overworld"));
                    } else {
                        warn!("Player crossed chunk but old and new chunks are the same");
                        return Ok(event);
                    }
                    chunk_recv.can_see.clear();
                    for x in new_x - VIEW_DISTANCE..new_x + VIEW_DISTANCE {
                        for z in new_z - VIEW_DISTANCE..new_z + VIEW_DISTANCE {
                            chunk_recv.can_see.insert((x, z, "overworld".to_string()));
                        }
                    }
                    chunks_need_sending = true;
                }
            }
        }

        if chunks_need_sending {
            send_chunks(state.clone(), conn_id)
                .map_err(|e| NetError::Misc(format!("Failed to send chunks to player: {:?}", e)))?;
        }

        trace!("Getting position 1 for player move");
        let mut position = conn_id.get_mut::<Position>(&state)?;
        trace!("Got position 1 for player move");

        delta_pos = Some((
            ((new_position.x * 4096.0) - (position.x * 4096.0)) as i16,
            ((new_position.y * 4096.0) - (position.y * 4096.0)) as i16,
            ((new_position.z * 4096.0) - (position.z * 4096.0)) as i16,
        ));

        *position = Position::new(new_position.x, new_position.y, new_position.z);
    }

    if let Some(ref new_rotation) = event.rotation {
        trace!("Getting rotation 1 for player move");
        let mut rotation = conn_id.get_mut::<Rotation>(&state)?;
        trace!("Got rotation 1 for player move");

        let new_rotation = Rotation::new(new_rotation.yaw, new_rotation.pitch);
        new_rot = Some(new_rotation);

        *rotation = new_rotation;
    }

    if let Some(new_grounded) = event.on_ground {
        trace!("Getting on_ground 1 for player move");
        let mut on_ground = conn_id.get_mut::<OnGround>(&state)?;
        trace!("Got on_ground 1 for player move");

        *on_ground = OnGround(new_grounded);
    }

    update_pos_for_all(conn_id, delta_pos, new_rot, &state)?;

    Ok(event)
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
    state: &GlobalState,
) -> NetResult<()> {
    let is_grounded = entity_id.get::<OnGround>(state)?.0;

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
        let pos = entity_id.get::<Position>(state)?;
        let rot = entity_id.get::<Rotation>(state)?;
        let grounded = entity_id.get::<OnGround>(state)?.0;

        BroadcastMovementPacket::TeleportEntity(TeleportEntityPacket::new(
            entity_id, &pos, &rot, grounded,
        ))
    } else {
        match (delta_pos, new_rot) {
            (Some(delta_pos), Some(new_rot)) => {
                BroadcastMovementPacket::UpdateEntityPositionAndRotation(
                    UpdateEntityPositionAndRotationPacket::new(
                        entity_id,
                        delta_pos,
                        &new_rot,
                        is_grounded,
                    ),
                )
            }
            (Some(delta_pos), None) => BroadcastMovementPacket::UpdateEntityPosition(
                UpdateEntityPositionPacket::new(entity_id, delta_pos, is_grounded),
            ),
            (None, Some(new_rot)) => BroadcastMovementPacket::UpdateEntityRotation(
                UpdateEntityRotationPacket::new(entity_id, &new_rot, is_grounded),
            ),
            _ => {
                return Ok(());
            }
        }
    };

    broadcast(&packet, state, BroadcastOptions::default().all())?;

    Ok(())
}
