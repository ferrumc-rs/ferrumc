use crate::ids;
use ferrumc_macros::{NetEncode, packet};
use ferrumc_text::TextComponent;

#[derive(NetEncode, Debug, Clone)]
#[packet(id = ids::PLAY_CLIENTBOUND_SYSTEM_CHAT, state = "play")]
pub struct SystemMessagePacket {
    pub message: TextComponent,
    pub overlay: bool,
}
