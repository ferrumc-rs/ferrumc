use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "set_default_spawn_position", state = "play")]
pub struct SetDefaultSpawnPositionPacket {
    pub spawn_position: NetworkPosition,
    pub angle: f32,
}

pub const DEFAULT_SPAWN_POSITION: NetworkPosition = NetworkPosition { x: 0, y: 100, z: 0 };

const DEFAULT_ANGLE: f32 = 0.0;

impl Default for SetDefaultSpawnPositionPacket {
    fn default() -> Self {
        Self::new()
    }
}

impl SetDefaultSpawnPositionPacket {
    pub fn new() -> Self {
        Self {
            spawn_position: DEFAULT_SPAWN_POSITION,
            angle: DEFAULT_ANGLE,
        }
    }
}
