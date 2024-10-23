use ferrumc_core::transform::position::Position;
use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::set_player_position::SetPlayerPositionEvent;
use ferrumc_net::GlobalState;
use ferrumc_net::utils::ecs_helpers::EntityExt;

#[event_handler]
async fn handle_player_move(
    event: SetPlayerPositionEvent,
    state: GlobalState,
) -> Result<SetPlayerPositionEvent, NetError> {
    let new_position = &event.data;
    let conn_id = event.conn_id;

    let mut position = conn_id.get_mut::<Position>(state)?;
    
    *position = Position::new(
        new_position.x,
        new_position.feet_y,
        new_position.z
    );
    
    Ok(event)
}