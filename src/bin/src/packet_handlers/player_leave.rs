use ferrumc_macros::event_handler;
use ferrumc_net::connection::PlayerDisconnectEvent;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::remove_entities::RemoveEntitiesPacket;
use ferrumc_net::utils::broadcast::{broadcast, BroadcastOptions};
use ferrumc_state::GlobalState;
use tracing::info;

#[event_handler]
fn handle_player_disconnect(
    event: PlayerDisconnectEvent,
    state: GlobalState,
) -> Result<PlayerDisconnectEvent, NetError> {
    let entity_id = event.entity_id;

    info!("Player disconnected: {:?}", entity_id);

    let remove_entity_packet = RemoveEntitiesPacket::from_entities([entity_id]);

    broadcast(
        &remove_entity_packet,
        &state,
        BroadcastOptions::default().all(),
    )
        ?;

    Ok(event)
}
