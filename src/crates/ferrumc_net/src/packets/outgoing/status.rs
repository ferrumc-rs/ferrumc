use ferrumc_macros::Encode;
use ferrumc_utils::encoding::varint::VarInt;

#[derive(Encode)]
pub struct OutgoingStatusResponse {
    pub packet_id: VarInt,
    pub json_response: String,
}