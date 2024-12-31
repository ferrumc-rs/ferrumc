use tracing::info;
use ferrumc_macros::event_handler;
use ferrumc_net::connection::PlayerDisconnectEvent;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::handshake::HandshakeEvent;
use ferrumc_state::GlobalState;

#[event_handler]
async fn handle_player_disconnect(
    event: PlayerDisconnectEvent,
    state: GlobalState,
) -> Result<PlayerDisconnectEvent, NetError> {
    let entity_id = event.conn_id;
    
    info!("Player disconnected: {:?}", entity_id);

    Ok(event)
}