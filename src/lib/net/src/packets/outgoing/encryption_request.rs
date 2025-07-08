use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "hello", state = "login")]
pub struct EncryptionRequestPacket {
    pub server_id: String,
    pub public_key: LengthPrefixedVec<u8>,
    pub verify_token: LengthPrefixedVec<u8>,
    pub should_authenticate: bool,
}
