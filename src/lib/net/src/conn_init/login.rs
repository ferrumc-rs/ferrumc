use crate::compression::compress_packet;
use crate::conn_init::VarInt;
use crate::conn_init::{LoginResult, NetDecodeOpts};
use crate::connection::StreamWriter;
use crate::errors::{NetError, PacketError};
use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use crate::packets::outgoing::{commands::CommandsPacket, registry_data::REGISTRY_PACKETS};
use crate::ConnState::*;
use ferrumc_components::player::identity::PlayerIdentity;
use ferrumc_components::state::server_state::GlobalState;
use ferrumc_config::server_config::get_global_config;
use ferrumc_macros::lookup_packet;
use ferrumc_net_codec::decode::NetDecode;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use tokio::net::tcp::OwnedReadHalf;
use tracing::{error, trace};

/// Handles the **login sequence** for a newly connecting client.
///
/// This function follows the Minecraft login/configuration handshake:
/// 1. Reads the initial login packet and authenticates the username/UUID.
/// 2. Optionally enables network compression.
/// 3. Sends required handshake completion packets:
///    - Login success
///    - Configuration phase packets
///    - Registry and world data
/// 4. Spawns the player in the world (initial chunks, teleport confirmation).
///
/// # Returns
/// `(false, LoginResult)` on success, where:
/// - `false` = keep connection open.
/// - `LoginResult` contains player identity and compression settings.
///
/// # Errors
/// Returns `NetError` for protocol violations, unexpected packets, or I/O errors.
pub(super) async fn login(
    conn_read: &mut OwnedReadHalf,
    conn_write: &StreamWriter,
    state: GlobalState,
) -> Result<(bool, LoginResult), NetError> {
    let mut compressed = false;

    // =============================================================================================
    // 1 Receive initial Login Start packet
    let mut skel = PacketSkeleton::new(conn_read, compressed, Login).await?;

    let expected_id = lookup_packet!("login", "serverbound", "hello");

    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Login,
        }));
    }

    let login_start = crate::packets::incoming::login_start::LoginStartPacket::decode(
        &mut skel.data,
        &NetDecodeOpts::None,
    )?;

    // =============================================================================================
    // 2 Negotiate compression if configured
    if get_global_config().network_compression_threshold > 0 {
        compressed = true;

        // Notify client to enable compression on subsequent packets
        let compression_packet = crate::packets::outgoing::set_compression::SetCompressionPacket {
            threshold: VarInt::new(get_global_config().network_compression_threshold),
        };
        conn_write.send_packet(compression_packet)?;
        conn_write
            .compress
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    // =============================================================================================
    // 3 Send Login Success (UUID and username acknowledgement)
    let login_success = crate::packets::outgoing::login_success::LoginSuccessPacket {
        uuid: login_start.uuid,
        username: &login_start.username,
        properties: LengthPrefixedVec::default(),
    };

    conn_write.send_packet(login_success)?;

    // Build PlayerIdentity for server-side tracking
    let player_identity = PlayerIdentity::new(login_start.username.clone(), login_start.uuid);

    // =============================================================================================
    // 4 Wait for client Login Acknowledged packet
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
        crate::packets::incoming::login_acknowledged::LoginAcknowledgedPacket::decode(
            &mut skel.data,
            &NetDecodeOpts::None,
        )?;

    // =============================================================================================
    // 5 Read Client Information (locale, view distance, etc.)
    let mut skel = PacketSkeleton::new(conn_read, compressed, Configuration).await?;
    let expected_id = lookup_packet!("configuration", "serverbound", "client_information");
    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Configuration,
        }));
    }

    let client_info = crate::packets::incoming::client_information::ClientInformation::decode(
        &mut skel.data,
        &NetDecodeOpts::None,
    )?;

    trace!(
        "Client information: {{ locale: {}, view_distance: {}, chat_mode: {}, chat_colors: {}, displayed_skin_parts: {} }}",
        client_info.locale,
        client_info.view_distance,
        client_info.chat_mode,
        client_info.chat_colors,
        client_info.displayed_skin_parts
    );

    // =============================================================================================
    // 6 Send known resource packs list
    let client_bound_known_packs =
        crate::packets::outgoing::client_bound_known_packs::ClientBoundKnownPacksPacket::new();
    conn_write.send_packet(client_bound_known_packs)?;

    // =============================================================================================
    // 7 Read client's selected known packs (currently ignored)
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
        crate::packets::incoming::server_bound_known_packs::ServerBoundKnownPacks::decode(
            &mut skel.data,
            &NetDecodeOpts::None,
        )?;

    // =============================================================================================
    // 8 Send server registry data (dimensions, biomes, etc.)
    for packet in &*REGISTRY_PACKETS {
        conn_write.send_packet_ref(packet)?;
    }

    // =============================================================================================
    // 9 Signal end of configuration phase
    let finish_config_packet =
        crate::packets::outgoing::finish_configuration::FinishConfigurationPacket;
    conn_write.send_packet(finish_config_packet)?;

    // =============================================================================================
    // 10 Wait for client's finish_configuration ack
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
        crate::packets::incoming::ack_finish_configuration::AckFinishConfigurationPacket::decode(
            &mut skel.data,
            &NetDecodeOpts::None,
        )?;

    // =============================================================================================
    // 11 Send login_play packet to switch to Play state

    let gamemode_to_send = state
        .player_cache
        .get(&player_identity.uuid)
        .map(|data| data.gamemode)
        .unwrap_or_default();

    let login_play = crate::packets::outgoing::login_play::LoginPlayPacket::new(
        player_identity.short_uuid,
        gamemode_to_send.0 as u8,
    );
    conn_write.send_packet(login_play)?;

    // =============================================================================================
    // 12 Send initial Player Abilities packet
    // We send this to sync the client with the cached player's abilities

    let abilities_to_send = state
        .player_cache
        .get(&player_identity.uuid)
        .map(|data| data.abilities)
        .unwrap_or_default();

    let abilities_packet =
        crate::packets::outgoing::player_abilities::PlayerAbilities::from_abilities(
            &abilities_to_send,
        );
    conn_write.send_packet(abilities_packet)?;

    // =============================================================================================
    // 13 Send entity status to grant Op Level
    // TODO: Replace this with actual OP code of the player
    let op_level_packet = crate::packets::outgoing::entity_event::EntityStatus {
        entity_id: player_identity.short_uuid, // same ID as LoginPlayPacket
        status: 28,                            // Status 28 = OP level 4
    };
    conn_write.send_packet(op_level_packet)?;

    // =============================================================================================
    // 14 Send initial player position sync (requires teleport confirmation)
    let teleport_id_i32: i32 = (rand::random::<u32>() & 0x3FFF_FFFF) as i32;
    let sync_player_pos =
        crate::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket {
            teleport_id: VarInt::new(teleport_id_i32),
            ..Default::default()
        };
    conn_write.send_packet(sync_player_pos)?;

    // =============================================================================================
    // 15 Await client's teleport acceptance
    // The client may send other packets (like client_tick_end) before accepting the teleport,
    // so we loop until we get the accept_teleportation packet
    let expected_id = lookup_packet!("play", "serverbound", "accept_teleportation");
    let confirm_player_teleport = loop {
        let mut skel = PacketSkeleton::new(conn_read, compressed, Play).await?;
        if skel.id == expected_id {
            // Got the teleport confirmation
            let confirm =
                crate::packets::incoming::confirm_player_teleport::ConfirmPlayerTeleport::decode(
                    &mut skel.data,
                    &NetDecodeOpts::None,
                )?;
            break confirm;
        } else {
            // Client sent another packet before confirming teleport - just ignore it
            trace!(
                "Ignoring packet 0x{:02X} while waiting for teleport confirmation",
                skel.id
            );
        }
    };

    if confirm_player_teleport.teleport_id.0 != teleport_id_i32 {
        error!(
            "Teleport ID mismatch: expected {}, got {}",
            teleport_id_i32, confirm_player_teleport.teleport_id.0
        )
    }

    // =============================================================================================
    // 16 Receive first movement packet from player
    // Similarly, the client may send other packets before the movement packet
    let expected_id = lookup_packet!("play", "serverbound", "move_player_pos_rot");
    let _player_pos_and_rot = loop {
        let mut skel = PacketSkeleton::new(conn_read, compressed, Play).await?;

        if skel.id == expected_id {
            let pos_rot = crate::packets::incoming::set_player_position_and_rotation::SetPlayerPositionAndRotationPacket::decode(
                &mut skel.data,
                &NetDecodeOpts::None,
            )?;
            break pos_rot;
        } else {
            // Client sent another packet before movement - ignore it
            trace!(
                "Ignoring packet 0x{:02X} while waiting for initial movement packet",
                skel.id
            );
        }
    };

    // =============================================================================================
    // 17 Send initial game event (e.g., "change game mode")
    let game_event = crate::packets::outgoing::game_event::GameEventPacket::new(13, 0.0);
    conn_write.send_packet(game_event)?;

    // =============================================================================================
    // 18 Send center chunk packet (player spawn location)
    let center_chunk = crate::packets::outgoing::set_center_chunk::SetCenterChunk::new(0, 0);
    conn_write.send_packet(center_chunk)?;

    // =============================================================================================
    // 19 Load and send surrounding chunks within render distance
    let radius = get_global_config().chunk_render_distance as i32;

    let mut batch = state.thread_pool.batch();

    for x in -radius..=radius {
        for z in -radius..=radius {
            batch.execute({
                let state = state.clone();
                move || -> Result<Vec<u8>, NetError> {
                    let chunk = state.world.load_chunk(x, z, "overworld")?;
                    let chunk_data =
                        crate::packets::outgoing::chunk_and_light_data::ChunkAndLightData::from_chunk(
                            &chunk,
                        )?;
                    let compressed_packet = compress_packet(&chunk_data, compressed, &NetEncodeOpts::WithLength, 512)?;
                    Ok(compressed_packet)
                }
            });
        }
    }

    let packets = batch.wait();

    for packet in packets {
        match packet {
            Ok(data) => {
                conn_write.send_raw_packet(data)?;
            }
            Err(err) => {
                error!("Failed to send chunk data: {:?}", err);
                return Err(NetError::Misc(format!(
                    "Failed to send chunk data: {:?}",
                    err
                )));
            }
        }
    }

    // =============================================================================================
    conn_write.send_packet(CommandsPacket::new())?;

    trace!(
        "sending command graph {:#?}",
        ferrumc_commands::infrastructure::get_graph()
    );

    // =============================================================================================
    // Login sequence complete
    Ok((
        false,
        LoginResult {
            player_identity: Some(player_identity),
            compression: compressed,
        },
    ))
}
