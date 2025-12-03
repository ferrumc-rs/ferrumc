use crate::codec::net_types::network_position::NetworkPosition;
use crate::codec::net_types::var_int::VarInt;
use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_PLAYER_ACTION, state = "play")]
pub struct PlayerAction {
    pub status: VarInt,
    pub location: NetworkPosition,
    pub face: u8,
    pub sequence: VarInt,
}
