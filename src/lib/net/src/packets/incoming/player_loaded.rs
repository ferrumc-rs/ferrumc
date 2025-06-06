use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode)]
#[packet(packet_id = "player_loaded", state = "play")]
pub struct PlayerLoaded;
