use bevy_math::IVec2;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
#[packet(packet_id = "set_chunk_cache_center", state = "play")]
pub struct SetCenterChunk {
    pub x: VarInt,
    pub z: VarInt,
}

impl SetCenterChunk {
    pub fn new(pos: IVec2) -> Self {
        Self {
            x: VarInt::new(pos.x),
            z: VarInt::new(pos.y),
        }
    }
}
