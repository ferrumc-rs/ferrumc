use ferrumc_macros::event_handler;
use ferrumc_net::connection::ConnectionState;
use ferrumc_net::errors::NetError::{Packet};
use ferrumc_net::errors::{NetError, PacketError};
use ferrumc_net::packets::incoming::handshake::HandshakeEvent;
use ferrumc_state::GlobalState;
use tracing::{error, trace};
use ferrumc_ecs::errors::ECSError;
use ferrumc_net::utils::ecs_helpers::EntityExt;

#[event_handler]
async fn handle_handshake(
    handshake_event: HandshakeEvent,
    state: GlobalState,
) -> Result<HandshakeEvent, NetError> {
    trace!("Handling handshake event");
    let handshake = &handshake_event.handshake;

    // set connection state to handshake
    let entity = handshake_event.conn_id;
    let Ok(mut connection_state) = entity
        .get_mut::<ConnectionState>(state) else {
        error!("Failed to get connection state");
        return Err(NetError::ECSError(ECSError::ComponentNotFound));
    };

    trace!(
        "conn state: {} -> {}",
        connection_state.as_str(),
        handshake.next_state.val
    );

    let next_state = handshake.next_state.val as u8;
    *connection_state = match next_state {
        1 => ConnectionState::Status,
        2 => ConnectionState::Login,
        s => return Err(Packet(PacketError::InvalidState(s))),
    };

    Ok(handshake_event)
}
