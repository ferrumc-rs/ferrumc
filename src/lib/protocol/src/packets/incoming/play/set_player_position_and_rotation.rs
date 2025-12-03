use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_MOVE_PLAYER_POS_ROT, state = "play")]
pub struct SetPlayerPositionAndRotationPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i8, // Bit field: 0x01: on ground, 0x02: pushing against wall.
}
