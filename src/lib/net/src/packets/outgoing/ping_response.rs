use ferrumc_macros::{NetEncode, packet};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "pong_response", state = "status")]
pub struct PongPacket {
    pub payload: i64,
}

impl PongPacket {
    pub fn new(payload: i64) -> Self {
        Self { payload }
    }
}
