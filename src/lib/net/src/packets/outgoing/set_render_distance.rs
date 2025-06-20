use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "set_chunk_cache_radius", state = "play")]
pub struct SetRenderDistance {
    pub distance: VarInt,
}

const DEFAULT_RENDER_DISTANCE: u8 = 5;

impl Default for SetRenderDistance {
    fn default() -> Self {
        Self::new(DEFAULT_RENDER_DISTANCE)
    }
}

impl SetRenderDistance {
    pub fn new(distance: u8) -> Self {
        Self {
            distance: VarInt::new(distance as i32),
        }
    }
}
