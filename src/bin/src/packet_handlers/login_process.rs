use ferrumc_commands::graph::CommandsPacket;
use ferrumc_config::statics::{get_global_config, get_whitelist};
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_ecs::components::storage::ComponentRefMut;
use ferrumc_ecs::entities::Entity;
use ferrumc_macros::event_handler;
use ferrumc_net::connection::{ConnectionState, StreamWriter};
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::ack_finish_configuration::AckFinishConfigurationEvent;
use ferrumc_net::packets::incoming::keep_alive::IncomingKeepAlivePacket;
use ferrumc_net::packets::incoming::login_acknowledged::LoginAcknowledgedEvent;
use ferrumc_net::packets::incoming::login_start::LoginStartEvent;
use ferrumc_net::packets::incoming::server_bound_known_packs::ServerBoundKnownPacksEvent;
use ferrumc_net::packets::outgoing::client_bound_known_packs::ClientBoundKnownPacksPacket;
use ferrumc_net::packets::outgoing::finish_configuration::FinishConfigurationPacket;
use ferrumc_net::packets::outgoing::game_event::GameEventPacket;
use ferrumc_net::packets::outgoing::keep_alive::OutgoingKeepAlivePacket;
use ferrumc_net::packets::outgoing::login_disconnect::LoginDisconnectPacket;
use ferrumc_net::packets::outgoing::login_play::LoginPlayPacket;
use ferrumc_net::packets::outgoing::login_success::LoginSuccessPacket;
use ferrumc_net::packets::outgoing::player_info_update::PlayerInfoUpdatePacket;
use ferrumc_net::packets::outgoing::registry_data::get_registry_packets;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net::packets::outgoing::set_default_spawn_position::SetDefaultSpawnPositionPacket;
use ferrumc_net::packets::outgoing::set_render_distance::SetRenderDistance;
use ferrumc_net::packets::outgoing::spawn_entity::SpawnEntityPacket;
use ferrumc_net::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;
use ferrumc_net::utils::broadcast::{broadcast, get_all_play_players, BroadcastOptions};
use ferrumc_net::NetResult;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::GlobalState;
use futures::StreamExt;
use std::time::Instant;
use tracing::{debug, trace};

#[event_handler]
async fn handle_login_start(
    login_start_event: LoginStartEvent,
    state: GlobalState,
) -> Result<LoginStartEvent, NetError> {
    let uuid = login_start_event.login_start_packet.uuid;
    let username = login_start_event.login_start_packet.username.as_str();
    let player_identity = PlayerIdentity::new(username.to_string(), uuid);
    debug!("Handling login start event for user: {username}, uuid: {uuid}");

    // Add the player identity component to the ECS for the entity.
    state
        .universe
        .add_component::<PlayerIdentity>(
            login_start_event.conn_id,
            PlayerIdentity::new(username.to_string(), uuid),
        )?
        /*.add_component::<ChunkReceiver>(login_start_event.conn_id, ChunkReceiver::default())?*/;

    //Send a Login Success Response to further the login sequence
    let mut writer = state
        .universe
        .get_mut::<StreamWriter>(login_start_event.conn_id)?;

    if get_global_config().whitelist {
        let whitelist = get_whitelist();

        if whitelist.get(&uuid).is_none() {
            writer
                .send_packet(
                    &LoginDisconnectPacket::new(
                        "{\"translate\":\"multiplayer.disconnect.not_whitelisted\"}",
                    ),
                    &NetEncodeOpts::WithLength,
                )
                .await?;
            return Ok(login_start_event);
        }
    }

    // Add the player identity component to the ECS for the entity.
    state
        .universe
        .add_component::<PlayerIdentity>(login_start_event.conn_id, player_identity)?;

    //Send a Login Success Response to further the login sequence
    writer
        .send_packet(
            &LoginSuccessPacket::new(uuid, username),
            &NetEncodeOpts::WithLength,
        )
        .await?;

    Ok(login_start_event)
}

#[event_handler]
async fn handle_login_acknowledged(
    login_acknowledged_event: LoginAcknowledgedEvent,
    state: GlobalState,
) -> Result<LoginAcknowledgedEvent, NetError> {
    trace!("Handling Login Acknowledged event");

    //Set the connection State to Configuration
    let mut connection_state = state
        .universe
        .get_mut::<ConnectionState>(login_acknowledged_event.conn_id)?;

    *connection_state = ConnectionState::Configuration;

    // Send packets packet
    let client_bound_known_packs = ClientBoundKnownPacksPacket::new();

    let mut writer = state
        .universe
        .get_mut::<StreamWriter>(login_acknowledged_event.conn_id)?;

    writer
        .send_packet(&client_bound_known_packs, &NetEncodeOpts::WithLength)
        .await?;

    Ok(login_acknowledged_event)
}

#[event_handler]
async fn handle_server_bound_known_packs(
    server_bound_known_packs_event: ServerBoundKnownPacksEvent,
    state: GlobalState,
) -> Result<ServerBoundKnownPacksEvent, NetError> {
    trace!("Handling Server Bound Known Packs event");

    let mut writer = state
        .universe
        .get_mut::<StreamWriter>(server_bound_known_packs_event.conn_id)?;

    let registry_packets = get_registry_packets();
    writer
        .send_packet(&registry_packets, &NetEncodeOpts::None)
        .await?;

    writer
        .send_packet(
            &FinishConfigurationPacket::new(),
            &NetEncodeOpts::WithLength,
        )
        .await?;

    Ok(server_bound_known_packs_event)
}

