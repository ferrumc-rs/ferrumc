use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(packet_id = "player_loaded", state = "play")]
pub struct PlayerLoaded;
