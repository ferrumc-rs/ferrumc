use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

/// Client-to-Server packet to request a gamemode change. (f3+f4)
#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_CHANGE_GAME_MODE, state = "play")]
pub struct ChangeGameMode {
    /// 0: Survival, 1: Creative, 2: Adventure, 3: Spectator
    pub gamemode: VarInt,
}
