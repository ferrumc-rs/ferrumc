use ferrumc_macros::event_handler;
use ferrumc_net::connection::ConnectionState;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::update_time::TickEvent;
use ferrumc_net::packets::outgoing::update_time::UpdateTimePacket;
use ferrumc_net::utils::broadcast::{BroadcastOptions, BroadcastToAll};
use ferrumc_net::GlobalState;

#[event_handler]
async fn handle_tick(event: TickEvent, state: GlobalState) -> Result<TickEvent, NetError> {
    // info!("Tick {} ", event.tick);
    // TODO: Handle tick in terms of game logic here
    // this should call a function in world which handles the world state and calls the appropriate events which send their respective packets

    ///////

    let packet = UpdateTimePacket::new(event.tick, event.tick % 24000);

    let entities = state
        .universe
        .query::<(&mut StreamWriter, &ConnectionState)>()
        .into_entities()
        .into_iter()
        .filter_map(|entity| {
            let conn_state = state.universe.get::<ConnectionState>(entity).ok()?;
            if matches!(*conn_state, ConnectionState::Play) {
                Some(entity)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    state.broadcast(&packet, BroadcastOptions::default().only(entities)).await?;

    Ok(event)
}
