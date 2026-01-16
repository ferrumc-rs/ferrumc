use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
#[packet(packet_id = "set_experience", state = "play")]
pub struct SetExperience {
    pub experience_bar: f32,
    pub level: VarInt,
    pub total_experience: VarInt,
}
