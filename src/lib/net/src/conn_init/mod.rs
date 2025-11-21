mod login;
mod status;

use crate::conn_init::login::login;
use crate::conn_init::status::status;
use crate::connection::StreamWriter;
use crate::errors::{NetError, PacketError};
use crate::packets::incoming::handshake::Handshake;
use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::lookup_packet;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_net_encryption::read::EncryptedReader;
use ferrumc_state::GlobalState;
use ferrumc_text::{ComponentBuilder, NamedColor, TextComponent};
use std::sync::atomic::Ordering;
use tokio::net::tcp::OwnedReadHalf;
use tracing::{error, trace};

/// Represents the result of a login attempt after the handshake process.
///
/// - `player_identity`: Populated when login is successful and a player is identified.
/// - `compression`: Indicates whether network compression should be enabled for this connection.
pub(crate) struct LoginResult {
    pub player_identity: Option<PlayerIdentity>,
    pub compression: bool,
}

/// Protocol version supported by this server implementation (Minecraft 1.21.8).
/// Used for rejecting clients with mismatched versions during handshake.
pub const PROTOCOL_VERSION_1_21_8: i32 = 772;

/// Handles the initial handshake sequence from a connecting client.
///
/// This function performs:
/// - Reading the first packet (handshake) from the client.
/// - Validating the packet type (expected handshake intent packet).
/// - Verifying that the client's protocol version matches the server's supported version.
/// - Transitioning the connection state to one of:
///   - **Status**: For server list ping requests (NextState = 1).
///   - **Login**: For actual login attempts (NextState = 2).
///   - **Transfer**: (NextState = 3) – not implemented yet.
///
/// # Parameters
/// - `conn_read`: Read half of the TCP stream for incoming data.
/// - `conn_write`: Writer for sending packets back to the client.
/// - `state`: Shared global server state.
///
/// # Returns
/// - `(bool, LoginResult)`:
///   - `bool`: Whether the connection should be closed after the handshake.
///   - `LoginResult`: Information about the login state and compression.
///
/// # Errors
/// Returns `NetError` if:
/// - An unexpected packet is received.
/// - Protocol version mismatches and cannot be gracefully handled.
/// - An invalid or unsupported handshake state is encountered.
pub async fn handle_handshake(
    mut conn_read: &mut EncryptedReader<OwnedReadHalf>,
    conn_write: &StreamWriter,
    state: GlobalState,
) -> Result<(bool, LoginResult), NetError> {
    // Build a PacketSkeleton from the first inbound packet.
    // This handles framing, reading packet ID and payload.
    let mut skel = PacketSkeleton::new(
        &mut conn_read,
        conn_write.compress.load(Ordering::Relaxed),
        crate::ConnState::Handshake,
    )
    .await?;

    // Ensure the packet ID matches the expected handshake packet.
    let expected_id = lookup_packet!("handshake", "serverbound", "intention");
    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: crate::ConnState::Handshake,
        }));
    }

    // Decode the handshake packet (protocol version, server address, next state, etc.).
    let hs_packet = Handshake::decode_async(&mut skel.data, &NetDecodeOpts::None).await?;

    // If protocol version is mismatched, handle gracefully or disconnect client.
    if hs_packet.protocol_version.0 != PROTOCOL_VERSION_1_21_8 {
        trace!(
            "Protocol version mismatch: {} != {}",
            hs_packet.protocol_version.0,
            PROTOCOL_VERSION_1_21_8
        );
        return handle_version_mismatch(hs_packet, conn_read, conn_write, state).await;
    }

    // Branch based on the next connection state requested by the client.
    match hs_packet.next_state.0 {
        1 => status(conn_read, conn_write, state).await,
        2 => login(conn_read, conn_write, state).await,
        3 => {
            // Placeholder for a potential server transfer state (not supported yet).
            trace!("Transfer state (3) not implemented");
            Err(NetError::InvalidState(hs_packet.next_state.0 as u8))
        }
        invalid_state => {
            error!("Invalid handshake state: {}", invalid_state);
            Err(NetError::InvalidState(invalid_state as u8))
        }
    }
}

/// Handles protocol version mismatches detected during handshake.
///
/// Sends an appropriate disconnect message or status response based on the
/// client's requested next state.
/// - Status requests: proceeds with status flow (client will see version info).
/// - Login requests: sends an explicit disconnect message describing the mismatch.
///
/// # Parameters
/// - `hs_packet`: The original handshake packet from the client.
/// - `conn_read`, `conn_write`: The connection's read/write halves.
/// - `state`: Shared global server state.
///
/// # Returns
/// Always returns `Err(NetError::MismatchedProtocolVersion)` to signal the mismatch.
async fn handle_version_mismatch(
    hs_packet: Handshake,
    conn_read: &mut EncryptedReader<OwnedReadHalf>,
    conn_write: &StreamWriter,
    state: GlobalState,
) -> Result<(bool, LoginResult), NetError> {
    match hs_packet.next_state.0 {
        // Status: let the client query the server's supported version without disconnecting.
        1 => {
            trace!(
                "Protocol version mismatch during status request: {} != {}",
                hs_packet.protocol_version.0,
                PROTOCOL_VERSION_1_21_8
            );
            status(conn_read, conn_write, state).await
        }
        // Login: actively disconnect with a descriptive message.
        2 => {
            let disconnect_reason = get_mismatched_version_message(hs_packet.protocol_version.0);

            let login_disconnect =
                crate::packets::outgoing::login_disconnect::LoginDisconnectPacket::new(
                    disconnect_reason,
                );

            if let Err(send_err) = conn_write.send_packet(login_disconnect) {
                error!("Failed to send login disconnect packet {:?}", send_err);
            }

            trace!(
                "Sent login disconnect due to protocol version mismatch: {} != {}",
                hs_packet.protocol_version.0,
                PROTOCOL_VERSION_1_21_8
            );

            Err(NetError::MismatchedProtocolVersion(
                hs_packet.protocol_version.0,
                PROTOCOL_VERSION_1_21_8,
            ))
        }
        // Unknown or unsupported state: just return a generic mismatch error.
        _ => Err(NetError::MismatchedProtocolVersion(
            hs_packet.protocol_version.0,
            PROTOCOL_VERSION_1_21_8,
        )),
    }
}

/// Builds a Minecraft chat component describing a protocol version mismatch.
///
/// # Format
/// ```text
/// Your client is outdated!
/// Please use Minecraft version 1.21.8 to connect to this server.
/// Server Version: 772 | Your Version: <client_version>
/// ```
///
/// This message is used in disconnect packets for login attempts with an
/// unsupported client protocol version.
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
            ComponentBuilder::text("1.21.8")
                .color(NamedColor::Green)
                .bold(),
        )
        .extra(ComponentBuilder::text(" to connect to this server.").color(NamedColor::Gray))
        .extra(ComponentBuilder::text("\n\n"))
        .extra(ComponentBuilder::text("Server Version: ").color(NamedColor::DarkGray))
        .extra(ComponentBuilder::text(PROTOCOL_VERSION_1_21_8.to_string()).color(NamedColor::Aqua))
        .extra(ComponentBuilder::text(" | Your Version: ").color(NamedColor::DarkGray))
        .extra(ComponentBuilder::text(client_version.to_string()).color(NamedColor::Red))
        .build()
}
