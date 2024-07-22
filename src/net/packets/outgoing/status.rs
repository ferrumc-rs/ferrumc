use ferrumc_macros::Encode;

use crate::utils::encoding::varint::VarInt;

/// The outgoing status response packet is sent by the server to the client to respond to a status request.
/// Contains the JSON response.
#[derive(Encode)]
pub struct OutgoingStatusResponse {
    pub packet_id: VarInt,
    pub json_response: String,
}
