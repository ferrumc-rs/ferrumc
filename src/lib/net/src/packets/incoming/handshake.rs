use crate::packets::IncomingPacket;

use crate::errors::NetError;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use std::sync::Arc;
use tracing::trace;

#[derive(NetDecode, Debug)]
#[packet(packet_id = "intention", state = "handshake")]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

impl IncomingPacket for Handshake {
    fn handle(self, conn_id: usize, _: Arc<ServerState>) -> Result<(), NetError> {
        trace!("Connection ID: {}", conn_id);
        trace!("Handshake packet received: {:?}", self);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ferrumc_macros::NetDecode;
    use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
    use ferrumc_net_codec::net_types::var_int::VarInt;
    use std::io::Cursor;

    #[test]
    fn test_macro_decode() {
        #[derive(NetDecode, Default)]
        struct Handshake {
            protocol_version: VarInt,
            server_address: String,
            server_port: u16,
            next_state: VarInt,
        }
        let mut data = Cursor::new(vec![
            255, 5, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1,
        ]);

        let handshake = Handshake::decode(&mut data, &NetDecodeOpts::None).unwrap();
        assert_eq!(handshake.protocol_version, VarInt::new(767));
        assert_eq!(handshake.server_address, "localhost".to_string());
        assert_eq!(handshake.server_port, 25565);
        assert_eq!(handshake.next_state, VarInt::new(1));
    }
}
