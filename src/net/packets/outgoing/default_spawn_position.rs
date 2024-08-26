use ferrumc_codec::network_types::varint::VarInt;

use ferrumc_macros::NetEncode;

use crate::utils::encoding::position::Position;

/// The default spawn position packet is sent by the server to the client to set the player's spawn position.
#[derive(NetEncode)]
pub struct DefaultSpawnPosition {
    #[encode(default = VarInt::from(0x50))]
    pub packet_id: VarInt,
    pub location: Position,
    pub angle: f32,
}
