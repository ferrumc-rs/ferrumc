use crate::codec::net_types::var_int::VarInt;
use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_SWING, state = "play")]
pub struct SwingArmPacket {
    pub hand: VarInt,
}
