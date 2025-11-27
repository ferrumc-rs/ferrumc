use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::ids;

#[derive(NetDecode, Debug, Clone)]
#[packet(id = ids::PLAY_SERVERBOUND_CHAT_COMMAND, state = "play")]
pub struct ChatCommandPacket {
    pub command: String,
}