#[event_handler]
async fn handle_ack_finish_configuration(
    ack_finish_configuration_event: AckFinishConfigurationEvent,
    state: GlobalState,
) -> Result<AckFinishConfigurationEvent, NetError> {
    trace!("Handling Ack Finish Configuration event");
    let entity_id = ack_finish_configuration_event.conn_id;
    {
        let mut conn_state = state.universe.get_mut::<ConnectionState>(entity_id)?;

        *conn_state = ConnectionState::Play;

        // add components to the entity after the connection state has been set to play.
        // to avoid wasting resources on entities that are fetching stuff like server status etc.
        state
            .universe
            .add_component::<Position>(entity_id, Position::default())?
            .add_component::<Rotation>(entity_id, Rotation::default())?
            .add_component::<OnGround>(entity_id, OnGround::default())?
            .add_component::<ChunkReceiver>(entity_id, ChunkReceiver::default())?;

        let mut writer = state.universe.get_mut::<StreamWriter>(entity_id)?;

        writer // 21
            .send_packet(&LoginPlayPacket::new(entity_id), &NetEncodeOpts::WithLength)
            .await?;
        writer // 29
            .send_packet(
                &SynchronizePlayerPositionPacket::default(), // The coordinates here should be used for the center chunk.
                &NetEncodeOpts::WithLength,
            )
            .await?;
        writer // 37
            .send_packet(
                &SetDefaultSpawnPositionPacket::default(), // Player specific, aka. home, bed, where it would respawn.
                &NetEncodeOpts::WithLength,
            )
            .await?;
        writer // 38
            .send_packet(
                &GameEventPacket::start_waiting_for_level_chunks(),
                &NetEncodeOpts::WithLength,
            )
            .await?;
        writer // 41
            .send_packet(
                &SetCenterChunk::new(0, 0), // TODO - Dependent on the player spawn position.
                &NetEncodeOpts::WithLength,
            )
            .await?;
        writer // other
            .send_packet(
                &SetRenderDistance::new(5), // TODO
                &NetEncodeOpts::WithLength,
            )
            .await?;
      
        trace!(
            "Sending command graph: {:#?}",
            ferrumc_commands::infrastructure::get_graph()
        );
        writer
            .send_packet(&CommandsPacket::create(), &NetEncodeOpts::WithLength)
            .await?;

        send_keep_alive(entity_id, &state, &mut writer).await?;

        let pos = state.universe.get_mut::<Position>(entity_id)?;
        let mut chunk_recv = state.universe.get_mut::<ChunkReceiver>(entity_id)?;
        chunk_recv.last_chunk = Some((pos.x as i32, pos.z as i32, String::from("overworld")));
        chunk_recv.calculate_chunks().await;
    }

    Ok(ack_finish_configuration_event)
}
async fn send_keep_alive(
    conn_id: usize,
    state: &GlobalState,
    writer: &mut ComponentRefMut<'_, StreamWriter>,
) -> Result<(), NetError> {
    let keep_alive_packet = OutgoingKeepAlivePacket::default();
    writer
        .send_packet(&keep_alive_packet, &NetEncodeOpts::WithLength)
        .await?;

    let timestamp = keep_alive_packet.timestamp;

    state
        .universe
        .add_component::<OutgoingKeepAlivePacket>(conn_id, keep_alive_packet)?;
    state
        .universe
        .add_component::<IncomingKeepAlivePacket>(conn_id, IncomingKeepAlivePacket { timestamp })?;

    Ok(())
}

async fn player_info_update_packets(entity_id: Entity, state: &GlobalState) -> NetResult<()> {
    // Broadcasts a player info update packet to all players.
    {
        let packet = PlayerInfoUpdatePacket::new_player_join_packet(entity_id, state);

        let start = Instant::now();
        broadcast(
            &packet,
            state,
            BroadcastOptions::default().except([entity_id]),
        )
        .await?;
        trace!(
            "Broadcasting player info update took: {:?}",
            start.elapsed()
        );
    }

    // Tell the player about all the other players that are already connected.
    {
        let packet = PlayerInfoUpdatePacket::existing_player_info_packet(entity_id, state);

        let start = Instant::now();
        let mut writer = state.universe.get_mut::<StreamWriter>(entity_id)?;
        writer
            .send_packet(&packet, &NetEncodeOpts::WithLength)
            .await?;
        debug!("Sending player info update took: {:?}", start.elapsed());
    }

    Ok(())
}

async fn broadcast_spawn_entity_packet(entity_id: Entity, state: &GlobalState) -> NetResult<()> {
    let packet = SpawnEntityPacket::player(entity_id, state)?;

    let start = Instant::now();
    broadcast(
        &packet,
        state,
        BroadcastOptions::default().except([entity_id]),
    )
    .await?;
    trace!("Broadcasting spawn entity took: {:?}", start.elapsed());

    let writer = state.universe.get_mut::<StreamWriter>(entity_id)?;
    futures::stream::iter(get_all_play_players(state))
        .fold(writer, |mut writer, entity| async move {
            if let Ok(packet) = SpawnEntityPacket::player(entity, state) {
                let _ = writer
                    .send_packet(&packet, &NetEncodeOpts::WithLength)
                    .await;
            }
            writer
        })
        .await;

    Ok(())
}
