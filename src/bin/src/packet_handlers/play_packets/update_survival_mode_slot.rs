use crate::packet_handlers::player::update_crafting::update_player_crafting_grid;
use bevy_ecs::prelude::{Query, Res};
use ferrumc_inventories::defined_slots;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_inventories::item::ItemID;
use ferrumc_inventories::slot::InventorySlot;
use tracing::error;

pub fn handle(
    receiver: Res<ferrumc_net::ClickContainerReceiver>,
    mut inventories: Query<&mut Inventory>,
) {
    for (event, eid) in receiver.0.try_iter() {
        // TODO: actually verify that the inventory is synced, this code assumes that the ClickContainer packet is 100% truthful

        if let Ok(mut inventory) = inventories.get_mut(eid) {
            for slot in event.changed_slots.data {
                if let Some(new_data) = slot.data.to_option() {
                    inventory
                        .set_item(
                            slot.number as _,
                            InventorySlot {
                                count: new_data.item_count,
                                item_id: Some(ItemID(new_data.item_id)),
                                components_to_add: None,
                                components_to_remove: None,
                                components_to_add_count: None,
                                components_to_remove_count: None,
                            },
                        )
                        .expect("failed to write to inventory");
                } else {
                    inventory
                        .clear_slot_with_update(slot.number as _, eid)
                        .expect("failed to clear item in inventory");
                }

                if (defined_slots::player::CRAFT_SLOT_1..=defined_slots::player::CRAFT_SLOT_4)
                    .contains(&(slot.number as u8))
                {
                    update_player_crafting_grid(&mut inventory, eid);
                }
            }
        } else {
            error!("Failed to get inventory for entity {eid}");
        }
    }
}
