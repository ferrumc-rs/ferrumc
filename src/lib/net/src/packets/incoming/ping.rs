use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode, Debug)]
#[packet(packet_id = "ping_request", state = "status")]
pub struct PingPacket {
    pub payload: i64,
}
