use bevy_ecs::prelude::{Query, Res};
use ferrumc_inventories::defined_slots::player::HOTBAR_SLOT_6;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_inventories::item::ItemID;
use ferrumc_inventories::slot::InventorySlot;
use ferrumc_net::SetCreativeModeSlotReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error};

pub fn handle(
    receiver: Res<SetCreativeModeSlotReceiver>,
    state: Res<GlobalStateResource>,
    mut query: Query<&mut Inventory>,
) {
    for (event, entity) in receiver.0.try_iter() {
        debug!(
            "Slot {} placed at {} by player {}",
            event.slot, event.slot_index, entity
        );
        if state.0.players.is_connected(entity) {
            if let Ok(mut inventory) = query.get_mut(entity) {
                if event.slot.count.0 == 0 {
                    // Clear the slot if the count is zero
                    if let Err(e) = inventory.remove_item(event.slot_index as usize) {
                        error!(
                            "Failed to clear slot {} for player {}: {:?}",
                            event.slot_index, entity, e
                        );
                    }
                } else {
                    // Set the item in the specified slot
                    if let Err(e) = inventory.set_item(event.slot_index as usize, event.slot) {
                        error!(
                            "Failed to set item in slot {} for player {}: {:?}",
                            event.slot_index, entity, e
                        );
                    }
                }
                // Display the updated inventory
                if let Err(err) = inventory.set_item_with_update(
                    HOTBAR_SLOT_6 as usize,
                    InventorySlot {
                        count: 1.into(),
                        item_id: Some(ItemID::new(872)), // Example item ID for the creative mode slot
                        components_to_add_count: None,
                        components_to_remove_count: None,
                        components_to_add: None,
                        components_to_remove: None,
                    },
                    entity,
                ) {
                    error!(
                        "Failed to update creative mode slot for player {}: {:?}",
                        entity, err
                    );
                }
            } else {
                error!("Could not find inventory for player {}", entity);
            }
        }
    }
}
