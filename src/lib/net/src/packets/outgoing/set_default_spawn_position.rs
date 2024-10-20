use ferrumc_macros::NetEncode;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
pub struct SetDefaultSpawnPositionPacket {
    pub packet_id: VarInt, // = 0x56
    pub spawn_position: NetworkPosition,
    pub angle: f32
}

pub const DEFAULT_SPAWN_POSITION: NetworkPosition = NetworkPosition {
    x: 0,
    y: 64,
    z: 0
};

const DEFAULT_ANGLE: f32 = 0.0;


impl Default for SetDefaultSpawnPositionPacket {
    fn default() -> Self {
        Self::new()
    }
}

impl SetDefaultSpawnPositionPacket {
    pub fn new() -> Self {
        Self {
            packet_id: VarInt::new(0x56),
            spawn_position: DEFAULT_SPAWN_POSITION,
            angle: DEFAULT_ANGLE
        }
    }
}