use ferrumc_macros::{NetDecode, packet};
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_SWING, state = "play")]
pub struct SwingArmPacket {
    pub hand: VarInt,
}
