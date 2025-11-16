use bevy_ecs::prelude::{Commands, Entity, EventWriter, Query, Res}; // Added EventWriter
use ferrumc_components::active_effects::ActiveEffects;
use ferrumc_components::health::Health;
use ferrumc_components::player::abilities::PlayerAbilities;
use ferrumc_components::player::experience::Experience;
use ferrumc_components::player::gamemode::GameModeComponent;
use ferrumc_components::player::gameplay_state::ender_chest::EnderChest;
use ferrumc_components::player::hunger::Hunger;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_events::player_leave::PlayerLeaveEvent;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::player_cache::OfflinePlayerData;
use ferrumc_state::GlobalStateResource;
use ferrumc_text::TextComponent;
use tracing::{info, trace, warn};

type PlayerCacheQuery<'a> = (
    Entity,
    &'a StreamWriter,
    &'a PlayerIdentity,
    &'a PlayerAbilities,
    &'a GameModeComponent,
    &'a Position,
    &'a Rotation,
    &'a Inventory,
    &'a Health,
    &'a Hunger,
    &'a Experience,
    &'a EnderChest,
    &'a ActiveEffects,
);

pub fn connection_killer(
    query: Query<PlayerCacheQuery>,
    mut cmd: Commands,
    state: Res<GlobalStateResource>,
    mut leave_events: EventWriter<PlayerLeaveEvent>,
) {
    // Loop through all entities marked for disconnection
    while let Some((disconnecting_entity, reason)) = state.0.players.disconnection_queue.pop() {
        // Get all components for the disconnecting entity in one go
        let Ok((
            entity,
            conn,
            player_identity,
            abilities,
            gamemode,
            pos,
            rot,
            inv,
            health,
            hunger,
            exp,
            echest,
            effects,
        )) = query.get(disconnecting_entity)
        else {
            warn!("Player's entity has already been removed or is missing components");
            continue;
        };

        // Log the disconnect
        info!(
            "Player {} ({}) disconnected: {}. Caching data...",
            player_identity.username,
            player_identity.uuid,
            reason.as_deref().unwrap_or("No reason")
        );

        // --- 1. Send Disconnect Packet ---
        if conn.running.load(std::sync::atomic::Ordering::Relaxed) {
            trace!(
                "Sending disconnect packet to player {}",
                player_identity.username
            );
            if let Err(e) = conn.send_packet_ref(
                &ferrumc_net::packets::outgoing::disconnect::DisconnectPacket {
                    reason: TextComponent::from(reason.as_deref().unwrap_or("Disconnected")),
                },
            ) {
                warn!(
                    "Failed to send disconnect packet to player {}: {:?}",
                    player_identity.username, e
                );
            }
        } else {
            trace!(
                "Connection for player {} is not running, skipping disconnect packet",
                player_identity.username
            );
        }

        // --- 2. Save Player Data to Cache ---
        let data_to_cache = OfflinePlayerData {
            abilities: abilities.clone(),
            gamemode: gamemode.0,
            position: pos.clone(),
            rotation: *rot,
            inventory: inv.clone(),
            health: *health,
            hunger: *hunger,
            experience: *exp,
            ender_chest: echest.clone(),
            active_effects: effects.clone(),
        };
        state
            .0
            .player_cache
            .insert(player_identity.uuid, data_to_cache);

        // --- 3. Fire PlayerLeaveEvent ---
        leave_events.write(PlayerLeaveEvent(player_identity.clone()));

        // --- 4. Despawn the Entity ---
        cmd.entity(entity).despawn();
    }
}
