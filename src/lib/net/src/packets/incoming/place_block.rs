use ferrumc_macros::{NetDecode, packet};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetDecode, Debug)]
#[packet(packet_id = "use_item_on", state = "play")]
pub struct PlaceBlock {
    pub hand: VarInt,
    pub position: NetworkPosition,
    pub face: VarInt,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub cursor_z: f32,
    pub inside_block: bool,
    pub world_border_hit: bool,
    pub sequence: VarInt,
}
