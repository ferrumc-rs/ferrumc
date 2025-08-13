use bevy_ecs::prelude::{Query, Res};
use ferrumc_inventories::display::DisplayType;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_net::SetCreativeModeSlotReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error};

pub fn handle(
    events: Res<SetCreativeModeSlotReceiver>,
    state: Res<GlobalStateResource>,
    mut query: Query<&mut Inventory>,
) {
    for (event, entity) in events.0.try_iter() {
        debug!("Slot {} placed at {} by player {}",
            event.slot, event.slot_index, entity);
        if state.0.players.is_connected(entity) {
            if let Ok(mut inventory) = query.get_mut(entity) {
                if event.slot.count.0 == 0 {
                    // Clear the slot if the count is zero
                    if let Err(e) = inventory.remove_item(event.slot_index as usize) {
                        error!("Failed to clear slot {} for player {}: {:?}", event.slot_index, entity, e);
                    }
                } else {
                    // Set the item in the specified slot
                    if let Err(e) = inventory.set_item(event.slot_index as usize, event.slot) {
                        error!("Failed to set item in slot {} for player {}: {:?}", event.slot_index, entity, e);
                    }
                }
                // Display the updated inventory
                inventory.display(DisplayType::Player);
            } else {
                error!("Could not find inventory for player {}", entity);
            }
        }
    }
}