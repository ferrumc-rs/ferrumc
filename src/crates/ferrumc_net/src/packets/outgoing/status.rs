use ferrumc_macros::Encode;

#[derive(Encode)]
pub struct OutgoingStatusResponse {
    pub packet_id: VarInt,
    pub json_response: String,
}