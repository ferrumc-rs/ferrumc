use crate::ids;
use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_CLIENT_TICK_END, state = "play")]
pub struct ClientTickEndPacket;
