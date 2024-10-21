use ferrumc_macros::{packet, NetEncode};
use std::io::Write;
use tokio::io::AsyncWriteExt;

#[derive(NetEncode)]
#[packet(packet_id = 0x00)]
pub struct StatusResponse {
    pub json_response: String,
}

impl StatusResponse {
    pub fn new(json_response: String) -> Self {
        Self { json_response }
    }
}
