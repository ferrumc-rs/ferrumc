use crate::auth::authenticate_user;
use crate::compression::compress_packet;
use crate::conn_init::VarInt;
use crate::conn_init::{LoginResult, NetDecodeOpts};
use crate::connection::StreamWriter;
use crate::errors::{NetAuthenticationError, NetError, PacketError};
use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use crate::packets::outgoing::login_success::{LoginSuccessPacket, LoginSuccessProperties};
use crate::packets::outgoing::set_default_spawn_position::DEFAULT_SPAWN_POSITION;
use crate::packets::outgoing::{commands::CommandsPacket, registry_data::REGISTRY_PACKETS};
use crate::ConnState::*;
use ferrumc_config::server_config::{get_global_config, ServerConfig};
use ferrumc_core::identity::player_identity::{PlayerIdentity, PlayerProperty};
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::lookup_packet;
use ferrumc_net_codec::decode::NetDecode;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_net_encryption::errors::NetEncryptionError;
use ferrumc_net_encryption::get_encryption_keys;
use ferrumc_net_encryption::read::EncryptedReader;
use ferrumc_state::GlobalState;
use ferrumc_world::pos::ChunkPos;

use crate::packets::incoming::ack_finish_configuration::AckFinishConfigurationPacket;
use crate::packets::incoming::client_information::ClientInformation;
use crate::packets::incoming::confirm_player_teleport::ConfirmPlayerTeleport;
use crate::packets::incoming::encryption_response::EncryptionResponse;
use crate::packets::incoming::login_acknowledged::LoginAcknowledgedPacket;
use crate::packets::incoming::login_start::LoginStartPacket;
use crate::packets::incoming::server_bound_known_packs::ServerBoundKnownPacks;
use crate::packets::incoming::set_player_position_and_rotation::SetPlayerPositionAndRotationPacket;
use crate::packets::outgoing::client_bound_known_packs::ClientBoundKnownPacksPacket;
use crate::packets::outgoing::client_bound_plugin_message::ClientBoundPluginMessagePacket;
use crate::packets::outgoing::encryption_request::EncryptionRequest;
use crate::packets::outgoing::entity_event::EntityStatus;
use crate::packets::outgoing::finish_configuration::FinishConfigurationPacket;
use crate::packets::outgoing::game_event::GameEventPacket;
use crate::packets::outgoing::login_play::LoginPlayPacket;
use crate::packets::outgoing::player_abilities::PlayerAbilities;
use crate::packets::outgoing::player_info_update::PlayerInfoUpdatePacket;
use crate::packets::outgoing::set_center_chunk::SetCenterChunk;
use crate::packets::outgoing::set_compression::SetCompressionPacket;
use crate::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;
use crate::ConnState;
use ferrumc_components::player::offline_player_data::OfflinePlayerData;
use rand::RngCore;
use tokio::net::tcp::OwnedReadHalf;
use tracing::{debug, error, trace};
use uuid::Uuid;
// =================================================================================================
// Helper Functions
// =================================================================================================

/// Waits for a specific packet type, ignoring any other packets received in the meantime.
async fn wait_for_packet<T: NetDecode>(
    conn_read: &mut EncryptedReader<OwnedReadHalf>,
    compressed: bool,
    conn_state: ConnState,
    expected_packet_id: i32,
) -> Result<T, NetError> {
    let expected_id = expected_packet_id as u8;
    loop {
        let mut skel = PacketSkeleton::new(conn_read, compressed, conn_state).await?;
        if skel.id == expected_id {
            return T::decode(&mut skel.data, &NetDecodeOpts::None).map_err(NetError::from);
        } else {
            trace!(
                "Ignoring packet 0x{:02X} while waiting for 0x{:02X}",
                skel.id,
                expected_id
            );
        }
    }
}

// =================================================================================================
// Login Phase 1: Initial Handshake
// =================================================================================================

/// Receives and validates the initial Login Start packet from the client.
async fn receive_login_start(
    conn_read: &mut EncryptedReader<OwnedReadHalf>,
    compressed: bool,
) -> Result<LoginStartPacket, NetError> {
    let mut skel = PacketSkeleton::new(conn_read, compressed, Login).await?;
    let expected_id = lookup_packet!("login", "serverbound", "hello");

    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Login,
        }));
    }

    LoginStartPacket::decode(&mut skel.data, &NetDecodeOpts::None).map_err(NetError::from)
}

