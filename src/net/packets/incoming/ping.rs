use ferrumc_codec::network_types::varint::VarInt;
use tracing::debug;

use ferrumc_macros::{packet, NetDecode};

use crate::net::packets::outgoing::ping::OutgoingPing;
use crate::net::packets::{ConnectionId, IncomingPacket};
use crate::state::GlobalState;
use crate::utils::prelude::*;

/// This ping packet is sent by the client to the server to request a pong.
///
/// The payload is a random number that the server should return in the pong.
/// For some reason, seems to be required for the client to acknowledge the server's status response.
#[derive(NetDecode)]
#[packet(packet_id = 0x01, state = "status")]
pub struct Ping {
    pub payload: i64,
}

impl IncomingPacket for Ping {
    async fn handle(self, conn_id: ConnectionId, state: GlobalState) -> Result<()> {
        debug!("Handling ping packet");

        // tokio::io::AsyncWriteExt::write_all()
        let response = OutgoingPing {
            packet_id: VarInt::from(0x01),
            payload: self.payload,
        };

        /*let mut cursor = std::io::Cursor::new(Vec::new());
                response.net_encode(&mut cursor).await?;
                let response = cursor.into_inner();
        */
        let conn = state.connections.get_connection(conn_id)?;
        let mut conn = conn.write().await;

        conn.drop = true;

        /* conn.socket
        .write_all(&*response)
        .await
        .map_err(|e| e.into())*/

        conn.send_packet(response).await
    }
}
