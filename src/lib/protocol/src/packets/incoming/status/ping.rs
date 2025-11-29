use crate::ids;
use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode, Debug)]
#[packet(id = ids::STATUS_SERVERBOUND_PING_REQUEST, state = "status")]
pub struct PingPacket {
    pub payload: i64,
}
