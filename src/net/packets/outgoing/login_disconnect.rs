use ferrumc_codec::network_types::varint::VarInt;
use ferrumc_macros::Encode;

/// The login disconnect packet is sent by the server to the client to disconnect the client.
/// Used to cancel the login process.
#[derive(Encode)]
pub struct LoginDisconnect {
    pub packet_id: VarInt,
    pub reason: String,
}
