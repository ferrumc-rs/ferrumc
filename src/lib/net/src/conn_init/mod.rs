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
use ferrumc_text::{ComponentBuilder, NamedColor, TextComponent};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tracing::{error, trace};

/// A small utility to remove the packet length and packet id from the stream, since we are pretty
/// sure we are going to get the right packet id and length, and we don't need to check it
/// If we get a packet with the id 0x12, we will skip it, since it is a serverbound plugin message packet
/// They have stupid formatting, and we don't want to deal with it
pub(crate) async fn trim_packet_head(conn: &mut OwnedReadHalf, value: u8) -> Result<(), NetError> {
    let mut len = VarInt::decode_async(conn, &NetDecodeOpts::None).await?;
    let mut id = VarInt::decode_async(conn, &NetDecodeOpts::None).await?;
    while id.0 == 0x14 {
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
    packet: &impl NetEncode,
) -> Result<(), NetError> {
    let mut packet_buffer = vec![];
    packet
        .encode_async(&mut packet_buffer, &NetEncodeOpts::WithLength)
        .await?;
    conn.write_all(&packet_buffer).await?;
    conn.flush().await?;
    Ok(())
}
pub const PROTOCOL_VERSION_1_21_5: i32 = 770;

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
    let hs_packet = Handshake::decode_async(&mut conn_read, &NetDecodeOpts::None).await?; // Check protocol version and send appropriate disconnect packet if mismatched

    if hs_packet.protocol_version.0 != PROTOCOL_VERSION_1_21_5 {
        trace!(
            "Protocol version mismatch: {} != {}",
            hs_packet.protocol_version.0,
            PROTOCOL_VERSION_1_21_5
        );
        return handle_version_mismatch(hs_packet, conn_read, conn_write, state).await;
    }

    match hs_packet.next_state.0 {
        1 => status(conn_read, conn_write, state)
            .await
            .map(|_| (true, None)),
        2 => login(conn_read, conn_write, state).await,
        3 => {
            // Transfer state - not implemented yet
            trace!("Transfer state (3) not implemented");
            Err(NetError::InvalidState(hs_packet.next_state.0 as u8))
        }
        invalid_state => {
            error!("Invalid handshake state: {}", invalid_state);
            Err(NetError::InvalidState(invalid_state as u8))
        }
    }
}

async fn handle_version_mismatch(
    hs_packet: Handshake,
    conn_read: &mut OwnedReadHalf,
    conn_write: &mut OwnedWriteHalf,
    state: GlobalState,
) -> Result<(bool, Option<PlayerIdentity>), NetError> {
    // Send appropriate disconnect packet based on the next state
    match hs_packet.next_state.0 {
        // If it was status, we can just send a status response, and the client will automatically understand the mismatch.
        1 => {
            // Status request - handle gracefully by proceeding to status
            // Status response will show the correct version
            trace!(
                "Protocol version mismatch during status request: {} != {}",
                hs_packet.protocol_version.0,
                PROTOCOL_VERSION_1_21_5
            );
            status(conn_read, conn_write, state)
                .await
                .map(|_| (true, None))
        } // If it was login, we need to send a login disconnect packet with a specific message
        2 => {
            // Login request - send login disconnect packet

            let disconnect_reason = get_mismatched_version_message(hs_packet.protocol_version.0);

            let login_disconnect =
                crate::packets::outgoing::login_disconnect::LoginDisconnectPacket::new(
                    disconnect_reason,
                );

            if let Err(send_err) = send_packet(conn_write, &login_disconnect).await {
                error!("Failed to send login disconnect packet {:?}", send_err);
            }

            trace!(
                "Sent login disconnect due to protocol version mismatch: {} != {}",
                hs_packet.protocol_version.0,
                PROTOCOL_VERSION_1_21_5
            );

            Err(NetError::MismatchedProtocolVersion(
                hs_packet.protocol_version.0,
                PROTOCOL_VERSION_1_21_5,
            ))
        }
        _ => {
            // Unknown state - just return error
            Err(NetError::MismatchedProtocolVersion(
                hs_packet.protocol_version.0,
                PROTOCOL_VERSION_1_21_5,
            ))
        }
    }
}

/// Generates a disconnect message for clients with mismatched protocol versions.
/// Format:
/// ```text
/// Your client is outdated!
/// Please use Minecraft version 1.21.1 to connect to this server.
/// Server Version: 767 | Your Version: 47
///```
fn get_mismatched_version_message(client_version: i32) -> TextComponent {
    ComponentBuilder::text("")
        .color(NamedColor::Yellow)
        .extra(
            ComponentBuilder::text("Your client is outdated!")
                .color(NamedColor::Red)
                .bold(),
        )
        .extra(ComponentBuilder::text("\n\n"))
        .extra(ComponentBuilder::text("Please use Minecraft version ").color(NamedColor::Gray))
        .extra(
            ComponentBuilder::text("1.21.5")
                .color(NamedColor::Green)
                .bold(),
        )
        .extra(ComponentBuilder::text(" to connect to this server.").color(NamedColor::Gray))
        .extra(ComponentBuilder::text("\n\n"))
        .extra(ComponentBuilder::text("Server Version: ").color(NamedColor::DarkGray))
        .extra(ComponentBuilder::text(PROTOCOL_VERSION_1_21_5.to_string()).color(NamedColor::Aqua))
        .extra(ComponentBuilder::text(" | Your Version: ").color(NamedColor::DarkGray))
        .extra(ComponentBuilder::text(client_version.to_string()).color(NamedColor::Red))
        .build()
}
