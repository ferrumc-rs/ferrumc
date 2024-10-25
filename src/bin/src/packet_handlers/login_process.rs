use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::login_start::LoginStartEvent;
use ferrumc_net::GlobalState;
use tracing::{info, trace};
use ferrumc_ecs::components::storage::ComponentRefMut;
use ferrumc_net::connection::{ConnectionState, StreamWriter};
use ferrumc_net::packets::incoming::ack_finish_configuration::AckFinishConfigurationEvent;
use ferrumc_net::packets::incoming::login_acknowledged::{LoginAcknowledgedEvent};
use ferrumc_net::packets::incoming::server_bound_known_packs::ServerBoundKnownPacksEvent;
use ferrumc_net::packets::outgoing::client_bound_known_packs::ClientBoundKnownPacksPacket;
use ferrumc_net::packets::outgoing::finish_configuration::FinishConfigurationPacket;
use ferrumc_net::packets::outgoing::game_event::GameEventPacket;
use ferrumc_net::packets::outgoing::keep_alive::{KeepAlive, KeepAlivePacket};
use ferrumc_net::packets::outgoing::login_play::LoginPlayPacket;
use ferrumc_net::packets::outgoing::login_success::LoginSuccessPacket;
use ferrumc_net::packets::outgoing::registry_data::{get_registry_packets};
use ferrumc_net::packets::outgoing::set_default_spawn_position::SetDefaultSpawnPositionPacket;
use ferrumc_net::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;
use ferrumc_net_codec::encode::{NetEncodeOpts};

#[event_handler]
async fn handle_login_start(
    login_start_event: LoginStartEvent,
    state: GlobalState,
) -> Result<LoginStartEvent, NetError> {

    info!("Handling login start event");

    let uuid = login_start_event.login_start_packet.uuid;
    let username = login_start_event.login_start_packet.username.clone();
    info!("Received login start from user with username {}", username);

    //Send a Login Success Response to further the login sequence
    let response = LoginSuccessPacket::new(uuid, username);
    let mut writer = state
        .universe
        .get_mut::<StreamWriter>(login_start_event.conn_id)?;

    writer.send_packet(&response, &NetEncodeOpts::WithLength).await?;
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

    writer.send_packet(&client_bound_known_packs, &NetEncodeOpts::WithLength).await?;

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
    writer.send_packet(&registry_packets, &NetEncodeOpts::None).await?;
    
    writer.send_packet(&FinishConfigurationPacket::new(), &NetEncodeOpts::WithLength).await?;

    Ok(server_bound_known_packs_event)
}

#[event_handler]
async fn handle_ack_finish_configuration(
    ack_finish_configuration_event: AckFinishConfigurationEvent,
    state: GlobalState,
) -> Result<AckFinishConfigurationEvent, NetError> {
    trace!("Handling Ack Finish Configuration event");

    let conn_id = ack_finish_configuration_event.conn_id;

    let mut conn_state = state
        .universe
        .get_mut::<ConnectionState>(conn_id)?;
    
    *conn_state = ConnectionState::Play;

    let mut writer = state
        .universe
        .get_mut::<StreamWriter>(conn_id)?;
    
    writer.send_packet(&LoginPlayPacket::new(conn_id), &NetEncodeOpts::WithLength).await?;
    writer.send_packet(&SetDefaultSpawnPositionPacket::default(), &NetEncodeOpts::WithLength).await?;
    writer.send_packet(&SynchronizePlayerPositionPacket::default(), &NetEncodeOpts::WithLength).await?;
    writer.send_packet(&GameEventPacket::start_waiting_for_level_chunks(), &NetEncodeOpts::WithLength).await?;

    send_keep_alive(conn_id, state, &mut writer).await?;
    

    Ok(ack_finish_configuration_event)
}
async fn send_keep_alive(conn_id: usize, state: GlobalState, writer: &mut ComponentRefMut<'_, StreamWriter>) -> Result<(), NetError> {
    let keep_alive_packet = KeepAlivePacket::default();
    writer.send_packet(&keep_alive_packet, &NetEncodeOpts::WithLength).await?;
    
    let id = keep_alive_packet.id;
    
    state.universe.add_component::<KeepAlive>(conn_id, id)?;

    
    Ok(())
}