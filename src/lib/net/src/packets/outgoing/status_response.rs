use ferrumc_macros::{NetEncode, packet};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "status_response", state = "status")]
pub struct StatusResponse {
    pub json_response: String,
}

impl StatusResponse {
    pub fn new(json_response: String) -> Self {
        Self { json_response }
    }
}
