use ferrumc_macros::NetEncode;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
pub struct OutgoingStatusResponse {
    // #[encode(default = VarInt::from(0x00))]
    pub packet_id: VarInt,
    pub json_response: String,
}


impl OutgoingStatusResponse {
    pub fn new(json_response: String) -> Self {
        Self {
            packet_id: VarInt::from(0x00),
            json_response,
        }
    }
}