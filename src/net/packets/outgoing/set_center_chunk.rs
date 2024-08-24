use ferrumc_codec::network_types::varint::VarInt;
use ferrumc_macros::Encode;

#[derive(Encode)]
pub struct SetCenterChunk {
    #[encode(default = VarInt::from(0x4E))]
    pub packet_id: VarInt,
    pub chunk_x: VarInt,
    pub chunk_z: VarInt,
}
impl SetCenterChunk {
    pub fn new(chunk_x: i32, chunk_z: i32) -> Self {
        Self::new_auto(chunk_x.into(), chunk_z.into())
    }
}