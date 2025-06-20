use ferrumc_macros::{NetDecode, packet};

#[derive(Debug, NetDecode)]
#[packet(packet_id = "hello", state = "login")]
pub struct LoginStartPacket {
    pub username: String,
    pub uuid: u128,
}
