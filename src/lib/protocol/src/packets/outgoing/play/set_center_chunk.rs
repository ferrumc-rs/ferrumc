use crate::codec::net_types::var_int::VarInt;
use crate::ids;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_SET_CHUNK_CACHE_CENTER, state = "play")]
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
