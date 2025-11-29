use crate::ids;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::STATUS_CLIENTBOUND_PONG_RESPONSE, state = "status")]
pub struct PongPacket {
    pub payload: i64,
}

impl PongPacket {
    pub fn new(payload: i64) -> Self {
        Self { payload }
    }
}
