use crate::connection::ConnectionState;
use crate::errors::NetError;
use crate::packets::IncomingPacket;
use crate::{GlobalState, NetResult, ServerState};
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{event_handler, packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::sync::Arc;
use tracing::info;

#[derive(NetDecode, Debug)]
#[packet(packet_id = 0x00, state = "handshake")]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

impl IncomingPacket for Handshake {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        info!("Connection ID: {}", conn_id);
        info!("Handshake packet received: {:?}", self);

        let current_state = state
            .universe
            .get::<ConnectionState>(conn_id)?;

        info!("Current state: {}", current_state.as_str());


        Handshake::trigger(self, state).await?;

        Ok(())
    }
}


impl Event for Handshake {
    type Data = Self;
    type State = GlobalState;
    type Error = NetError;

    fn name() -> &'static str {
        "handshake"
    }
}

#[event_handler]
async fn handle_handshake(
    handshake: Handshake,
    state: GlobalState,
) -> Result<Handshake, <Handshake as Event>::Error> {
    info!("Handling handshake event: {:?}", handshake);

    Ok(handshake)
}



#[cfg(test)]
mod tests {
    use ferrumc_macros::NetDecode;
    use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
    use ferrumc_net_codec::net_types::var_int::VarInt;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_macro_decode() {
        #[derive(NetDecode, Default)]
        #[allow(unused)]
        struct Handshake {
            protocol_version: VarInt,
            server_address: String,
            server_port: u16,
            next_state: VarInt,
        }
        let mut data = Cursor::new(vec![
            255, 5, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1
        ]);

        let handshake = Handshake::decode(&mut data, &NetDecodeOpts::None).unwrap();
        assert_eq!(handshake.protocol_version, VarInt::new(767));
        assert_eq!(handshake.server_address, "localhost".to_string());
        assert_eq!(handshake.server_port, 25565);
        assert_eq!(handshake.next_state, VarInt::new(1));
    }
}