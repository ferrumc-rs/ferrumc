use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x01, state = "status")]
pub struct PongPacket {
    pub payload: i64,
}

impl PongPacket {
    pub fn new(payload: i64) -> Self {
        Self { payload }
    }
}
