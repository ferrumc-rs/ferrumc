use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::ids;

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_PLAYER_LOADED, state = "play")]
pub struct PlayerLoaded;
