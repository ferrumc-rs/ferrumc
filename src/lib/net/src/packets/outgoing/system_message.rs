use ferrumc_macros::{packet, NetEncode};
use ferrumc_text::TextComponent;

#[derive(NetEncode, Debug, Clone)]
#[packet(packet_id = "system_chat", state = "play")]
pub struct SystemMessagePacket {
    pub message: TextComponent,
    pub overlay: bool,
}
