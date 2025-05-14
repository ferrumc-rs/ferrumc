use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetDecode)]
#[packet(packet_id = "player_action", state = "play")]
pub struct PlayerAction {
    pub status: VarInt,
    pub location: NetworkPosition,
    pub face: u8,
    pub sequence: VarInt,
}
