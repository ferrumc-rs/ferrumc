use ferrumc_macros::NetEncode;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;
use tokio::io::AsyncWriteExt;

#[derive(NetEncode)]
pub struct FinishConfigurationPacket {
    // 0x03
    pub packet_id: VarInt,
}

impl Default for FinishConfigurationPacket {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FinishConfigurationPacket {
    fn default() -> Self {
        Self::new()
    }
}

impl FinishConfigurationPacket {
    pub fn new() -> Self {
        Self {
            packet_id: VarInt::new(0x03),
        }
    }
}
