mod login;
mod status;

use crate::conn_init::login::login;
use crate::conn_init::status::status;
use crate::errors::NetError;
use crate::packets::incoming::handshake::Handshake;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalState;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

// A small utility to remove the packet length and packet id from the stream, since we are pretty
// sure we are going to get the right packet id and length, and we don't need to check it
// We still do debug asserts on the ID though, just to be sure
#[macro_export]
macro_rules! trim_packet_head {
    ($conn:ident,  $value:literal) => {{
        let _ = VarInt::decode_async(&mut $conn, &NetDecodeOpts::None).await?;
        let val = VarInt::decode_async(&mut $conn, &NetDecodeOpts::None).await?;
        assert_eq!(val.0, $value);
    }};
}
pub const PROTOCOL_VERSION_1_21_1: i32 = 767;

// Todo: Make this function return encryption and compression settings
/// Handle the handshake sequence for the server.
///
/// This function is responsible for processing the initial handshake sequence
/// from the client. It reads the handshake packet, verifies the protocol version,
/// and determines the next state of the connection (status, login, etc.).
///
/// It returns a `Result<bool, NetError>` indicating whether the handshake was successful
/// or not. If the handshake returns an Ok value, the inner bool indicates whether the connection
/// should be closed or not after the handshake is complete.
pub async fn handle_handshake(
    mut conn_read: &mut OwnedReadHalf,
    conn_write: &mut OwnedWriteHalf,
    state: GlobalState,
) -> Result<bool, NetError> {
    trim_packet_head!(conn_read, 0x00);

    // Get incoming handshake packet
    let hs_packet = Handshake::decode_async(&mut conn_read, &NetDecodeOpts::None).await?;

    // Todo: Send either a disconnect packet or a status packet indicating the versions don't match
    if hs_packet.protocol_version.0 != PROTOCOL_VERSION_1_21_1 {
        return Err(NetError::MismatchedProtocolVersion(
            hs_packet.protocol_version.0,
            PROTOCOL_VERSION_1_21_1,
        ));
    }

    match hs_packet.next_state.0 {
        1 => status(conn_read, conn_write, state).await,
        2 => login(conn_read, conn_write, state).await,
        3 => unimplemented!(),
        _ => Err(NetError::InvalidState(hs_packet.next_state.0 as u8)),
    }
}
