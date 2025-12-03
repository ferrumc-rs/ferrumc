use ferrumc_macros::{NetDecode, packet};

#[derive(Debug, NetDecode)]
#[packet(id = ids::LOGIN_SERVERBOUND_LOGIN_ACKNOWLEDGED, state = "login")]
pub struct LoginAcknowledgedPacket {}
