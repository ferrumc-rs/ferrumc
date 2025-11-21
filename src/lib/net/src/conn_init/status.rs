use crate::conn_init::LoginResult;
use crate::connection::StreamWriter;
use crate::errors::{NetError, PacketError};
use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use crate::packets::incoming::ping::PingPacket;
use crate::packets::incoming::status_request::StatusRequestPacket;
use crate::packets::outgoing::ping_response::PongPacket;
use crate::packets::outgoing::status_response::StatusResponse;
use ferrumc_config::favicon::get_favicon_base64;
use ferrumc_config::server_config::get_global_config;
use ferrumc_macros::lookup_packet;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_encryption::read::EncryptedReader;
use ferrumc_state::GlobalState;
use rand::prelude::IndexedRandom;
use tokio::net::tcp::OwnedReadHalf;

/// Handles the Minecraft server "status" state of the handshake.
///
/// This function implements the **Server List Ping protocol**:
/// 1. Reads the incoming **Status Request** packet from the client.
/// 2. Responds with a **Status Response** packet containing:
///    - Server version
///    - Player count and sample list
///    - MOTD (Message of the Day)
///    - Favicon
/// 3. Reads the subsequent **Ping Request** packet.
/// 4. Responds with a **Pong** packet echoing the received payload.
///
/// This is part of the server list query process (when the client pings a server
/// to display it in the multiplayer menu, before login).
///
/// # Returns
/// A tuple `(true, LoginResult)`:
/// - `true`: Indicates that the connection should be closed after responding.
/// - `LoginResult`: Contains no player identity or compression because this is a stateless query.
pub(super) async fn status(
    mut conn_read: &mut EncryptedReader<OwnedReadHalf>,
    conn_write: &StreamWriter,
    state: GlobalState,
) -> Result<(bool, LoginResult), NetError> {
    // ---- Phase 1: Receive and validate Status Request packet ----

    // Read next incoming packet in "status" connection state
    let mut skel = PacketSkeleton::new(&mut conn_read, false, crate::ConnState::Status).await?;

    // Expected packet ID for a status request
    let expected_id = lookup_packet!("status", "serverbound", "status_request");

    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: crate::ConnState::Status,
        }));
    }

    // Parse the incoming status request (no fields, acts as a trigger)
    let _status_req =
        StatusRequestPacket::decode_async(&mut skel.data, &NetDecodeOpts::None).await?;

    // ---- Phase 2: Send Status Response ----

    let status_response = StatusResponse {
        json_response: get_server_status(&state),
    };

    // Send server status information back to client
    conn_write.send_packet(status_response)?;

    // ---- Phase 3: Wait for Ping Request ----

    let mut skel = PacketSkeleton::new(&mut conn_read, false, crate::ConnState::Status).await?;

    let expected_id = lookup_packet!("status", "serverbound", "ping_request");

    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: crate::ConnState::Status,
        }));
    }

    // Parse ping request containing a payload (usually current timestamp)
    let ping_req = PingPacket::decode_async(&mut skel.data, &NetDecodeOpts::None).await?;

    // Respond with Pong containing the same payload (echo test)
    let pong_packet = PongPacket {
        payload: ping_req.payload,
    };
    conn_write.send_packet(pong_packet)?;

    // Status flow does not transition to login state.
    // The connection can be safely closed.
    Ok((
        true,
        LoginResult {
            player_identity: None,
            compression: false,
        },
    ))
}

/// Builds a JSON string describing the server's status.
///
/// This data is used in the **Status Response packet** for client pings.
/// Includes version info, player counts, MOTD, favicon, and secure chat flag.
///
/// # Parameters
/// - `state`: A reference to the global server state, used to retrieve the online player list.
///
/// # Returns
/// A JSON-encoded string containing the server's status.
fn get_server_status(state: &GlobalState) -> String {
    // Internal structs serialized to match Minecraft's server list response schema
    mod structs {
        #[derive(serde_derive::Serialize)]
        pub(super) struct ServerStatus<'a> {
            pub version: Version<'a>,
            pub players: Players<'a>,
            pub description: Description<'a>,
            pub favicon: &'a str,
            pub enforces_secure_chat: bool,
        }

        #[derive(serde_derive::Serialize)]
        pub(super) struct Version<'a> {
            pub name: &'a str,
            pub protocol: u16,
        }

        #[derive(serde_derive::Serialize)]
        pub(super) struct Players<'a> {
            pub max: u32,
            pub online: u16,
            pub sample: Vec<Player<'a>>,
        }

        #[derive(serde_derive::Serialize)]
        pub(super) struct Player<'a> {
            pub name: &'a str,
            pub id: &'a str,
        }

        /// Temporary struct used before borrowing string slices for serialization.
        pub(super) struct PlayerData {
            pub name: String,
            pub id: String,
        }

        #[derive(serde_derive::Serialize)]
        pub(super) struct Description<'a> {
            pub text: &'a str,
        }
    }

    let config = get_global_config();

    // Protocol info
    let version = structs::Version {
        name: "1.21.8",
        protocol: crate::conn_init::PROTOCOL_VERSION_1_21_8 as u16,
    };

    // Collect up to 5 players from the active player list
    let online_players_sample = state
        .players
        .player_list
        .iter()
        .take(5)
        .map(|player_data| structs::PlayerData {
            name: player_data.value().1.clone(),
            id: uuid::Uuid::from_u128(player_data.value().0).to_string(),
        })
        .collect::<Vec<_>>();

    // Convert owned Strings into &str for serialization
    let online_players_sample = online_players_sample
        .iter()
        .map(|p| structs::Player {
            name: p.name.as_str(),
            id: p.id.as_str(),
        })
        .collect::<Vec<_>>();

    // Player counts and sample
    let players = structs::Players {
        max: config.max_players,
        online: online_players_sample.len() as u16,
        sample: online_players_sample,
    };

    // Randomly choose a MOTD line from the configured list
    let motd = config.motd.choose(&mut rand::rng()).unwrap();
    let description = structs::Description { text: motd };

    // Encode favicon image in base64
    let favicon = get_favicon_base64();

    // Construct and serialize status payload
    let status = structs::ServerStatus {
        version,
        players,
        description,
        favicon,
        enforces_secure_chat: false,
    };

    serde_json::to_string(&status).unwrap()
}
