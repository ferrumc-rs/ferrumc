use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::ids;

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_PLAYER_INPUT, state = "play")]
pub struct PlayerInput {
    pub flags: u8,
}
