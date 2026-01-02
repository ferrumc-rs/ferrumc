use bevy_ecs::message::MessageReader;
use bevy_ecs::query::With;
use bevy_ecs::system::Query;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_inventories::item::ItemID;
use ferrumc_inventories::slot::InventorySlot;
use ferrumc_messages::GiveItemToPlayer;
use ferrumc_net_codec::net_types::var_int::VarInt;

pub fn give_item_to_player_handler(
    mut events: MessageReader<GiveItemToPlayer>,
    mut player_inventories: Query<&mut Inventory, With<PlayerIdentity>>,
) {
    for event in events.read() {
        let Ok(mut inventory) = player_inventories.get_mut(event.player) else {
            continue;
        };
        let mut quantity = event.quantity;

        let ordered_slot_indexes = vec![36..45, 9..36]
            .into_iter() // hotbar before main inventory
            .flatten()
            .collect::<Vec<usize>>();

        // fill *existing* stacks of items
        for i in ordered_slot_indexes.clone() {
            let slot = inventory.slots.get_mut(i).unwrap().clone();
            let Some(mut slot) = slot else { continue };
            let Some(item_id) = slot.item_id else {
                continue;
            };
            if item_id.as_u32() as u16 != event.item_id {
                continue;
            }
            let slot_quantity_to_add = (64 - slot.count.0).min(quantity as i32);
            slot.count.0 += slot_quantity_to_add;
            quantity -= slot_quantity_to_add as u32;
            let _ = inventory.set_item_with_update(i, slot.clone(), event.player);
        }

        // add *new* stacks of items
        for i in ordered_slot_indexes {
            if inventory.get_item(i).unwrap().is_some() {
                continue;
            }
            let slot_quantity_to_add = quantity.min(64);
            quantity -= slot_quantity_to_add;
            let slot = InventorySlot {
                item_id: Some(ItemID::new(event.item_id as i32)),
                count: VarInt(slot_quantity_to_add as i32),
                components_to_add: None,
                components_to_add_count: None,
                components_to_remove: None,
                components_to_remove_count: None,
            };
            inventory
                .set_item_with_update(i, slot.clone(), event.player)
                .unwrap();
        }
    }
}
