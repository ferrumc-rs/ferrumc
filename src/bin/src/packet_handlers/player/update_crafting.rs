use bevy_ecs::prelude::Entity;
use ferrumc_core::crafting::get_recipes_from_2x2;
use ferrumc_data::items::Item;
use ferrumc_inventories::defined_slots;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_inventories::item::ItemID;
use ferrumc_inventories::slot::InventorySlot;
use ferrumc_net_codec::net_types::var_int::VarInt;
use tracing::error;

/// Takes in a player inventory and entity id and will update the survival crafting grid output based on its inputs
pub fn update_player_crafting_grid(inventory: &mut Inventory, eid: Entity) {
    let recipes = get_recipes_from_2x2([
        [
            get_inventory_slot(inventory, defined_slots::player::CRAFT_SLOT_1),
            get_inventory_slot(inventory, defined_slots::player::CRAFT_SLOT_2),
        ],
        [
            get_inventory_slot(inventory, defined_slots::player::CRAFT_SLOT_3),
            get_inventory_slot(inventory, defined_slots::player::CRAFT_SLOT_4),
        ],
    ]);

    if let Some(first) = recipes.first().and_then(|recipe| recipe.result.as_ref()) {
        let item = Item::from_registry_key(first.id)
            .unwrap_or_else(|| panic!("Failed to get item: {:?}", first.id));

        let slot = InventorySlot {
            item_id: Some(ItemID(VarInt(item.id as _))),
            count: VarInt(first.count as _),
            ..Default::default()
        };

        inventory
            .set_item_with_update(defined_slots::player::CRAFT_SLOT_OUTPUT as _, slot, eid)
            .unwrap_or_else(|err| error!("Failed to set player crafting output slot: {}", err))
    } else {
        inventory
            .clear_slot_with_update(defined_slots::player::CRAFT_SLOT_OUTPUT as _, eid)
            .unwrap_or_else(|err| error!("Failed to clear player crafting output slot: {}", err))
    }
}

fn get_inventory_slot(inventory: &Inventory, slot: u8) -> Option<&Item> {
    inventory
        .get_item(slot as usize)
        .ok()
        .and_then(|slot| slot.and_then(|id| id.item_id))
        .and_then(|item_id| Item::from_id(item_id.0 .0 as u16))
}
