use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;

#[derive(NetDecode)]
#[packet(packet_id = "key", state = "login")]
pub struct EncryptionResponse {
    shared_secret: LengthPrefixedVec<u8>,
    verify_token: LengthPrefixedVec<u8>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_decode() {
        todo!()
    }
}