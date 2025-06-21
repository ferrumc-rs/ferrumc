use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;
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
            if let Err(e) = conn.send_packet(&packet) {
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
