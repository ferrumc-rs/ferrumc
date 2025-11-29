use crate::ids;
use ferrumc_macros::{NetEncode, packet};
use ferrumc_text::TextComponent;

#[derive(NetEncode)]
#[packet(id = ids::LOGIN_CLIENTBOUND_LOGIN_DISCONNECT, state = "login")]
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
