use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "game_profile", state = "login")]
pub struct LoginSuccessPacket {
    pub identity: PlayerIdentity,
    pub strict_error_handling: bool,
}

impl LoginSuccessPacket {
    pub fn new(identity: PlayerIdentity) -> Self {
        Self {
            identity,
            strict_error_handling: false,
        }
    }
}
