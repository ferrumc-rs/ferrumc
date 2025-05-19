use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetDecode)]
#[packet(packet_id = "swing", state = "play")]
pub struct SwingArmPacket {
    pub hand: VarInt,
}
