use crate::codec::net_types::var_int::VarInt;
use crate::ids;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::LOGIN_CLIENTBOUND_LOGIN_COMPRESSION, state = "login")]
pub struct SetCompressionPacket {
    pub threshold: VarInt,
}
