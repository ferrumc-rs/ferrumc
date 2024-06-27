use ferrumc_macros::Encode;

#[derive(Encode)]
pub struct EncryptionBegin {
    pub packet_id: VarInt,
    pub server_id: String,
    // Public key len is encoded within
    pub public_key: Vec<u8>,
    // Verify token len is encoded within
    pub verify_token: Vec<u8>,
    pub should_auth: bool
}