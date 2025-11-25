use bevy_ecs::prelude::Entity;
use ferrumc_core::crafting::get_recipes_from_2x2;
use ferrumc_data::items::Item;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_inventories::item::ItemID;
use ferrumc_inventories::slot::InventorySlot;
use ferrumc_inventories::defined_slots;
use ferrumc_net_codec::net_types::var_int::VarInt;

pub fn update_player_crafting_grid(
    inventory: &mut Inventory,
    eid: Entity,
) {
        let craft_slot_1 = inventory.get_item(defined_slots::player::CRAFT_SLOT_1 as _)
            .expect("failed to get craft slot 1")
            .and_then(|slot| slot.item_id)
            .and_then(|id| Item::from_id(id.0.0 as _));
        let craft_slot_2 = inventory.get_item(defined_slots::player::CRAFT_SLOT_2 as _)
            .expect("failed to get craft slot 2")
            .and_then(|slot| slot.item_id)
            .and_then(|id| Item::from_id(id.0.0 as _));
        let craft_slot_3 = inventory.get_item(defined_slots::player::CRAFT_SLOT_3 as _)
            .expect("failed to get craft slot 3")
            .and_then(|slot| slot.item_id)
            .and_then(|id| Item::from_id(id.0.0 as _));
        let craft_slot_4 = inventory.get_item(defined_slots::player::CRAFT_SLOT_4 as _)
            .expect("failed to get craft slot 4")
            .and_then(|slot| slot.item_id)
            .and_then(|id| Item::from_id(id.0.0 as _));

        let recipes = get_recipes_from_2x2([[craft_slot_1, craft_slot_2], [craft_slot_3, craft_slot_4]]);

        if let Some(first) = recipes.first().and_then(|recipe| recipe.result.as_ref()) {
            let item = Item::from_registry_key(first.id).expect(format!("failed to get item: {:?}", first.id).as_str());

            let slot = InventorySlot {
                item_id: Some(ItemID(VarInt(item.id as _))),
                count: VarInt(first.count as _),
                .. Default::default()
            };

            inventory.set_item_with_update(
                defined_slots::player::CRAFT_SLOT_OUTPUT as _,
                slot,
                eid,
            ).expect("failed to set item in inventory");
        } else {
            inventory.clear_slot_with_update(
                defined_slots::player::CRAFT_SLOT_OUTPUT as _,
                eid,
            ).expect("failed to clear item in inventory");
        }
}