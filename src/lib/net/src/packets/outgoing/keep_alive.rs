use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode, Clone)]
#[packet(packet_id = 0x26)]
pub struct OutgoingKeepAlive {
    pub timestamp: i64,
}

impl Default for OutgoingKeepAlive {
    fn default() -> Self {
        let current_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards?? LMAO")
            .as_millis() as i64;
        Self::new(current_ms)
    }
}

impl OutgoingKeepAlive {
    pub fn new(timestamp: i64) -> Self {
        Self { timestamp }
    }
}
