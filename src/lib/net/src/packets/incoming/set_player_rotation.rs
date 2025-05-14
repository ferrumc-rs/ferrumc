use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode)]
#[packet(packet_id = "move_player_rot", state = "play")]
pub struct SetPlayerRotationPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

