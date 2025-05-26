mod login;
mod status;

use crate::conn_init::login::login;
use crate::conn_init::status::status;
use crate::errors::NetError;
use crate::packets::incoming::handshake::Handshake;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalState;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tracing::trace;

// A small utility to remove the packet length and packet id from the stream, since we are pretty
// sure we are going to get the right packet id and length, and we don't need to check it
// If we get a packet with the id 0x12, we will skip it, since it is a serverbound plugin message packet
// They have stupid formatting, and we don't want to deal with it
pub(crate) async fn trim_packet_head(conn: &mut OwnedReadHalf, value: u8) -> Result<(), NetError> {
    let mut len = VarInt::decode_async(conn, &NetDecodeOpts::None).await?;
    let mut id = VarInt::decode_async(conn, &NetDecodeOpts::None).await?;
    while id.0 == 0x12 {
        trace!("Serverbound plugin message packet detected");
        let mut packet_data = vec![0; len.0 as usize - id.len()];
        conn.read_exact(&mut packet_data).await?;
        trace!("Packet data: {:?}", &packet_data);
        len = VarInt::decode_async(conn, &NetDecodeOpts::None).await?;
        id = VarInt::decode_async(conn, &NetDecodeOpts::None).await?;
    }
    assert_eq!(id.0, value as i32);
    Ok(())
}

pub(crate) async fn send_packet(
    conn: &mut OwnedWriteHalf,
    packet: impl NetEncode,
) -> Result<(), NetError> {
    let mut packet_buffer = vec![];
    packet
        .encode_async(&mut packet_buffer, &NetEncodeOpts::WithLength)
        .await?;
    conn.write_all(&packet_buffer).await?;
    conn.flush().await?;
    Ok(())
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
) -> Result<(bool, Option<PlayerIdentity>), NetError> {
    trim_packet_head(conn_read, 0x00).await?;

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
        1 => status(conn_read, conn_write, state)
            .await
            .map(|_| (true, None)),
        2 => login(conn_read, conn_write, state).await,
        3 => unimplemented!(),
        _ => Err(NetError::InvalidState(hs_packet.next_state.0 as u8)),
    }
}
