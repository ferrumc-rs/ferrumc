use ferrumc_macros::{packet, NetDecode};

#[derive(Debug, NetDecode)]
#[packet(packet_id = "hello", state = "login")]
pub struct LoginStartPacket {
    pub username: String,
    pub uuid: u128,
}
