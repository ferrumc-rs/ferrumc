use ferrumc_macros::{packet, NetEncode};
use ferrumc_nbt::NBT;
use ferrumc_text::{ComponentBuilder, TextComponent};

#[derive(NetEncode)]
#[packet(packet_id = "disconnect", state = "play")]
pub struct DisconnectPacket {
    pub reason: NBT<TextComponent>,
}

impl DisconnectPacket {
    pub fn new(reason: TextComponent) -> Self {
        Self { reason: NBT::new(reason) }
    }
    pub fn from_string(reason: String) -> Self {
        let reason = ComponentBuilder::text(reason);
        Self {
            reason: NBT::new(reason.build()),
        }
    }
}

impl Default for DisconnectPacket {
    fn default() -> Self {
        Self::from_string("FERRUMC-DISCONNECTED".to_string())
    }
}
