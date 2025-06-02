use crate::packets::IncomingPacket;

use crate::errors::NetError;
use ferrumc_macros::packet;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use ferrumc_state::ServerState;
use std::io::Read;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tracing::debug;
use typename::TypeName;

#[derive(Debug)]
#[packet(packet_id = "custom_payload", state = "configuration")]
pub struct ServerBoundPluginMessage {
    _channel: String,
    _data: Vec<u8>,
}
#[derive(TypeName)]
pub struct ClientMinecraftBrand {
    pub brand: String,
}

impl NetDecode for ServerBoundPluginMessage {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let channel = <String>::decode(reader, opts)?;
        let mut buf = Vec::<u8>::new();
        reader.read_to_end(&mut buf)?;

        Ok(Self {
            _channel: channel,
            _data: buf,
        })
    }

    async fn decode_async<R: tokio::io::AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> NetDecodeResult<Self> {
        let channel = <String>::decode_async(reader, opts).await?;
        let mut buf = Vec::<u8>::new();
        reader.read_to_end(&mut buf).await?;

        Ok(Self {
            _channel: channel,
            _data: buf,
        })
    }
}
impl IncomingPacket for ServerBoundPluginMessage {
    fn handle(self, _: usize, _: Arc<ServerState>) -> Result<(), NetError> {
        debug!("Received plugin message: {:?}", self);

        Ok(())
    }
}
