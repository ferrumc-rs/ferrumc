use crate::codec::net_types::var_int::VarInt;
use crate::ids;
use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_ACCEPT_TELEPORTATION, state = "play")]
pub struct ConfirmPlayerTeleport {
    pub teleport_id: VarInt,
}
