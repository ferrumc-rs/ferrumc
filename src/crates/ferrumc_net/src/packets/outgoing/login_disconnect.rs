use ferrumc_macros::Encode;
use ferrumc_utils::encoding::varint::VarInt;

#[derive(Encode)]
pub struct LoginDisconnect {
    pub packet_id: VarInt,
    pub reason: String,
    #[encode(raw_bytes(prepend_length=true))]
    pub some_list: Vec<u8>
}