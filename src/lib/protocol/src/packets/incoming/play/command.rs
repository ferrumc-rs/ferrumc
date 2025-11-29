use crate::ids;
use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode, Debug, Clone)]
#[packet(id = ids::PLAY_SERVERBOUND_CHAT_COMMAND, state = "play")]
pub struct ChatCommandPacket {
    pub command: String,
}
