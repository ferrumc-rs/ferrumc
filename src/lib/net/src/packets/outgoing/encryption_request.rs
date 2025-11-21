use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;

#[derive(NetEncode)]
#[packet(packet_id = "hello", state = "login")]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: LengthPrefixedVec<u8>,
    pub verify_token: LengthPrefixedVec<u8>,
    pub should_authenticate: bool,
}
