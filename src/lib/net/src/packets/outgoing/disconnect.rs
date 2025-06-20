use ferrumc_macros::{NetEncode, packet};
use ferrumc_text::{ComponentBuilder, TextComponent};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "disconnect", state = "play")]
pub struct DisconnectPacket {
    pub reason: TextComponent,
}

impl DisconnectPacket {
    pub fn new(reason: TextComponent) -> Self {
        Self { reason }
    }
    pub fn from_string(reason: String) -> Self {
        let reason = ComponentBuilder::text(reason);
        Self {
            reason: reason.build(),
        }
    }
}

impl Default for DisconnectPacket {
    fn default() -> Self {
        Self::from_string("FERRUMC-DISCONNECTED".to_string())
    }
}
