use ferrumc_macros::Encode;

use crate::utils::encoding::varint::VarInt;

/// The outgoing ping packet is sent by the server to the client to check the connection.
/// Payload is just the same as whatever the client sent.
#[derive(Encode)]
pub struct OutgoingPing {
    pub packet_id: VarInt,
    pub payload: i64,
}
