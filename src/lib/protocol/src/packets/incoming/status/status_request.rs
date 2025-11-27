use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::ids;

#[derive(NetDecode, Debug)]
#[packet(id = ids::STATUS_SERVERBOUND_STATUS_REQUEST, state = "status")]
pub struct StatusRequestPacket {}