/// Negotiates compression with the client if configured.
fn setup_compression(conn_write: &StreamWriter, config: &ServerConfig) -> Result<bool, NetError> {
    if config.network_compression_threshold > 0 {
        let compression_packet = SetCompressionPacket {
            threshold: VarInt::new(config.network_compression_threshold),
        };
        conn_write.send_packet(compression_packet)?;
        conn_write
            .compress
            .store(true, std::sync::atomic::Ordering::Relaxed);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Handles encryption setup and optional Mojang authentication.
/// Returns player properties if authentication was performed.
async fn setup_encryption_and_auth(
    conn_read: &mut EncryptedReader<OwnedReadHalf>,
    conn_write: &StreamWriter,
    config: &ServerConfig,
    login_start: &LoginStartPacket,
    compressed: bool,
) -> Result<Vec<PlayerProperty>, NetError> {
    let mut player_properties = Vec::new();

    if !config.encryption_enabled && !config.online_mode {
        return Ok(player_properties);
    }

    // Generate verify token
    let mut verify_token = vec![0u8; 16];
    rand::rng().fill_bytes(&mut verify_token);

    // Send encryption request
    let encryption_packet = EncryptionRequest {
        server_id: "".to_string(),
        public_key: LengthPrefixedVec::new(get_encryption_keys().clone_der()),
        verify_token: LengthPrefixedVec::new(verify_token.clone()),
        should_authenticate: config.online_mode,
    };
    conn_write.send_packet(encryption_packet)?;

    // Wait for encryption response
    let mut skel = PacketSkeleton::new(conn_read, compressed, Login).await?;
    let expected_id = lookup_packet!("login", "serverbound", "key");

    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Login,
        }));
    }

    let encryption_response = EncryptionResponse::decode(&mut skel.data, &NetDecodeOpts::None)?;

    // Verify token
    let received_verify_token =
        get_encryption_keys().decrypt_bytes(&encryption_response.verify_token.data)?;

    if verify_token != received_verify_token {
        return Err(NetError::EncryptionError(
            NetEncryptionError::VerifyTokenMismatch {
                expected: verify_token,
                returned: encryption_response.verify_token.data,
            },
        ));
    }

    // Enable encryption
    let shared_secret =
        get_encryption_keys().decrypt_bytes(&encryption_response.shared_secret.data)?;
    conn_read.update_cipher(&shared_secret);
    conn_write.update_encryption_cipher(&shared_secret)?;
    debug!("Successfully enabled encryption!");

    // Authenticate with Mojang if online mode
    if config.online_mode {
        let (username, uuid, properties) =
            authenticate_user(&login_start.username, "", &shared_secret).await?;

        if username != login_start.username || uuid.as_u128() != login_start.uuid {
            return Err(NetError::AuthenticationError(
                NetAuthenticationError::InformationDoesntMatch,
            ));
        }

        player_properties.extend_from_slice(&properties);
    }

    Ok(player_properties)
}

/// Sends Login Success packet and waits for client acknowledgement.
async fn send_login_success(
    conn_read: &mut EncryptedReader<OwnedReadHalf>,
    conn_write: &StreamWriter,
    login_start: &LoginStartPacket,
    player_properties: &[PlayerProperty],
    compressed: bool,
) -> Result<PlayerIdentity, NetError> {
    // Send Login Success
    let login_success = LoginSuccessPacket {
        uuid: login_start.uuid,
        username: &login_start.username,
        properties: LengthPrefixedVec::new(
            player_properties
                .iter()
                .map(|property: &PlayerProperty| LoginSuccessProperties {
                    name: &property.name,
                    value: &property.value,
                    signature: PrefixedOptional::new(property.signature.as_deref()),
                })
                .collect(),
        ),
    };
    conn_write.send_packet(login_success)?;

    // Build PlayerIdentity
    let player_identity = PlayerIdentity {
        uuid: Uuid::from_u128(login_start.uuid),
        username: login_start.username.clone(),
        short_uuid: login_start.uuid as i32,
        properties: player_properties.to_vec(),
    };

    // Wait for Login Acknowledged
    let mut skel = PacketSkeleton::new(conn_read, compressed, Login).await?;
    let expected_id = lookup_packet!("login", "serverbound", "login_acknowledged");

    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Login,
        }));
    }

    let _login_acknowledged =
        LoginAcknowledgedPacket::decode(&mut skel.data, &NetDecodeOpts::None)?;

    Ok(player_identity)
}

// =================================================================================================
// Login Phase 2: Configuration
// =================================================================================================

/// Receives client information (locale, view distance, etc.).
async fn receive_client_information(
    conn_read: &mut EncryptedReader<OwnedReadHalf>,
    compressed: bool,
) -> Result<ClientInformation, NetError> {
    let mut skel = PacketSkeleton::new(conn_read, compressed, Configuration).await?;
    let expected_id = lookup_packet!("configuration", "serverbound", "client_information");

    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Configuration,
        }));
    }

    let client_info = ClientInformation::decode(&mut skel.data, &NetDecodeOpts::None)?;

    trace!(
        "Client information: {{ locale: {}, view_distance: {}, chat_mode: {}, chat_colors: {}, displayed_skin_parts: {} }}",
        client_info.locale,
        client_info.view_distance,
        client_info.chat_mode,
        client_info.chat_colors,
        client_info.displayed_skin_parts
    );

    Ok(client_info)
}

