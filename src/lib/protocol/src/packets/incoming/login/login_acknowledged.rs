use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::ids;

#[derive(Debug, NetDecode)]
#[packet(id = ids::LOGIN_SERVERBOUND_LOGIN_ACKNOWLEDGED, state = "login")]
pub struct LoginAcknowledgedPacket {}
