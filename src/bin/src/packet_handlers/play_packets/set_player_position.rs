use bevy_ecs::prelude::{Entity, EventWriter, Query, Res};
use ferrumc_components::player::identity::PlayerIdentity;
use ferrumc_messages::chunk::CrossChunkBoundaryEvent;
use ferrumc_net::SetPlayerPositionPacketReceiver;
use std::sync::atomic::Ordering;
use tracing::{debug, error, trace};

use crate::errors::BinaryError;
use ferrumc_components::player::transform::grounded::OnGround;
use ferrumc_components::player::transform::position::Position;
use ferrumc_components::player::transform::rotation::Rotation;
use ferrumc_components::state::server_state::{GlobalState, GlobalStateResource};
use ferrumc_macros::NetEncode;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_position_sync::TeleportEntityPacket;
use ferrumc_net::packets::outgoing::update_entity_position::UpdateEntityPositionPacket;
use ferrumc_net::packets::outgoing::update_entity_position_and_rotation::UpdateEntityPositionAndRotationPacket;
use ferrumc_net::packets::outgoing::update_entity_rotation::UpdateEntityRotationPacket;

pub fn handle(
    events: Res<SetPlayerPositionPacketReceiver>,
    mut pos_query: Query<(&mut Position, &mut OnGround, &Rotation, &PlayerIdentity)>,
    pass_conn_query: Query<(Entity, &StreamWriter)>,
    mut cross_chunk_events: EventWriter<CrossChunkBoundaryEvent>,
    state: Res<GlobalStateResource>,
) {
    for (event, eid) in events.0.try_iter() {
        if !state.0.players.is_connected(eid) {
            debug!(
                "Player {} is not connected, skipping SetPlayerPositionPacket processing",
                eid
            );
            // Player is not connected, skip processing this event
            continue;
        }
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

        let old_chunk = (position.x as i32 >> 4, position.z as i32 >> 4);

        let new_chunk = (new_position.x as i32 >> 4, new_position.z as i32 >> 4);

        if old_chunk != new_chunk {
            cross_chunk_events.write(CrossChunkBoundaryEvent {
                player: eid,
                old_chunk,
                new_chunk,
            });
        }

        *position = Position::new(new_position.x, new_position.y, new_position.z);

        *on_ground = OnGround::from(event.on_ground);

        if let Err(err) = update_pos_for_all(
            eid,
            delta_pos,
            new_rot,
            &pos_query,
            &pass_conn_query,
            state.0.clone(),
        ) {
            error!("Failed to update position for player {}: {}", eid, err);
        } else {
            trace!(
                "Updated position for player {}: ({}, {}, {})",
                eid,
                new_position.x,
                new_position.y,
                new_position.z
            );
        }
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
    conn_query: &Query<(Entity, &StreamWriter)>,
    state: GlobalState,
) -> Result<(), BinaryError> {
    if !state.players.is_connected(entity_id) {
        // Use trace/debug to avoid log spam on disconnects
        trace!(
            "Player {} is not connected, skipping position update",
            entity_id
        );
        return Ok(());
    }
    let (pos, grounded, rot, identity) = pos_query.get(entity_id)?;

    const MAX_DELTA: i16 = (7.5 * 4096f32) as i16;
    let delta_exceeds_threshold = match delta_pos {
        Some((delta_x, delta_y, delta_z)) => {
            if delta_x == i16::MIN || delta_y == i16::MIN || delta_z == i16::MIN {
                true
            } else {
                delta_x.abs() > MAX_DELTA || delta_y.abs() > MAX_DELTA || delta_z.abs() > MAX_DELTA
            }
        }
        None => false,
    };

    let packet: BroadcastMovementPacket = if delta_exceeds_threshold {
        // If they moved too fast, we broadcast a Teleport to everyone else
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

    for (entity, conn) in conn_query.iter() {
        // --- FIX: Don't send the movement packet back to the player who moved ---
        if entity == entity_id {
            continue;
        }
        // ----------------------------------------------------------------------

        if !state.players.is_connected(entity) || !conn.running.load(Ordering::Relaxed) {
            // (Log logic is fine, just skipped for brevity)
            continue;
        }

        // Add a distance check here later!
        // e.g. if pos.distance(other_pos) > view_distance { continue; }

        conn.send_packet_ref(&packet)?;
    }

    Ok(())
}
