use ferrumc_codec::network_types::varint::VarInt;
use ferrumc_macros::NetEncode;

/// Sent by the server to enable compression for each subsequent packet.
/// Packets that are greater or equal to the threshold will be compressed with zlib.
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
