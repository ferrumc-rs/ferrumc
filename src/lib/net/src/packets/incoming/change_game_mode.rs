use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Client-to-Server packet to request a gamemode change. (f3+f4)
#[derive(NetDecode)]
#[packet(packet_id = "change_game_mode", state = "play")]
pub struct ChangeGameMode {
    /// 0: Survival, 1: Creative, 2: Adventure, 3: Spectator
    pub gamemode: VarInt,
}
