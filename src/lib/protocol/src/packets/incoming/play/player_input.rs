use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_PLAYER_INPUT, state = "play")]
pub struct PlayerInput {
    pub flags: u8,
}
