use crate::conn_init::{LoginResult, NetDecodeOpts};
use crate::conn_init::VarInt;
use crate::connection::StreamWriter;
use crate::errors::{NetError, PacketError};
use crate::packets::outgoing::registry_data::REGISTRY_PACKETS;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net_codec::decode::NetDecode;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_state::GlobalState;
use tokio::net::tcp::{OwnedReadHalf};
use tracing::{error, trace};
use uuid::Uuid;
use ferrumc_macros::lookup_packet;
use ferrumc_net_codec::encode::NetEncodeOpts;
use crate::compression::compress_packet;
use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use crate::ConnState::*;

pub(super) async fn login(
    conn_read: &mut OwnedReadHalf,
    conn_write: &StreamWriter,
    state: GlobalState,
) -> Result<(bool, LoginResult), NetError> {
    let mut compressed = false;

    // =============================================================================================
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
    if get_global_config().network_compression_threshold > 0 {
        compressed = true;

        // Send SetCompression packet to the client
        let compression_packet = crate::packets::outgoing::set_compression::SetCompressionPacket {
            threshold: VarInt::new(get_global_config().network_compression_threshold),
        };
        conn_write.send_packet(compression_packet)?;
        conn_write.compress.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    // =============================================================================================
    // Send login success packet
    let login_success = crate::packets::outgoing::login_success::LoginSuccessPacket {
        uuid: login_start.uuid,
        username: &login_start.username,
        properties: LengthPrefixedVec::default(),
    };

    conn_write.send_packet(login_success)?;

    let player_identity = PlayerIdentity {
        uuid: Uuid::from_u128(login_start.uuid),
        username: login_start.username.clone(),
        short_uuid: login_start.uuid as i32,
    };
    
    // =============================================================================================
    
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
    
    // Read client information packet
    let mut skel = PacketSkeleton::new(conn_read, compressed, Configuration).await?;
    let expected_id = lookup_packet!("configuration", "serverbound", "client_information");
    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Configuration,
        }));
    }

    let client_info =
        crate::packets::incoming::client_information::ClientInformation::decode(
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
    // Send client_bound_known_packs packet
    let client_bound_known_packs =
        crate::packets::outgoing::client_bound_known_packs::ClientBoundKnownPacksPacket::new();
    conn_write.send_packet(client_bound_known_packs)?;

    // =============================================================================================
    // Read select_known_packs packet
    let mut skel = PacketSkeleton::new(conn_read, compressed, Configuration).await?;
    let expected_id = lookup_packet!("configuration", "serverbound", "select_known_packs");
    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Configuration,
        }));
    }

    // What to do with this packet? For now, we ignore it
    let _server_bound_known_packs =
        crate::packets::incoming::server_bound_known_packs::ServerBoundKnownPacks::decode(
            &mut skel.data,
            &NetDecodeOpts::None,
        )?;

    // =============================================================================================
    // Send all registry packets
    for packet in &*REGISTRY_PACKETS {
        conn_write.send_packet_ref(packet)?;
    }

    // =============================================================================================
    // Send finish_configuration packet
    let finish_config_packet =
        crate::packets::outgoing::finish_configuration::FinishConfigurationPacket;
    conn_write.send_packet(finish_config_packet)?;

    // =============================================================================================
    // Read finish_configuration ack
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
    // Send login/play packet
    let login_play =
        crate::packets::outgoing::login_play::LoginPlayPacket::new(player_identity.short_uuid);
    conn_write.send_packet(login_play)?;

    // =============================================================================================
    // Generate teleport ID and synchronize player position
    let teleport_id_i32: i32 = (rand::random::<u32>() & 0x3FFF_FFFF) as i32;
    let sync_player_pos =
        crate::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket {
            teleport_id: VarInt::new(teleport_id_i32),
            ..Default::default()
        };
    conn_write.send_packet(sync_player_pos)?;

    // =============================================================================================
    // Read accept_teleportation packet
    let mut skel = PacketSkeleton::new(conn_read, compressed, Play).await?;
    let expected_id = lookup_packet!("play", "serverbound", "accept_teleportation");
    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Play,
        }));
    }

    let confirm_player_teleport =
        crate::packets::incoming::confirm_player_teleport::ConfirmPlayerTeleport::decode(
            &mut skel.data,
            &NetDecodeOpts::None,
        )?;

    if confirm_player_teleport.teleport_id.0 != teleport_id_i32 {
        error!(
            "Teleport ID mismatch: expected {}, got {}",
            teleport_id_i32, confirm_player_teleport.teleport_id.0
        )
    }

    // =============================================================================================
    // Read move_player_pos_rot packet
    let mut skel = PacketSkeleton::new(conn_read, compressed, Play).await?;
    let expected_id = lookup_packet!("play", "serverbound", "move_player_pos_rot");
    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Play,
        }));
    }

    let _player_pos_and_rot =
        crate::packets::incoming::set_player_position_and_rotation::SetPlayerPositionAndRotationPacket::decode(
            &mut skel.data,
            &NetDecodeOpts::None,
        )?;

    // =============================================================================================
    // Send game event packet
    let game_event = crate::packets::outgoing::game_event::GameEventPacket::new(13, 0.0);
    conn_write.send_packet(game_event)?;

    // =============================================================================================
    // Send center chunk packet
    let center_chunk = crate::packets::outgoing::set_center_chunk::SetCenterChunk::new(0, 0);
    conn_write.send_packet(center_chunk)?;

    // =============================================================================================
    // Load and send chunks within render distance
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
                    let compressed_packet = compress_packet(&chunk_data, compressed, &NetEncodeOpts::WithLength)?;
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
                return Err(NetError::Misc(
                    format!("Failed to send chunk data: {:?}", err),
                ));
            }
        }
    }

    Ok((false, LoginResult {
        player_identity: Some(player_identity),
        compression: compressed,
    }))
}
