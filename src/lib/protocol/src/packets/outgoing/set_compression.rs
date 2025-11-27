use ferrumc_macros::{packet, NetEncode};
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

#[derive(NetEncode)]
#[packet(packet_id = "login_compression", state = "login")]
pub struct SetCompressionPacket {
    pub threshold: VarInt,
}
