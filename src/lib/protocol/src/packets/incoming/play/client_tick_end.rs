use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::ids;

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_CLIENT_TICK_END, state = "play")]
pub struct ClientTickEndPacket;
