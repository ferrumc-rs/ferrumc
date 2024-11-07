use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x26)]
pub struct OutgoingKeepAlivePacket {
    pub id: i64,
}

impl Default for OutgoingKeepAlivePacket {
    fn default() -> Self {
        let current_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards?? LMAO")
            .as_millis() as i64;
        Self { id: current_ms }
    }
}
