use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode, Debug)]
#[packet(packet_id = "status_request", state = "status")]
pub struct StatusRequestPacket {}
