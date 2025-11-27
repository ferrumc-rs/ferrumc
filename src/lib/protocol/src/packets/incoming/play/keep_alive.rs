use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::ids;
use typename::TypeName;

#[derive(TypeName, NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_KEEP_ALIVE, state = "play")]
pub struct IncomingKeepAlivePacket {
    pub timestamp: i64,
}
