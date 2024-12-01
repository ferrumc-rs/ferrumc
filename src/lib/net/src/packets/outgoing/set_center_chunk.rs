use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x54)]
pub struct SetCenterChunk {
    pub x: VarInt,
    pub z: VarInt,
}

impl SetCenterChunk {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x: VarInt::new(x),
            z: VarInt::new(z),
        }
    }
}
