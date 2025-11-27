use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::ids;

#[derive(NetDecode, Debug)]
#[packet(id = ids::STATUS_SERVERBOUND_PING_REQUEST, state = "status")]
pub struct PingPacket {
    pub payload: i64,
}
