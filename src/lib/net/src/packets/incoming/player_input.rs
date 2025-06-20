use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(packet_id = "player_input", state = "play")]
pub struct PlayerInput {
    pub flags: u8,
}
