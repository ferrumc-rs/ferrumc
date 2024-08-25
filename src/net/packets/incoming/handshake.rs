use ferrumc_macros::{packet, Decode};

use crate::net::packets::{ConnectionId, IncomingPacket};
use crate::state::GlobalState;
use crate::utils::prelude::*;
use crate::{net::State, Connection};
use ferrumc_codec::network_types::varint::VarInt;

/// The first packet sent by the client to the server.
///
/// This packet is used to negotiate the protocol version, server address, server port, and the next state.
#[derive(Decode)]
#[packet(packet_id = 0x00, state = "handshake")]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

impl IncomingPacket for Handshake {
    async fn handle(self, conn_id: ConnectionId, state: GlobalState) -> Result<()> {
        let Some(mut conn) = state.connections.connections.get(&conn_id) else {
            return Err(Error::ConnectionNotFound(conn_id));
        };

        let mut conn = conn.write().await;

        conn.metadata.protocol_version = self.protocol_version.get_val();
        conn.state = match self.next_state.get_val() {
            1 => State::Status,
            2 => State::Login,
            s => return Err(Error::InvalidState(s)),
        };

        Ok(())
    }
}
