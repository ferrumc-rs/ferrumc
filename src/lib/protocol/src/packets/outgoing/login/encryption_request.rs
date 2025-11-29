use crate::codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use crate::ids;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::LOGIN_CLIENTBOUND_HELLO, state = "login")]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: LengthPrefixedVec<u8>,
    pub verify_token: LengthPrefixedVec<u8>,
    pub should_authenticate: bool,
}
