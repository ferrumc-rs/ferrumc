//! Inventory synchronization systems for player equipment visibility.
//!
//! This module implements:
//! - Phase 1: Initial inventory sync on join (sends full inventory)
//! - Phase 2: Equipment broadcast (armor/held items visible to others)
//! - Phase 3: Join equipment exchange (see existing players' gear)
//! - Phase 4: Plugin hooks via messages

use bevy_ecs::prelude::*;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_inventories::hotbar::Hotbar;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_inventories::slot::InventorySlot;
use ferrumc_inventories::sync::{EquipmentState, NeedsInventorySync};
use ferrumc_messages::inventory::{EquipmentChanged, InventorySynced};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_container_content::SetContainerContent;
use ferrumc_net::packets::outgoing::set_equipment::{EquipmentEntry, SetEquipmentPacket};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error, trace};

// ============================================================================
// Phase 1: Initial Inventory Sync
// ============================================================================

/// Syncs full inventory to newly connected players.
/// Runs on players with the `NeedsInventorySync` marker component.
pub fn initial_inventory_sync(
    mut commands: Commands,
    state: Res<GlobalStateResource>,
    query: Query<
        (Entity, &Inventory, &StreamWriter),
        With<NeedsInventorySync>,
    >,
    mut sync_events: MessageWriter<InventorySynced>,
) {
    for (entity, inventory, writer) in query.iter() {
        if !state.0.players.is_connected(entity) {
            continue;
        }

        // Build slot list from inventory (46 slots for player inventory)
        let slots: Vec<InventorySlot> = inventory
            .slots
            .iter()
            .map(|slot| slot.clone().unwrap_or_default())
            .collect();

        let packet = SetContainerContent {
            window_id: VarInt::new(0), // 0 = player inventory
            state_id: VarInt::new(0),  // State tracking (0 for initial)
            slots: LengthPrefixedVec::new(slots),
            carried_item: InventorySlot::empty(), // Cursor item (empty on join)
        };

        if let Err(e) = writer.send_packet(packet) {
            error!("Failed to send initial inventory to {:?}: {:?}", entity, e);
            continue;
        }

        debug!("Sent initial inventory sync to {:?}", entity);

        // Remove the marker so we don't sync again
        commands.entity(entity).remove::<NeedsInventorySync>();

        // Fire the event for plugins
        sync_events.write(InventorySynced { player: entity });
    }
}

// ============================================================================
// Phase 2: Equipment Broadcast
// ============================================================================

/// Detects equipment changes and broadcasts them to other players.
/// Uses `Changed<Inventory>` and `Changed<Hotbar>` filters.
#[expect(clippy::type_complexity, reason = "Bevy ECS queries require complex tuples")]
pub fn equipment_broadcast(
    state: Res<GlobalStateResource>,
    mut changed_query: Query<
        (
            Entity,
            &PlayerIdentity,
            &Inventory,
            &Hotbar,
            &mut EquipmentState,
        ),
        Or<(Changed<Inventory>, Changed<Hotbar>)>,
    >,
    other_players: Query<(Entity, &StreamWriter)>,
    mut equipment_events: MessageWriter<EquipmentChanged>,
) {
    for (entity, identity, inventory, hotbar, mut cached_state) in changed_query.iter_mut() {
        if !state.0.players.is_connected(entity) {
            continue;
        }

        // Compute current equipment state
        let current_state = EquipmentState::from_inventory(inventory, hotbar);

        // Find which slots changed
        let changed_slots = cached_state.diff(&current_state);

        if changed_slots.is_empty() {
            // Update cache even if diff is empty (handles component differences)
            *cached_state = current_state;
            continue;
        }

        trace!(
            "Equipment changed for {}: {:?}",
            identity.username,
            changed_slots
        );

        // Build equipment entries for changed slots
        let entries: Vec<EquipmentEntry> = changed_slots
            .iter()
            .map(|&slot| EquipmentEntry {
                slot,
                item: current_state.get(slot).cloned().unwrap_or_default(),
            })
            .collect();

        let packet = SetEquipmentPacket::new(identity.short_uuid, entries);

        // Broadcast to all other connected players
        for (other_entity, writer) in other_players.iter() {
            if other_entity == entity {
                continue; // Don't send to self
            }
            if !state.0.players.is_connected(other_entity) {
                continue;
            }

            if let Err(e) = writer.send_packet_ref(&packet) {
                error!(
                    "Failed to send equipment update to {:?}: {:?}",
                    other_entity, e
                );
            }
        }

        // Fire the event for plugins
        equipment_events.write(EquipmentChanged {
            player: entity,
            slots: changed_slots,
        });

        // Update cached state
        *cached_state = current_state;
    }
}

// ============================================================================
// Phase 3: Join Equipment Exchange
// ============================================================================

/// When a new player joins, send their equipment to everyone else,
/// and send everyone else's equipment to them.
///
/// Uses `Added<PlayerIdentity>` instead of `PlayerJoined` message to ensure
/// the entity is queryable (commands have been applied).
pub fn join_equipment_exchange(
    state: Res<GlobalStateResource>,
    // Query new players using Added<> filter - ensures entity exists and is queryable
    new_players: Query<
        (Entity, &PlayerIdentity, &Inventory, &Hotbar, &StreamWriter),
        Added<PlayerIdentity>,
    >,
    // Query all players for exchange
    all_players: Query<(
        Entity,
        &PlayerIdentity,
        &Inventory,
        &Hotbar,
        &StreamWriter,
    )>,
) {
    for (joining_entity, joining_identity, joining_inv, joining_hotbar, joining_writer) in
        new_players.iter()
    {
        if !state.0.players.is_connected(joining_entity) {
            continue;
        }

        trace!(
            "Processing equipment exchange for joining player: {}",
            joining_identity.username
        );

        // Build joining player's equipment
        let joining_equipment = EquipmentState::from_inventory(joining_inv, joining_hotbar);

        // Only send if they have equipment
        let joining_packet = if !joining_equipment.is_empty() {
            let entries: Vec<EquipmentEntry> = joining_equipment
                .non_empty_slots()
                .map(|(slot, item)| EquipmentEntry {
                    slot,
                    item: item.clone(),
                })
                .collect();
            Some(SetEquipmentPacket::new(joining_identity.short_uuid, entries))
        } else {
            None
        };

        // Exchange with all other players
        for (other_entity, other_identity, other_inv, other_hotbar, other_writer) in
            all_players.iter()
        {
            if other_entity == joining_entity {
                continue;
            }
            if !state.0.players.is_connected(other_entity) {
                continue;
            }

            // Send joining player's equipment to this other player
            if let Some(ref packet) = joining_packet {
                if let Err(e) = other_writer.send_packet_ref(packet) {
                    error!(
                        "Failed to send joining player equipment to {:?}: {:?}",
                        other_entity, e
                    );
                }
            }

            // Send this other player's equipment to the joining player
            let other_equipment = EquipmentState::from_inventory(other_inv, other_hotbar);

            if !other_equipment.is_empty() {
                let other_entries: Vec<EquipmentEntry> = other_equipment
                    .non_empty_slots()
                    .map(|(slot, item)| EquipmentEntry {
                        slot,
                        item: item.clone(),
                    })
                    .collect();
                let other_packet =
                    SetEquipmentPacket::new(other_identity.short_uuid, other_entries);
                if let Err(e) = joining_writer.send_packet(other_packet) {
                    error!(
                        "Failed to send other player equipment to joining player: {:?}",
                        e
                    );
                }
            }
        }

        debug!(
            "Completed equipment exchange for {}",
            joining_identity.username
        );
    }
}

