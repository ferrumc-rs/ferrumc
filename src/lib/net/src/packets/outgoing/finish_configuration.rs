use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
pub struct FinishConfigurationPacket {
    // 0x03
    pub packet_id: VarInt
}

impl FinishConfigurationPacket {
    pub fn new() -> Self {
        Self {
            packet_id: VarInt::new(0x03)
        }
    }
}