use ferrumc_macros::{packet, NetEncode};
use ferrumc_nbt::NBT;
use ferrumc_text::TextComponent;

#[derive(NetEncode, Debug, Clone)]
#[packet(packet_id = "system_chat", state = "play")]
pub struct SystemMessagePacket {
    pub message: NBT<TextComponent>,
    pub overlay: bool,
}

impl SystemMessagePacket {
    pub fn new(message: TextComponent, overlay: bool) -> Self {
        Self {
            message: NBT::new(message),
            overlay,
        }
    }
}

impl From<TextComponent> for SystemMessagePacket {
    fn from(value: TextComponent) -> Self {
        Self::new(value, false)
    }
}