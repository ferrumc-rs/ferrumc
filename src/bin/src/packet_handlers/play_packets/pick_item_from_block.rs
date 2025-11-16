use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::player::abilities::PlayerAbilities;
use ferrumc_inventories::item::ItemID;
use ferrumc_inventories::slot::InventorySlot;
use ferrumc_inventories::{hotbar::Hotbar, inventory::Inventory};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_held_slot::SetHeldItem;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;

use ferrumc_net::PickItemFromBlockReceiver;
use tracing::{debug, error, warn};

/// 1. Get Player's abilities, inventory, and hotbar
/// 2. Get the block at `packet.location` from the world
/// 3. Convert the `BlockStateId` to and `ItemId`
/// 4. Search the inventory for this `ItemId`
/// 5. If found:
///      - Swap the item with the player's currently held item
///      - Send `SetContainerSlot` packets to sync the client
/// 6. If not found AND the player is in creative:
///      - Create a new item stack (with NBT if `packet.include_data`) TODO
///      - Set the player's current hotbar slot to this item
/// 7. If not found AND player is in survival
///      - Do nothing.
pub fn handle(
    events: Res<PickItemFromBlockReceiver>, // Packet queue
    state: Res<GlobalStateResource>,
    mut player_inv_query: Query<(
        Entity,
        &PlayerIdentity,
        &PlayerAbilities,
        &mut Inventory,
        &mut Hotbar,
        &StreamWriter,
    )>,
) {
    for (packet, sender_entity) in events.0.try_iter() {
        // 1. Get player's components
        let (entity, identity, abilities, mut inventory, mut hotbar, writer) =
            match player_inv_query.get_mut(sender_entity) {
                Ok(data) => data,
                Err(e) => {
                    warn!(
                    "PickItemFromBlock: Recieved packet from entity {:?} without components {:?}",
                    sender_entity, e
                );
                    continue;
                }
            };

        debug!(
            "Player {} requested pick block at {:?} (Include Data: {})",
            identity.username, packet.location, packet.include_data,
        );

        // 2. Get block from world
        let block_state_id = match state.0.world.get_block_and_fetch(
            packet.location.x,
            packet.location.y as i32,
            packet.location.z,
            "overworld", // TODO: Remove overworld hard coding for the dimension
        ) {
            Ok(id) => id,
            Err(e) => {
                warn!(
                    "PickItemFromBlock: Failed to get block at {:?}: {:?}",
                    packet.location, e
                );
                continue;
            }
        };

        // 3. Convert `BlockStateId` to `ItemId`
        let item_id = match ItemID::from_block_state(block_state_id) {
            Some(id) => id,
            None => {
                debug!(
                    "PickItemFromBlock: No item for block state {:?}",
                    block_state_id
                );
                continue; // No item for this block (e.g., air)
            }
        };

        debug!(
            "PickItemFromBlock: Block corresponds to ItemID: {:?}",
            item_id
        );

        // 4. Search the inventory for `ItemID`
        let found_slot_index = inventory.find_item(item_id);

        // 5a. Search hotbar
        if let Some(hotbar_slot) = hotbar.find_item(&inventory, item_id) {
            // Item is in the hotbar. Check if we're already holding it.
            if hotbar.selected_slot == hotbar_slot {
                continue; // Do nothing
            }

            debug!(
                "Item found in hotbar slot {}. Switching held item.",
                hotbar_slot
            );

            // 1. Update the server's state
            hotbar.selected_slot = hotbar_slot;

            // 2. Send the packet to sync the client
            let packet = SetHeldItem { slot: hotbar_slot };
            if let Err(e) = writer.send_packet_ref(&packet) {
                error!("Failed to send SetHeldItem packet: {:?}", e);
            }
        }
        // 5b. Search rest of inventory
        else if let Some(inventory_slot_index) = found_slot_index {
            debug!(
                "Found item in slot {}. Swapping with hotbar slot {}.",
                inventory_slot_index, hotbar.selected_slot
            );

            // Check if the item is already in the selected hotbar slot.
            if inventory_slot_index == hotbar.get_selected_inventory_index() {
                continue; // Nothing to do
            }

            if let Err(e) =
                hotbar.swap_with_inventory_slot(&mut inventory, inventory_slot_index, entity)
            {
                warn!("Failed to swap slots: {:?}", e);
            }
        }
        // 6. If not found AND in creative mode
        else if abilities.creative_mode {
            debug!("Item not found. Creating stack for creative player.");

            let new_slot = InventorySlot {
                item_id: Some(item_id),
                count: VarInt::new(1),
                ..Default::default()
            };

            // TODO: Handle NBT data
            if packet.include_data {
                warn!("PickBlock: NBT data request (include_data=true is not implemented yet.");
            }

            if let Err(e) = hotbar.set_selected_item(&mut inventory, new_slot, entity) {
                warn!("Failed to set creative item in hotbar: {:?}", e);
            }
        }
        // 7. If not found AND survival...
        else {
            debug!("Item not found in inventory and player is in survival. Doing nothing.")
            // No-op
        }
    }
}
