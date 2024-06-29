use ferrumc_macros::Encode;
use ferrumc_utils::encoding::position::Position;
use ferrumc_utils::encoding::varint::VarInt;

#[derive(Encode)]
pub struct DefaultSpawnPosition {
    pub packet_id: VarInt,
    pub location: Position,
    pub angle: f32
}