use serde_json::Value;
use ferrumc_macros::event_handler;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::incoming::login_start::LoginStartEvent;
use ferrumc_net::GlobalState;
use tracing::{info, trace};
use ferrumc_net::connection::{ConnectionState, StreamWriter};
use ferrumc_net::packets::incoming::login_acknowledged::{LoginAcknowledgedEvent};
use ferrumc_net::packets::incoming::server_bound_known_packs::ServerBoundKnownPacksEvent;
use ferrumc_net::packets::outgoing::client_bound_known_packs::ClientBoundKnownPacksPacket;
use ferrumc_net::packets::outgoing::login_success::LoginSuccessPacket;
use ferrumc_net_codec::encode::NetEncodeOpts;

#[event_handler]
async fn handle_login_start(
    login_start_event: LoginStartEvent,
    state: GlobalState,
) -> Result<LoginStartEvent, NetError> {

    info!("Handling login start event");

    let uuid = login_start_event.login_start_packet.uuid;
    let username = login_start_event.login_start_packet.username.clone();
    trace!("Received login start from user with username {}", username);

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

    // Send registry data here.
    let registry_data = register_data::DATA;
    let json : Value = serde_json::from_str(registry_data).unwrap();
    
    let mut writer = state
        .universe
        .get_mut::<StreamWriter>(server_bound_known_packs_event.conn_id)?;
    
    json.as_object().map(|map| map.iter().for_each(|(identifier, value)| {
        
    }));

    Ok(server_bound_known_packs_event)
}

mod register_data {
    pub(super) const DATA: &str = include_str!("../../../../.etc/registry.json");
}