use ferrumc_macros::{NetEncode, packet};
use ferrumc_text::TextComponent;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "login_disconnect", state = "login")]
pub struct LoginDisconnectPacket {
    pub reason: String,
}

impl LoginDisconnectPacket {
    pub fn new(reason: impl Into<TextComponent>) -> Self {
        Self {
            reason: reason.into().to_string(),
        }
    }
}
