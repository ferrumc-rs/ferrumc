use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;

#[derive(NetDecode)]
#[packet(packet_id = "hello", state = "login")]
pub struct EncryptionRequest {
    server_id: String,
    public_key: LengthPrefixedVec<u8>,
    verify_token: LengthPrefixedVec<u8>,
    should_authenticate: bool,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_request() {
        todo!()
    }
}