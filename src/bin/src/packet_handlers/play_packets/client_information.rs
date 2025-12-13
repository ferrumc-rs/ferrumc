//! Handler for ClientInformation packets received during play state.
//!
//! When a player changes their settings (view distance, locale, skin parts, etc.),
//! the client sends a ClientInformation packet. This handler:
//! 1. Updates the player's ClientInformation ECS component
//! 2. Fires a ClientInformationUpdated message if view distance changed
//!    (which triggers chunk recalculation)

use bevy_ecs::prelude::{MessageWriter, Query, Res};
use ferrumc_components::chunks::{ChunkCommand, ChunkSender};
use ferrumc_components::player::client_information::ClientInformation;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::transform::position::Position;
use ferrumc_messages::ClientInformationUpdated;
use ferrumc_net::packets::incoming::client_information_play::{
    ChatMode, MainHand, ParticleStatus,
};
use ferrumc_net::ClientInformationPlayReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, trace, warn};

/// Handles ClientInformation packets from clients during play state.
///
/// When a player changes their settings in the options menu, this handler:
/// 1. Updates the ClientInformation ECS component
/// 2. If view distance changed, sends a chunk update command to the async loader
/// 3. Fires a ClientInformationUpdated message for other systems to react
pub fn handle(
    receiver: Res<ClientInformationPlayReceiver>,
    mut query: Query<(&mut ClientInformation, &Position, &ChunkSender)>,
    state: Res<GlobalStateResource>,
    mut events: MessageWriter<ClientInformationUpdated>,
) {
    for (packet, entity) in receiver.0.try_iter() {
        // Check if player is still connected
        if !state.0.players.is_connected(entity) {
            trace!(
                "Skipping ClientInformation for disconnected player {:?}",
                entity
            );
            continue;
        }

        // Get the player's components
        let Ok((mut client_info, position, chunk_sender)) = query.get_mut(entity) else {
            warn!(
                "Failed to get ClientInformation components for entity {:?}",
                entity
            );
            continue;
        };

        // Store old view distance to detect changes
        let old_view_distance = client_info.view_distance;

        // Update the ECS component with new values
        client_info.locale = packet.locale.clone();
        client_info.view_distance = packet.view_distance.max(2) as u8; // Clamp to minimum 2
        client_info.chat_mode = match packet.chat_mode {
            ChatMode::Enabled => 0,
            ChatMode::CommandsOnly => 1,
            ChatMode::Hidden => 2,
        };
        client_info.chat_colors = packet.chat_colors;
        client_info.displayed_skin_parts = packet.displayed_skin_parts;
        client_info.main_hand = match packet.main_hand {
            MainHand::Left => 0,
            MainHand::Right => 1,
        };
        client_info.enable_text_filtering = packet.enable_text_filtering;
        client_info.allow_server_listings = packet.allow_server_listings;
        client_info.particle_status = match packet.particle_status {
            ParticleStatus::All => 0,
            ParticleStatus::Decreased => 1,
            ParticleStatus::Minimal => 2,
        };

        let new_view_distance = client_info.view_distance;

        debug!(
            "Updated client settings for {:?}: locale={}, view_distance={} (was {})",
            entity, client_info.locale, new_view_distance, old_view_distance
        );

        // If view distance changed, trigger chunk recalculation
        if old_view_distance != new_view_distance {
            let server_render_distance = get_global_config().chunk_render_distance as u8;
            let effective_radius = server_render_distance.min(new_view_distance);

            // Calculate current chunk position
            let chunk_x = position.x.floor() as i32 >> 4;
            let chunk_z = position.z.floor() as i32 >> 4;

            debug!(
                "View distance changed for {:?}: {} -> {}, effective radius: {}, triggering chunk recalculation",
                entity, old_view_distance, new_view_distance, effective_radius
            );

            // Send update to the async chunk loader
            if let Err(e) = chunk_sender.tx.try_send(ChunkCommand::UpdateCenter {
                chunk_x,
                chunk_z,
                radius: effective_radius,
            }) {
                warn!(
                    "Failed to send chunk update command for {:?}: {:?}",
                    entity, e
                );
            }

            // Fire event for other systems
            events.write(ClientInformationUpdated::new(
                entity,
                old_view_distance,
                new_view_distance,
            ));
        }
    }
}
