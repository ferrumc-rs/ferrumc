use ferrumc_codec::network_types::varint::VarInt;

use ferrumc_macros::NetEncode;

/// The outgoing status response packet is sent by the server to the client to respond to a status request.
/// Contains the JSON response.
#[derive(NetEncode)]
pub struct OutgoingStatusResponse {
    #[encode(default = VarInt::from(0x00))]
    pub packet_id: VarInt,
    pub json_response: String,
}
