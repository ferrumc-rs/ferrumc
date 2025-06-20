use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(packet_id = "move_player_rot", state = "play")]
pub struct SetPlayerRotationPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i8,
}
