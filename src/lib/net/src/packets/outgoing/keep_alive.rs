use ferrumc_macros::{packet, NetEncode};
use std::io::Write;
use tokio::io::AsyncWriteExt;

#[derive(Debug, NetEncode)]
pub struct KeepAlive {
    pub id: i64,
}

mod adapters {
    impl From<i64> for super::KeepAlive {
        fn from(id: i64) -> Self {
            Self { id }
        }
    }
}

#[derive(NetEncode)]
#[packet(packet_id = 0x26)]
pub struct KeepAlivePacket {
    pub id: KeepAlive,
}

impl Default for KeepAlivePacket {
    fn default() -> Self {
        let current_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards?? LMAO")
            .as_millis() as i64;
        Self::new(current_ms)
    }
}

impl KeepAlivePacket {
    pub fn new(id: i64) -> Self {
        Self {
            id: KeepAlive::from(id),
        }
    }
}
