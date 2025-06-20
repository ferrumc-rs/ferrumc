use ferrumc_macros::{NetDecode, packet};

#[derive(Debug, NetDecode)]
#[packet(packet_id = "login_acknowledged", state = "login")]
pub struct LoginAcknowledgedPacket {}
