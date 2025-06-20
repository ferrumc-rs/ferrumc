use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(packet_id = "client_tick_end", state = "play")]
pub struct ClientTickEndPacket;
