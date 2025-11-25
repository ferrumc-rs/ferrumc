use bevy_ecs::system::{Query, Res};
use ferrumc_inventories::defined_slots;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_inventories::slot::InventorySlot;
use ferrumc_net::CloseContainerReceiver;
use tracing::debug;

pub fn handle(receiver: Res<CloseContainerReceiver>, mut inventories: Query<&mut Inventory>) {
    for (event, eid) in receiver.0.try_iter() {
        // 0 is the player's inventory
        if event.window_id.0 == 0 {
            debug!("Clearing crafting grid");

            if let Ok(mut inventory) = inventories.get_mut(eid) {
                for slot_id in
                    defined_slots::player::CRAFT_SLOT_OUTPUT..=defined_slots::player::CRAFT_SLOT_4
                {
                    inventory
                        .set_item_with_update(slot_id as _, InventorySlot::empty(), eid)
                        .expect("failed to write to inventory");
                }
            }
        }
    }
}
