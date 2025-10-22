use bevy_ecs::{
    event::EventWriter,
    prelude::{Entity, Query, Res},
};
use ferrumc_core::{
    conn::player_disconnect_event::PlayerDisconnectEvent,
    data::player::PlayerData,
    identity::player_identity::PlayerIdentity,
    transform::{grounded::OnGround, position::Position, rotation::Rotation},
};
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;
use ferrumc_text::TextComponent;

pub fn handle(
    query: Query<(
        Entity,
        &StreamWriter,
        &PlayerIdentity,
        &Position,
        &OnGround,
        &Rotation,
    )>,
    mut dispatch_events: EventWriter<PlayerDisconnectEvent>,
    state: Res<GlobalStateResource>,
) {
    let packet = ferrumc_net::packets::outgoing::disconnect::DisconnectPacket {
        reason: TextComponent::from("Server is shutting down"),
    };
    for (entity, conn, identity, position, on_ground, rotation) in query.iter() {
        if state.0.players.is_connected(entity) {
            let player_disconnect = PlayerDisconnectEvent {
                data: PlayerData::new(position, on_ground.0, "overworld", rotation),
                identity: identity.to_owned(),
                entity,
            };
            dispatch_events.write(player_disconnect);
            if let Err(e) = conn.send_packet_ref(&packet) {
                tracing::error!(
                    "Failed to send shutdown packet to player {}: {}",
                    identity.username,
                    e
                );
            } else {
                tracing::info!("Shutdown packet sent to player {}", identity.username);
            }
        }
    }
}
