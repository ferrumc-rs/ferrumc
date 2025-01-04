use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x0D, state = "play")]
pub struct ChunkBatchStart {}
