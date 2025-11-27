use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_protocol::ids;

#[derive(NetDecode)]
#[packet(id = ids::LOGIN_SERVERBOUND_KEY, state = "login")]
pub struct EncryptionResponse {
    pub shared_secret: LengthPrefixedVec<u8>,
    pub verify_token: LengthPrefixedVec<u8>,
}
