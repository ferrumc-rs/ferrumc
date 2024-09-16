use ferrumc_codec::network_types::varint::VarInt;

use ferrumc_macros::NetEncode;

/// The outgoing ping packet is sent by the server to the client to check the connection.
/// Payload is just the same as whatever the client sent.
#[derive(NetEncode)]
pub struct OutgoingPing {
    #[encode(default = VarInt::from(0x32))]
    pub packet_id: VarInt,
    pub payload: i64,
}
