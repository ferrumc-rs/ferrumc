use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::packet_events::TransformEvent;
use ferrumc_net::utils::ecs_helpers::EntityExt;
use ferrumc_state::GlobalState;
use tracing::{debug, trace};

#[event_handler]
async fn handle_player_move(
    event: TransformEvent,
    state: GlobalState,
) -> Result<TransformEvent, NetError> {
    let conn_id = event.conn_id;
    if let Some(ref new_position) = event.position {
        debug!(
            "Block: {}",
            state
                .world
                .get_block(
                    new_position.x.floor() as i32,
                    new_position.y.floor() as i32 - 1,
                    new_position.z.floor() as i32,
                    "overworld"
                )
                .await
                .unwrap()
        );
        trace!("Getting chunk_recv 1 for player move");
        {
            let mut chunk_recv = state.universe.get_mut::<ChunkReceiver>(conn_id)?;
            trace!("Got chunk_recv 1 for player move");
            if let Some(last_chunk) = &chunk_recv.last_chunk {
                let new_chunk = (
                    new_position.x as i32 / 16,
                    new_position.z as i32 / 16,
                    String::from("overworld"),
                );
                if *last_chunk != new_chunk {
                    chunk_recv.last_chunk = Some(new_chunk);
                    chunk_recv.calculate_chunks().await;
                }
            } else {
                chunk_recv.last_chunk = Some((
                    new_position.x as i32 / 16,
                    new_position.z as i32 / 16,
                    String::from("overworld"),
                ));
                chunk_recv.calculate_chunks().await;
            }
        }

        trace!("Getting position 1 for player move");
        let mut position = conn_id.get_mut::<Position>(&state)?;
        trace!("Got position 1 for player move");
        *position = Position::new(new_position.x, new_position.y, new_position.z);
    }

    if let Some(ref new_rotation) = event.rotation {
        trace!("Getting rotation 1 for player move");
        let mut rotation = conn_id.get_mut::<Rotation>(&state)?;
        trace!("Got rotation 1 for player move");

        *rotation = Rotation::new(new_rotation.yaw, new_rotation.pitch);
    }

    if let Some(new_grounded) = event.on_ground {
        trace!("Getting on_ground 1 for player move");
        let mut on_ground = conn_id.get_mut::<OnGround>(&state)?;
        trace!("Got on_ground 1 for player move");

        *on_ground = OnGround(new_grounded);
    }

    Ok(event)
}
