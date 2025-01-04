use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode, Clone)]
#[packet(packet_id = "keep_alive", state = "play")]
pub struct OutgoingKeepAlivePacket {
    pub timestamp: i64,
}

impl Default for OutgoingKeepAlivePacket {
    fn default() -> Self {
        let current_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards?? LMAO")
            .as_millis() as i64;
        Self::new(current_ms)
    }
}

impl OutgoingKeepAlivePacket {
    pub fn new(timestamp: i64) -> Self {
        Self { timestamp }
    }
}
