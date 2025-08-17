use crate::inventory::Inventory;
use crate::item::ItemID;
use ferrumc_net_codec::net_types::var_int::VarInt;

pub(crate) fn display_player(inventory: &Inventory) {
    // TODO: Crafting grid, armor slots, etc.
    inventory.slots.chunks(9).skip(1).take(4).for_each(|slots| {
        println!("*{}*", "-".repeat(70));
        for slot in slots {
            if let Some(item) = slot {
                print!(
                    "| {:04} |",
                    item.item_id.unwrap_or(ItemID(VarInt::new(-1))).0
                );
            } else {
                print!("|      |");
            }
        }
        println!();
    });
    println!("*{}*", "-".repeat(70));
}
