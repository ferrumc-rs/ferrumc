use std::io::Read;
use std::sync::Arc;
use tracing::trace;
use ferrumc_macros::{packet, Event};
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use ferrumc_net_codec::net_types::var_int::VarInt;
use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_state::ServerState;
use std::fmt::Debug;
use ferrumc_events::infrastructure::Event;

/// This event triggers when a [LoginPluginResponse] is received.
///
#[derive(Event, Debug)]
pub struct LoginPluginResponseEvent {
    /// The entity that the event was triggered for
    pub entity: usize,
    /// The [LoginPluginResponse] packet received.
    pub packet: LoginPluginResponse,
}

#[derive(Debug)]
#[packet(packet_id = 0x02, state = "configuration")]
pub struct ServerBoundPluginMessage {
    pub channel: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
#[packet(packet_id = 0x02, state = "login")]
pub struct LoginPluginResponse {
    pub message_id: VarInt,
    pub success: bool,
    pub data: Vec<u8>,
}

pub struct ClientMinecraftBrand {
    pub brand: String
}

impl NetDecode for ServerBoundPluginMessage {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let channel = <String>::decode(reader, opts)?;
        let mut buf = Vec::<u8>::new();
        reader.read_to_end(&mut buf)?;

        Ok(Self {
            channel,
            data: buf
        })
    }
}

impl IncomingPacket for ServerBoundPluginMessage {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        trace!("Received plugin message: {:?}", self);

        if self.channel == "minecraft:brand" {
            let brand = String::from_utf8(self.data)?;
            trace!("Client brand: {}", brand);
            
            state.universe.add_component(conn_id, ClientMinecraftBrand { brand })?;
        }

        Ok(())
    }
}

impl NetDecode for LoginPluginResponse {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let message_id = <VarInt>::decode(reader, opts)?; 
        let success = <bool>::decode(reader, opts)?;

        let mut buf = Vec::<u8>::new();
        if success {
            reader.read_to_end(&mut buf)?;
        }

        Ok(Self {
            message_id,
            success,
            data: buf
        })
    }
}

impl IncomingPacket for LoginPluginResponse {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        LoginPluginResponseEvent::trigger(LoginPluginResponseEvent {
            entity: conn_id,
            packet: self,
        }, Arc::clone(&state)).await?;

        Ok(())
    }
}
