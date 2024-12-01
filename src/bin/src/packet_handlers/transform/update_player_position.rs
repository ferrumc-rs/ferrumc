use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::set_player_position::SetPlayerPositionEvent;
use ferrumc_state::GlobalState;
use ferrumc_net::utils::ecs_helpers::EntityExt;

#[event_handler]
async fn handle_player_move(
    event: TransformEvent,
    state: GlobalState,
) -> Result<TransformEvent, NetError> {
    let conn_id = event.conn_id;
    if let Some(ref new_position) = event.position {
        let mut position = conn_id.get_mut::<Position>(&state)?;

        *position = Position::new(
            new_position.x,
            new_position.y,
            new_position.z,
        );
    }

    if let Some(ref new_rotation) = event.rotation {
        let mut rotation = conn_id.get_mut::<Rotation>(&state)?;

        *rotation = Rotation::new(
            new_rotation.yaw,
            new_rotation.pitch,
        );

    }

    if let Some(new_grounded) = event.on_ground {
        let mut on_ground = conn_id.get_mut::<OnGround>(&state)?;

        *on_ground = OnGround(new_grounded);
    }

    Ok(event)
}