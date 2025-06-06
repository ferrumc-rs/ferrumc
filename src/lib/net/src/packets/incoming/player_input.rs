use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode)]
#[packet(packet_id = "player_input", state = "play")]
pub struct PlayerInput {
    pub flags: u8,
}

