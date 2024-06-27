use ferrumc_macros::Encode;

#[derive(Encode)]
pub struct LoginDisconnect {
    pub packet_id: VarInt,
    pub reason: String,
}