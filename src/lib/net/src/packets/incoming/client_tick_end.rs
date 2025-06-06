use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode)]
#[packet(packet_id = "client_tick_end", state = "play")]
pub struct ClientTickEndPacket;
