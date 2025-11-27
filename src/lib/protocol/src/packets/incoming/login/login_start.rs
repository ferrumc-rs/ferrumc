use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::ids;

#[derive(Debug, NetDecode)]
#[packet(id = ids::LOGIN_SERVERBOUND_HELLO, state = "login")]
pub struct LoginStartPacket {
    pub username: String,
    pub uuid: u128,
}
