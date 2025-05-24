use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode)]
#[packet(packet_id = "move_player_pos_rot", state = "play")]
pub struct SetPlayerPositionAndRotationPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
