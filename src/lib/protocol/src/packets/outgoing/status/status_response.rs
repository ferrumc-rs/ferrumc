use crate::ids;
use ferrumc_macros::NetEncode;
use ferrumc_macros::packet;

#[derive(NetEncode)]
#[packet(id = ids::STATUS_CLIENTBOUND_STATUS_RESPONSE, state = "status")]
pub struct StatusResponse {
    pub json_response: String,
}

impl StatusResponse {
    pub fn new(json_response: String) -> Self {
        Self { json_response }
    }
}
