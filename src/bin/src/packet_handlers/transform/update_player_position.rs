use ferrumc_core::transform::position::Position;
use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::GlobalState;
use ferrumc_net::packets::incoming::set_player_position::SetPlayerPositionEvent;

#[event_handler]
async fn handle_player_move(
    event: SetPlayerPositionEvent,
    state: GlobalState,
) -> Result<SetPlayerPositionEvent, NetError> {
    let new_position = &event.data;
    let conn_id = event.conn_id;

    let mut position = state
        .universe
        .get_mut::<Position>(conn_id)?;

    *position = Position::new(new_position.x, new_position.feet_y, new_position.z);

    Ok(event)
}