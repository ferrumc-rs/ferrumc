use ferrumc_macros::{packet, NetEncode};
use ferrumc_text::{ComponentBuilder, TextComponent};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x1D)]
pub struct Disconnect {
    pub reason: TextComponent,
}

impl Disconnect {
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

impl Default for Disconnect {
    fn default() -> Self {
        Self::from_string("Disconnected".to_string())
    }
}