/// Exchanges known packs with the client.
async fn exchange_known_packs(
    conn_read: &mut EncryptedReader<OwnedReadHalf>,
    conn_write: &StreamWriter,
    compressed: bool,
) -> Result<(), NetError> {
    // Send known packs
    conn_write.send_packet(ClientBoundKnownPacksPacket::new())?;

    // Receive client's selected packs
    let mut skel = PacketSkeleton::new(conn_read, compressed, Configuration).await?;
    let expected_id = lookup_packet!("configuration", "serverbound", "select_known_packs");

    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Configuration,
        }));
    }

    let _server_bound_known_packs =
        ServerBoundKnownPacks::decode(&mut skel.data, &NetDecodeOpts::None)?;

    Ok(())
}

/// Sends registry data, brand info, and finishes configuration phase.
async fn finish_configuration(
    conn_read: &mut EncryptedReader<OwnedReadHalf>,
    conn_write: &StreamWriter,
    compressed: bool,
) -> Result<(), NetError> {
    // Send registry data
    for packet in &*REGISTRY_PACKETS {
        conn_write.send_packet_ref(packet)?;
    }

    // Send brand
    conn_write.send_packet(ClientBoundPluginMessagePacket::brand())?;

    // Signal end of configuration
    conn_write.send_packet(FinishConfigurationPacket)?;

    // Wait for client ack
    let mut skel = PacketSkeleton::new(conn_read, compressed, Configuration).await?;
    let expected_id = lookup_packet!("configuration", "serverbound", "finish_configuration");

    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Configuration,
        }));
    }

    let _finish_config_ack =
        AckFinishConfigurationPacket::decode(&mut skel.data, &NetDecodeOpts::None)?;

    Ok(())
}

// =================================================================================================
// Login Phase 3: Play State Setup
// =================================================================================================

/// Sends initial play state packets (login_play, abilities, op level).
fn send_initial_play_packets(
    conn_write: &StreamWriter,
    state: &GlobalState,
    player_identity: &PlayerIdentity,
) -> Result<(), NetError> {
    // Send login_play
    let player_data: OfflinePlayerData = state
        .world
        .load_player_data(player_identity.uuid)
        .unwrap_or_default()
        .unwrap_or_default();
    let game_mode = player_data.gamemode;

    conn_write.send_packet(LoginPlayPacket::new(
        player_identity.short_uuid,
        game_mode as u8,
    ))?;

    // Send abilities
    let abilities = player_data.abilities;

    conn_write.send_packet(PlayerAbilities::from_abilities(&abilities))?;

    // Send OP level (TODO: use actual player OP level)
    conn_write.send_packet(EntityStatus {
        entity_id: player_identity.short_uuid,
        status: 28, // OP level 4
    })?;

    Ok(())
}

/// Sends player position sync and waits for teleport confirmation.
async fn sync_player_position(
    conn_read: &mut EncryptedReader<OwnedReadHalf>,
    conn_write: &StreamWriter,
    state: &GlobalState,
    player_identity: &PlayerIdentity,
    compressed: bool,
) -> Result<(), NetError> {
    let teleport_id_i32: i32 = (rand::random::<u32>() & 0x3FFF_FFFF) as i32;

    // Get spawn position from cache or use defaults
    let (spawn_pos, spawn_rotation) = if let Some(data) = state
        .world
        .load_player_data::<OfflinePlayerData>(player_identity.uuid)
        .unwrap_or_else(|err| {
            error!(
                "Error loading player data for {}: {:?}",
                player_identity.username, err
            );
            None
        }) {
        (data.position.into(), data.rotation)
    } else {
        (
            Position::new(
                DEFAULT_SPAWN_POSITION.x as f64,
                DEFAULT_SPAWN_POSITION.y as f64,
                DEFAULT_SPAWN_POSITION.z as f64,
            ),
            Rotation::default(),
        )
    };

    // Send position sync
    conn_write.send_packet(SynchronizePlayerPositionPacket::from_position_rotation(
        &spawn_pos,
        &spawn_rotation,
        VarInt::new(teleport_id_i32),
    ))?;

    // Wait for teleport confirmation
    let expected_id = lookup_packet!("play", "serverbound", "accept_teleportation");
    let confirm: ConfirmPlayerTeleport =
        wait_for_packet(conn_read, compressed, Play, expected_id).await?;

    if confirm.teleport_id.0 != teleport_id_i32 {
        error!(
            "Teleport ID mismatch: expected {}, got {}",
            teleport_id_i32, confirm.teleport_id.0
        );
    }

    // Wait for first movement packet
    let expected_id = lookup_packet!("play", "serverbound", "move_player_pos_rot");
    let _: SetPlayerPositionAndRotationPacket =
        wait_for_packet(conn_read, compressed, Play, expected_id).await?;

    Ok(())
}

