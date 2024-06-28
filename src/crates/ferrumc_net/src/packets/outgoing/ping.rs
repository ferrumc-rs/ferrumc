use ferrumc_macros::Encode;
use ferrumc_utils::encoding::varint::VarInt;

#[derive(Encode)]
pub struct OutgoingPing {
    pub packet_id: VarInt,
    pub payload: i64,
}