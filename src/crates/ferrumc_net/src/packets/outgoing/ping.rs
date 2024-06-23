use ferrumc_macros::Encode;

#[derive(Encode)]
pub struct OutgoingPing {
    pub packet_id: VarInt,
    pub payload: i64,
}