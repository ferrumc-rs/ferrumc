use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::ids;

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_MOVE_PLAYER_ROT, state = "play")]
pub struct SetPlayerRotationPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i8,
}
