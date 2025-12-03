use ferrumc_macros::{NetDecode, packet};

#[derive(Debug, NetDecode)]
#[packet(id = ids::LOGIN_SERVERBOUND_HELLO, state = "login")]
pub struct LoginStartPacket {
    pub username: String,
    pub uuid: u128,
}
