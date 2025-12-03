use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_PLAYER_LOADED, state = "play")]
pub struct PlayerLoaded;
