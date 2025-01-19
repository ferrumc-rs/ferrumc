use ferrumc_macros::{packet, NetEncode};
use ferrumc_text::TextComponent;
use std::io::Write;

#[derive(NetEncode, Debug, Clone)]
#[packet(packet_id = "system_chat", state = "play")]
pub struct SystemMessagePacket {
    message: TextComponent,
    overlay: bool,
}

impl SystemMessagePacket {
    pub fn new(message: TextComponent, overlay: bool) -> Self {
        Self { message, overlay }
    }
}
