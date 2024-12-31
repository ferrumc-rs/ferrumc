#[test]
fn create_inventory() {
    let mut inventory =
        crate::inventory::Inventory::new(1, "Something", crate::inventory::InventoryType::Chest(4));
    inventory.set_slot(0, crate::slot::Slot::with_item(9));

    inventory.clear();
}
