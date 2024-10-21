use crate::packets::outgoing::set_default_spawn_position::DEFAULT_SPAWN_POSITION;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;
use tokio::io::AsyncWriteExt;

#[derive(NetEncode)]
#[packet(packet_id = 0x40)]
pub struct SynchronizePlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i8,
    pub teleport_id: VarInt,
}

impl Default for SynchronizePlayerPositionPacket {
    fn default() -> Self {
        let default_pos = DEFAULT_SPAWN_POSITION;
        Self::new(
            default_pos.x as f64,
            default_pos.y as f64,
            default_pos.z as f64,
            0.0,
            0.0,
            0,
            VarInt::new(0),
        )
    }
}

impl SynchronizePlayerPositionPacket {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        flags: i8,
        teleport_id: VarInt,
    ) -> Self {
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
            flags,
            teleport_id,
        }
    }
}
