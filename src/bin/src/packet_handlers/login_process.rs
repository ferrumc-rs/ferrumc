use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::login_start::LoginStartEvent;
use ferrumc_net::GlobalState;
use tracing::info;

#[event_handler]
async fn handle_login_start(
    login_start_event: LoginStartEvent,
    _state: GlobalState,
) -> Result<LoginStartEvent, NetError> {
    info!("Handling login start event");

    Ok(login_start_event)
}
