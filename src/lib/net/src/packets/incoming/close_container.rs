use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetDecode)]
#[packet(packet_id = "container_close", state = "play")]
pub struct CloseContainer {
    pub window_id: VarInt,
}
