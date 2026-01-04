use ferrumc_macros::{packet, NetEncode};
#[derive(NetEncode)]
#[packet(packet_id = "forget_level_chunk", state = "play")]
/// For some godforsaken reason, the x and z are backwards in this packet.
pub struct UnloadChunk {
    pub z: i32,
    pub x: i32,
}
