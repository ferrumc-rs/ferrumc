use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_ecs::components::storage::ComponentRefMut;
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
use ferrumc_net::packets::outgoing::login_play::LoginPlayPacket;
use ferrumc_net::packets::outgoing::registry_data::get_registry_packets;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net::packets::outgoing::set_default_spawn_position::SetDefaultSpawnPositionPacket;
use ferrumc_net::packets::outgoing::set_render_distance::SetRenderDistance;
use ferrumc_net::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::GlobalState;
use tracing::{debug, trace};
use ferrumc_net::packets::outgoing::player_info_update::{PlayerInfoUpdatePacket, PlayerInfo};
use ferrumc_net::packets::outgoing::spawn_entity::SpawnEntityPacket;
use ferrumc_net::packets::outgoing::destroy_entity::DestroyEntitiesPacket;
use ferrumc_net::packets::outgoing::player_info_remove::PlayerInfoRemovePacket;
use crate::events::PlayerStartLoginEvent;
use ferrumc_events::errors::EventsError;
use ferrumc_events::infrastructure::Event;
use ferrumc_net::{utils::broadcast::*, events::PlayerQuitEvent};

#[event_handler]
async fn handle_login_start(
    login_start_event: LoginStartEvent,
    state: GlobalState,
) -> Result<LoginStartEvent, NetError> {
    let uuid = login_start_event.login_start_packet.uuid;
    let username = login_start_event.login_start_packet.username.as_str();
    debug!("Handling login start event for user: {username}, uuid: {uuid}");

    // Add the player identity component to the ECS for the entity.
    let event = PlayerStartLoginEvent {
        entity: login_start_event.conn_id,
        profile: PlayerIdentity::new(username.to_string(), uuid),
    };

    match PlayerStartLoginEvent::trigger(event, state.clone()).await {
        Err(NetError::Kick(msg)) => Err(NetError::Kick(msg)),
        Err(NetError::EventsError(EventsError::Cancelled)) => Ok(login_start_event),
        Ok(event) => {
            crate::send_login_success(state, login_start_event.conn_id, event.profile).await?;
            Ok(login_start_event)
        },
        e => e.map(|_| login_start_event),
    }
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

    let conn_id = ack_finish_configuration_event.conn_id;

    let mut conn_state = state.universe.get_mut::<ConnectionState>(conn_id)?;

    *conn_state = ConnectionState::Play;

    let chunk = state.world.load_chunk(0, 0, "overworld").await.ok();

    let y = if let Some(ref chunk) = chunk {
        (chunk.heightmaps.motion_blocking_height(0, 0)) as f64
    } else {
        256.0
    };

    // add components to the entity after the connection state has been set to play.
    // to avoid wasting resources on entities that are fetching stuff like server status etc.
    state
        .universe
        .add_component::<Position>(conn_id, Position::new(0.0, y, 0.0))?
        .add_component::<Rotation>(conn_id, Rotation::new(0.0, 0.0))?
        .add_component::<OnGround>(conn_id, OnGround::default())?;

    let mut writer = state.universe.get_mut::<StreamWriter>(conn_id)?;

    writer // 21
        .send_packet(&LoginPlayPacket::new(conn_id), &NetEncodeOpts::WithLength)
        .await?;
    writer // 29
        .send_packet(
            &SynchronizePlayerPositionPacket::from_player(conn_id, state.clone())?, // The coordinates here should be used for the center chunk.
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

    send_keep_alive(conn_id, state.clone(), &mut writer).await?;

    if let Some(ref chunk) = chunk {
        writer.send_packet(&ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData::from_chunk(chunk)?, &NetEncodeOpts::WithLength).await?;
    }

    // todos in this code below
    // - fix where sometimes players don't get spawned
    // - make sure to only spawn players that are in range
    for (entity, profile) in state.universe.query::<&PlayerIdentity>() {
        // spawn all players but ours in server for new connection
        if entity != conn_id {
            // send player info update
            writer.send_packet(&PlayerInfoUpdatePacket::new(vec![
                PlayerInfo::from(&profile)
            ]), &NetEncodeOpts::WithLength).await?;
            // send spawn entity packet
            writer.send_packet(&SpawnEntityPacket::new(conn_id, state.clone())?, &NetEncodeOpts::WithLength).await?;
        }
    }

    drop(writer);

    state.universe.add_component::<ChunkReceiver>(conn_id, ChunkReceiver::default())?;
    let pos = state.universe.get::<Position>(conn_id)?;
    let mut chunk_recv = state.universe.get_mut::<ChunkReceiver>(conn_id)?;
    chunk_recv.last_chunk = Some((pos.x as i32, pos.z as i32, String::from("overworld")));
    chunk_recv.calculate_chunks().await;
    drop(chunk_recv);

    // broadcast player info update
    let profile = state
        .universe
        .get::<PlayerIdentity>(ack_finish_configuration_event.conn_id)?;

    state.broadcast(&PlayerInfoUpdatePacket::new(vec![
        PlayerInfo::from(&profile)
    ]), BroadcastOptions::default()).await?;

    // broadcast spawn entity packet for everyone online but current connection
    state.broadcast(&SpawnEntityPacket::new(conn_id, state.clone())?, BroadcastOptions::default().except(vec![conn_id])).await?;

    Ok(ack_finish_configuration_event)
}

#[event_handler]
async fn handle_player_quit(
    event: PlayerQuitEvent,
    state: GlobalState,
) -> Result<PlayerQuitEvent, NetError> {
    let conn_id = event.entity;
    let profile = state
        .universe
        .get::<PlayerIdentity>(conn_id)?;
    state.broadcast(&PlayerInfoRemovePacket::new(vec![profile.uuid]), BroadcastOptions::default()).await?;
    state.broadcast(&DestroyEntitiesPacket::new(vec![conn_id]), BroadcastOptions::default().except(vec![conn_id])).await?;
    Ok(event)
}

async fn send_keep_alive(
    conn_id: usize,
    state: GlobalState,
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
