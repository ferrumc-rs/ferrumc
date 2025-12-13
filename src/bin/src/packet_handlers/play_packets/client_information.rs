//! Handler for ClientInformation packets received during play state.
//!
//! When a player changes their settings (view distance, locale, skin parts, etc.),
//! the client sends a ClientInformation packet. This handler:
//! 1. Updates the player's ClientInformation ECS component
//! 2. Triggers chunk recalculation if view distance changed
//! 3. Fires a `ClientInformationUpdated` message for other systems

use bevy_ecs::prelude::{MessageWriter, Query, Res};
use ferrumc_components::chunks::{ChunkCommand, ChunkSender};
use ferrumc_components::player::client_information::{
    ChatMode, ClientInformation, MainHand, ParticleStatus,
};
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::transform::position::Position;
use ferrumc_messages::ClientInformationUpdated;
use ferrumc_net::ClientInformationPlayReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, trace, warn};

/// Handles `ClientInformation` packets from clients during play state.
///
/// When a player changes their settings in the options menu, this handler:
/// 1. Updates the `ClientInformation` ECS component with all new values
/// 2. If view distance changed, sends a chunk update command to the async loader
/// 3. Fires a `ClientInformationUpdated` message for other systems to react
pub fn handle(
    receiver: Res<ClientInformationPlayReceiver>,
    mut query: Query<(&mut ClientInformation, &Position, &ChunkSender)>,
    state: Res<GlobalStateResource>,
    mut events: MessageWriter<ClientInformationUpdated>,
) {
    for (packet, entity) in receiver.0.try_iter() {
        // Skip disconnected players
        if !state.0.players.is_connected(entity) {
            trace!(
                "Skipping ClientInformation for disconnected entity {:?}",
                entity
            );
            continue;
        }

        // Get the player's components
        let Ok((mut client_info, position, chunk_sender)) = query.get_mut(entity) else {
            warn!("Failed to get components for entity {:?}", entity);
            continue;
        };

        // Capture old view distance before updating
        let old_view_distance = client_info.view_distance;

        // Apply all settings from the packet (converting packet enums to component enums)
        client_info.locale = packet.locale.clone();
        client_info.view_distance = packet.view_distance.max(2) as u8;
        client_info.chat_mode = ChatMode::from(packet.chat_mode as u8);
        client_info.chat_colors = packet.chat_colors;
        client_info.displayed_skin_parts = packet.displayed_skin_parts;
        client_info.main_hand = MainHand::from(packet.main_hand as u8);
        client_info.enable_text_filtering = packet.enable_text_filtering;
        client_info.allow_server_listings = packet.allow_server_listings;
        client_info.particle_status = ParticleStatus::from(packet.particle_status as u8);

        let new_view_distance = client_info.view_distance;

        debug!(
            "Updated settings for {:?}: locale={}, view_distance={} (was {})",
            entity, client_info.locale, new_view_distance, old_view_distance
        );

        // Trigger chunk recalculation if view distance changed
        if old_view_distance != new_view_distance {
            recalculate_chunks(
                entity,
                position,
                chunk_sender,
                old_view_distance,
                new_view_distance,
                &mut events,
            );
        }
    }
}

/// Recalculates chunk loading for a player when their view distance changes.
///
/// This function:
/// 1. Calculates the effective render radius (min of server and client distance)
/// 2. Sends an `UpdateCenter` command to the async chunk loader
/// 3. Fires a `ClientInformationUpdated` event for other systems
fn recalculate_chunks(
    entity: bevy_ecs::entity::Entity,
    position: &Position,
    chunk_sender: &ChunkSender,
    old_view_distance: u8,
    new_view_distance: u8,
    events: &mut MessageWriter<ClientInformationUpdated>,
) {
    let server_render_distance = get_global_config().chunk_render_distance as u8;
    let effective_radius = server_render_distance.min(new_view_distance);

    // Calculate current chunk position
    let chunk_x = position.x.floor() as i32 >> 4;
    let chunk_z = position.z.floor() as i32 >> 4;

    debug!(
        "View distance changed for {:?}: {} -> {}, effective radius: {}",
        entity, old_view_distance, new_view_distance, effective_radius
    );

    // Notify the async chunk loader
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

    // Broadcast event for other systems
    events.write(ClientInformationUpdated::new(
        entity,
        old_view_distance,
        new_view_distance,
    ));
}
