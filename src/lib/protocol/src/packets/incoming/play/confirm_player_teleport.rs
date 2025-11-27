use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_ACCEPT_TELEPORTATION, state = "play")]
pub struct ConfirmPlayerTeleport {
    pub teleport_id: VarInt,
}
