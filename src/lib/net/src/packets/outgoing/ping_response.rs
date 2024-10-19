use ferrumc_macros::NetEncode;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
pub struct PongPacket {
    pub packet_id: VarInt,
    pub payload: i64,
}

impl PongPacket {
    pub fn new(payload: i64) -> Self {
        Self {
            packet_id: VarInt::from(0x01),
            payload,
        }
    }
}
