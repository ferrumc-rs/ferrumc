use ferrumc_macros::Encode;
use crate::utils::encoding::varint::VarInt;

/// The login disconnect packet is sent by the server to the client to disconnect the client.
/// Used to cancel the login process.
#[derive(Encode)]
pub struct LoginDisconnect {
    pub packet_id: VarInt,
    pub reason: String,
}