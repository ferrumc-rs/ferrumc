use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode, Debug)]
#[packet(id = ids::STATUS_SERVERBOUND_STATUS_REQUEST, state = "status")]
pub struct StatusRequestPacket {}
