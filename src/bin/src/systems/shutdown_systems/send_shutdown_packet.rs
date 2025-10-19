use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::{data::player::PlayerData, identity::player_identity::PlayerIdentity};
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;
use ferrumc_text::TextComponent;

pub fn handle(
    query: Query<(Entity, &StreamWriter, &PlayerIdentity, &PlayerData)>,
    state: Res<GlobalStateResource>,
) {
    let packet = ferrumc_net::packets::outgoing::disconnect::DisconnectPacket {
        reason: TextComponent::from("Server is shutting down"),
    };

    for (entity, conn, identity, player_data) in query.iter() {
        // I guess that save player state before sending shutdown packet is important to ensure data integrity and prevent data loss.
        if let Err(e) = state
            .0
            .world
            .save_player_state(identity.uuid.as_u128(), player_data)
        {
            tracing::error!(
                "Failed to save player state for {}: {}",
                identity.username,
                e
            );
        } else {
            tracing::info!("Player state saved for {}", identity.username);
        }
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
