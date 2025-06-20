use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(packet_id = "move_player_pos", state = "play")]
pub struct SetPlayerPositionPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub on_ground: bool,
}
