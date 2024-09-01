use crate::utils::components::rotation::Rotation;
use crate::utils::encoding::position::Position;
use ferrumc_codec::network_types::varint::VarInt;
use ferrumc_macros::NetEncode;

#[derive(NetEncode)]
pub struct SynchronizePlayerPosition {
    #[encode(default = VarInt::from(0x3C))]
    pub packet_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
    pub teleport_id: VarInt,
}

impl SynchronizePlayerPosition {
    pub fn new(position: &Position, rotation: &Rotation) -> Self {
        Self {
            packet_id: VarInt::from(0x3C),
            x: position.x as f64,
            y: position.y as f64,
            z: position.z as f64,
            yaw: rotation.yaw,
            pitch: rotation.pitch,
            flags: 0, // Absolute position & rotation
            teleport_id: VarInt::from(0),
        }
    }
}
