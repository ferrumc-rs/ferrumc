use ferrumc_macros::{NetDecode, packet};
use typename::TypeName;

#[derive(TypeName, NetDecode)]
#[packet(packet_id = "keep_alive", state = "play")]
pub struct IncomingKeepAlivePacket {
    pub timestamp: i64,
}
