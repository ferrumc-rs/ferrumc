use tracing::info;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::event_handler;
use ferrumc_net::GlobalState;
use ferrumc_net::packets::incoming::handshake::Handshake;

#[event_handler]
async fn handle_handshake(
    handshake: Handshake,
    _state: GlobalState,
) -> Result<Handshake, <Handshake as Event>::Error> {
    info!("Handling handshake event: {:?}", handshake);

    Ok(handshake)
}
