use ferrumc_codec::enc::NetEncode;
use ferrumc_macros::NetEncode;
use crate::Result;

#[derive(Debug, NetEncode)]
pub struct PacketQueue {
    queue: Vec<u8>,
}

impl PacketQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    /// Queue a packet to be sent.
    pub async fn queue(&mut self, packet: impl NetEncode) -> Result<()> {
        packet.net_encode(&mut self.queue).await.map_err(Into::into)
    }
}