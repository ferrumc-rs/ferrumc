use ferrumc_macros::{packet, NetEncode};
use ferrumc_text::TextComponent;

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
