use std::io::Write;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
#[packet(packet_id = 0x54)]
pub struct SetCenterChunk {
    pub x: VarInt,
    pub z: VarInt,
}