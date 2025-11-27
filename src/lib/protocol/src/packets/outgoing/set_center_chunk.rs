use ferrumc_macros::{packet, NetEncode};
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

#[derive(NetEncode)]
#[packet(packet_id = "set_chunk_cache_center", state = "play")]
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
