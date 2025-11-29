use crate::codec::net_types::network_position::NetworkPosition;
use crate::ids;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(ud = ids::PLAY_CLIENTBOUND_SET_DEFAULT_SPAWN_POSITION, state = "play")]
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
