use ferrumc_macros::{NetDecode, packet};
use ferrumc_protocol::codec::net_types::network_position::NetworkPosition;
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_PLAYER_ACTION, state = "play")]
pub struct PlayerAction {
    pub status: VarInt,
    pub location: NetworkPosition,
    pub face: u8,
    pub sequence: VarInt,
}
