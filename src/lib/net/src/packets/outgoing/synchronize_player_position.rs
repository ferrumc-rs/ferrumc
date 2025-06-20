use crate::packets::outgoing::set_default_spawn_position::DEFAULT_SPAWN_POSITION;
use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::teleport_flags::TeleportFlags;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "player_position", state = "play")]
pub struct SynchronizePlayerPositionPacket {
    pub teleport_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: TeleportFlags,
}

impl Default for SynchronizePlayerPositionPacket {
    fn default() -> Self {
        let default_pos = DEFAULT_SPAWN_POSITION;
        Self::new(
            (
                default_pos.x as f64,
                default_pos.y as f64,
                default_pos.z as f64,
            ),
            (0.0, 0.0, 0.0),
            0.0,
            0.0,
            0,
            VarInt::new(0),
        )
    }
}

impl SynchronizePlayerPositionPacket {
    pub fn new(
        xyz: (f64, f64, f64),
        vel: (f64, f64, f64),
        yaw: f32,
        pitch: f32,
        flags: i32,
        teleport_id: VarInt,
    ) -> Self {
        Self {
            x: xyz.0,
            y: xyz.1,
            z: xyz.2,
            vel_x: vel.0,
            vel_y: vel.1,
            vel_z: vel.2,
            yaw,
            pitch,
            flags,
            teleport_id,
        }
    }
}
