use crate::conn_init::NetDecodeOpts;
use crate::conn_init::VarInt;
use crate::conn_init::{send_packet, trim_packet_head};
use crate::errors::{NetError, PacketError};
use crate::packets::outgoing::registry_data::REGISTRY_PACKETS;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net_codec::decode::NetDecode;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_state::GlobalState;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tracing::{error, trace};

pub(super) async fn login(
    mut conn_read: &mut OwnedReadHalf,
    conn_write: &mut OwnedWriteHalf,
    state: GlobalState,
) -> Result<(bool, Option<PlayerIdentity>), NetError> {
    // =============================================================================================
    trim_packet_head(conn_read, 0x00).await?;

    let login_start = crate::packets::incoming::login_start::LoginStartPacket::decode_async(
        &mut conn_read,
        &NetDecodeOpts::None,
    )
        .await?;

    // =============================================================================================

    let login_success = crate::packets::outgoing::login_success::LoginSuccessPacket {
        uuid: login_start.uuid,
        username: &login_start.username,
        properties: LengthPrefixedVec::default(),
    };

    send_packet(conn_write, &login_success).await?;

    let player_identity = PlayerIdentity::new(login_start.username.clone(), login_start.uuid);

    // =============================================================================================

    trim_packet_head(conn_read, 0x03).await?;

    // The login ack packet doesn't contain any data, so we just need to read it
    let _ = crate::packets::incoming::login_acknowledged::LoginAcknowledgedPacket::decode_async(
        &mut conn_read,
        &NetDecodeOpts::None,
    )
        .await?;

    // =============================================================================================

    // The server bound plugin message packet is a bit special, since the inner fields aren't length
    // prefixed, so we need to read the length prefix first, and then read the rest of the packet

    let len = VarInt::decode_async(&mut conn_read, &NetDecodeOpts::None).await?;
    let id = VarInt::decode_async(&mut conn_read, &NetDecodeOpts::None).await?;
    assert_eq!(id.0, 0x02);
    // Limit the buffer to this max length, so we don't allocate too much memory
    // The wiki says it can't be larger than 1048576, but we add 64 just to be safe
    if len.0 < 1 || len.0 > 1048576 + 64 {
        error!("Received packet with invalid length: {}", len.0);
        return Err(NetError::Packet(PacketError::MalformedPacket(Some(
            id.0 as u8,
        ))));
    }
    let mut buf = vec![0; len.0 as usize - id.len()];
    conn_read.read_exact(&mut buf).await?;

    // =============================================================================================

    trim_packet_head(conn_read, 0x00).await?;

    let client_info =
        crate::packets::incoming::client_information::ClientInformation::decode_async(
            &mut conn_read,
            &NetDecodeOpts::None,
        )
            .await?;

    trace!(
        "Client information: {{ locale: {}, view_distance: {}, chat_mode: {}, chat_colors: {}, displayed_skin_parts: {} }}",
        client_info.locale,
        client_info.view_distance,
        client_info.chat_mode,
        client_info.chat_colors,
        client_info.displayed_skin_parts
    );

    // =============================================================================================

    let client_bound_known_packs =
        crate::packets::outgoing::client_bound_known_packs::ClientBoundKnownPacksPacket::default();

    send_packet(conn_write, &client_bound_known_packs).await?;

    // =============================================================================================

    trim_packet_head(conn_read, 0x07).await?;

    // What are we supposed to do with this packet?
    let _server_bound_known_packs =
        crate::packets::incoming::server_bound_known_packs::ServerBoundKnownPacks::decode_async(
            &mut conn_read,
            &NetDecodeOpts::None,
        )
            .await?;

    // =============================================================================================

    for packet in &*REGISTRY_PACKETS {
        send_packet(conn_write, packet).await?;
    }

    // =============================================================================================

    let finish_config_packet =
        crate::packets::outgoing::finish_configuration::FinishConfigurationPacket;

    send_packet(conn_write, &finish_config_packet).await?;

    // =============================================================================================

    trim_packet_head(conn_read, 0x03).await?;

    let _finish_config_ack =
        crate::packets::incoming::ack_finish_configuration::AckFinishConfigurationPacket::decode_async(
            &mut conn_read,
            &NetDecodeOpts::None,
        )
            .await?;

    // =============================================================================================

    let login_play =
        crate::packets::outgoing::login_play::LoginPlayPacket::new(player_identity.short_uuid);

    send_packet(conn_write, &login_play).await?;

    // =============================================================================================

    let teleport_id_i32 = rand::random();

    let sync_player_pos =
        crate::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket {
            teleport_id: VarInt::new(teleport_id_i32),
            ..Default::default()
        };

    send_packet(conn_write, &sync_player_pos).await?;

    // =============================================================================================

    trim_packet_head(conn_read, 0x00).await?;

    let confirm_player_teleport =
        crate::packets::incoming::confirm_player_teleport::ConfirmPlayerTeleport::decode_async(
            &mut conn_read,
            &NetDecodeOpts::None,
        )
            .await?;

    if confirm_player_teleport.teleport_id.0 != teleport_id_i32 {
        error!(
            "Teleport ID mismatch: expected {}, got {}",
            teleport_id_i32, confirm_player_teleport.teleport_id.0
        )
    }

    // =============================================================================================

    trim_packet_head(conn_read, 0x1D).await?;

    let _player_pos_and_rot =
        crate::packets::incoming::set_player_position_and_rotation::SetPlayerPositionAndRotationPacket::decode_async(
            &mut conn_read,
            &NetDecodeOpts::None,
        ).await?;

    // =============================================================================================

    let game_event = crate::packets::outgoing::game_event::GameEventPacket::new(13, 0.0);

    send_packet(conn_write, &game_event).await?;

    // =============================================================================================

    let center_chunk = crate::packets::outgoing::set_center_chunk::SetCenterChunk::new(0, 0);

    send_packet(conn_write, &center_chunk).await?;

    // =============================================================================================

    let radius = get_global_config().chunk_render_distance as i32;

    let mut batch = state.thread_pool.batch();

    for x in -radius..=radius {
        for z in -radius..=radius {
            batch.execute({
                let state = state.clone();
                move || {
                    let chunk = state.world.load_chunk(x, z, "overworld")?;
                    crate::packets::outgoing::chunk_and_light_data::ChunkAndLightData::from_chunk(
                        &chunk,
                    )
                }
            });
        }
    }
    let chunks = batch.wait();
    for chunk in chunks {
        match chunk {
            Ok(chunk_data) => {
                send_packet(conn_write, &chunk_data).await?;
            }
            Err(err) => {
                error!("Failed to send chunk data: {:?}", err);
            }
        }
    }

    conn_write.flush().await?;

    Ok((false, Some(player_identity)))
}
