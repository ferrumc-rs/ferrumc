use ferrumc_macros::Encode;
use ferrumc_utils::encoding::position::Position;
use ferrumc_utils::encoding::varint::VarInt;

/// The default spawn position packet is sent by the server to the client to set the player's spawn position.
#[derive(Encode)]
pub struct DefaultSpawnPosition {
    pub packet_id: VarInt,
    pub location: Position,
    pub angle: f32
}