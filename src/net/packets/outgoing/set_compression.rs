use ferrumc_codec::network_types::varint::VarInt;
use ferrumc_macros::NetEncode;

#[derive(NetEncode)]
pub struct SetCompression {
    #[encode(default=VarInt::from(0x03))]
    pub packet_id: VarInt,
    pub threshold: VarInt, // Any packet larger than this will be compressed
}
impl SetCompression {
    pub fn new(threshold: i32) -> Self {
        Self::new_auto(threshold.into())
    }
}
