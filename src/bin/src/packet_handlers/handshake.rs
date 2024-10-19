use ferrumc_events::infrastructure::Event;
use ferrumc_macros::event_handler;
use ferrumc_net::connection::ConnectionState;
use ferrumc_net::errors::NetError::Packet;
use ferrumc_net::errors::{NetError, PacketError};
use ferrumc_net::packets::incoming::handshake::HandshakeEvent;
use ferrumc_net::GlobalState;
use tracing::trace;

#[event_handler]
async fn handle_handshake(
    handshake_event: HandshakeEvent,
    state: GlobalState,
) -> Result<HandshakeEvent, NetError> {
    trace!("Handling handshake event");
    let handshake = &handshake_event.handshake;

    // set connection state to handshake
    let mut connection_state = state
        .universe
        .get_mut::<ConnectionState>(handshake_event.conn_id)?;

    let next_state = handshake.next_state.val as u8;
    *connection_state = match next_state {
        1 => ConnectionState::Status,
        2 => ConnectionState::Login,
        s => return Err(Packet(PacketError::InvalidState(s))),
    };

    Ok(handshake_event)
}

const EXAMPLE_JSON: &str = r#"{
    "version": {
        "name": "1.19.4",
        "protocol": 762
    },
    "players": {
        "max": 100,
        "online": 5,
        "sample": [
            {
                "name": "thinkofdeath",
                "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
            }
        ]
    },
    "description": {
        "text": "Hello, world!"
    },
    "favicon": "data:image/png;base64,<data>",
    "enforcesSecureChat": false
}"#;
