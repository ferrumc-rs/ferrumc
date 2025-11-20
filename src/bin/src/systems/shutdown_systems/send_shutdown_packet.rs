use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_components::player::identity::PlayerIdentity;
use ferrumc_components::state::server_state::GlobalStateResource;
use ferrumc_net::connection::StreamWriter;
use ferrumc_text::TextComponent;

pub fn handle(
    query: Query<(Entity, &StreamWriter, &PlayerIdentity)>,
    state: Res<GlobalStateResource>,
) {
    let packet = ferrumc_net::packets::outgoing::disconnect::DisconnectPacket {
        reason: TextComponent::from("Server is shutting down"),
    };

    for (entity, conn, identity) in query.iter() {
        if state.0.players.is_connected(entity) {
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
