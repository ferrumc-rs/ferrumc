use ferrumc_macros::{packet, NetDecode};

#[derive(Debug, NetDecode)]
#[packet(packet_id = "login_acknowledged", state = "login")]
pub struct LoginAcknowledgedPacket {}