/// Sends player info and game event packets.
fn send_player_info(
    conn_write: &StreamWriter,
    player_identity: &PlayerIdentity,
) -> Result<(), NetError> {
    conn_write.send_packet(PlayerInfoUpdatePacket::new_player_join_packet(
        player_identity,
    ))?;
    conn_write.send_packet(GameEventPacket::new(13, 0.0))?;
    Ok(())
}

/// Sends initial chunks to the player.
fn send_initial_chunks(
    conn_write: &StreamWriter,
    state: &GlobalState,
    config: &ServerConfig,
    client_view_distance: i8,
    compressed: bool,
) -> Result<(), NetError> {
    // Send center chunk
    conn_write.send_packet(SetCenterChunk::new(0, 0))?;

    // Calculate render distance
    let server_render_distance = config.chunk_render_distance as i32;
    let client_view_distance = client_view_distance as i32;
    let radius = server_render_distance.min(client_view_distance);

    // Generate/load chunks in parallel
    let mut batch = state.thread_pool.batch();

    for x in -radius..=radius {
        for z in -radius..=radius {
            batch.execute({
                let state = state.clone();
                move || -> Result<Vec<u8>, NetError> {
                    let chunk = ferrumc_utils::world::load_or_generate_mut(&state, ChunkPos::new(x,z), "overworld").expect("Failed to load or generate chunk");
                    let chunk_data =
                        crate::packets::outgoing::chunk_and_light_data::ChunkAndLightData::from_chunk(
                        ChunkPos::new(x,z),
                            &chunk,
                        )?;
                    compress_packet(&chunk_data, compressed, &NetEncodeOpts::WithLength, 64)
                }
            });
        }
    }

    // Send all chunks
    for packet in batch.wait() {
        match packet {
            Ok(data) => conn_write.send_raw_packet(data)?,
            Err(err) => {
                error!("Failed to send chunk data: {:?}", err);
                return Err(NetError::Misc(format!(
                    "Failed to send chunk data: {:?}",
                    err
                )));
            }
        }
    }

    Ok(())
}

/// Sends the command graph to the client.
fn send_command_graph(conn_write: &StreamWriter) -> Result<(), NetError> {
    conn_write.send_packet(CommandsPacket::from_global_graph())?;
    trace!(
        "sending command graph {:#?}",
        ferrumc_commands::infrastructure::get_graph()
    );
    Ok(())
}

// =================================================================================================
// Main Login Function
// =================================================================================================

/// Handles the **login sequence** for a newly connecting client.
///
/// This function orchestrates the complete Minecraft login/configuration handshake:
/// 1. Initial handshake (login start, compression, encryption)
/// 2. Configuration phase (client info, known packs, registry data)
/// 3. Play state setup (position sync, chunks, commands)
///
/// # Returns
/// `(false, LoginResult)` on success, where:
/// - `false` = keep connection open.
/// - `LoginResult` contains player identity and compression settings.
///
/// # Errors
/// Returns `NetError` for protocol violations, unexpected packets, or I/O errors.
pub(super) async fn login(
    conn_read: &mut EncryptedReader<OwnedReadHalf>,
    conn_write: &StreamWriter,
    state: GlobalState,
) -> Result<(bool, LoginResult), NetError> {
    let config = get_global_config();

    // Phase 1: Initial Handshake
    let login_start = receive_login_start(conn_read, false).await?;
    let compressed = setup_compression(conn_write, config)?;
    let player_properties =
        setup_encryption_and_auth(conn_read, conn_write, config, &login_start, compressed).await?;

    let player_identity = send_login_success(
        conn_read,
        conn_write,
        &login_start,
        &player_properties,
        compressed,
    )
    .await?;

    // Phase 2: Configuration
    let client_info = receive_client_information(conn_read, compressed).await?;
    exchange_known_packs(conn_read, conn_write, compressed).await?;
    finish_configuration(conn_read, conn_write, compressed).await?;

    // Phase 3: Play State Setup
    send_initial_play_packets(conn_write, &state, &player_identity)?;
    sync_player_position(conn_read, conn_write, &state, &player_identity, compressed).await?;
    send_player_info(conn_write, &player_identity)?;
    send_initial_chunks(
        conn_write,
        &state,
        config,
        client_info.view_distance,
        compressed,
    )?;
    send_command_graph(conn_write)?;

    // Login complete
    Ok((
        false,
        LoginResult {
            player_identity: Some(player_identity),
            compression: compressed,
        },
    ))
}
