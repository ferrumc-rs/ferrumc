use ferrumc_text::{TextComponent, TextComponentBuilder};

use crate::{
    contents::InventoryContents,
    inventory::{Inventory, InventoryType},
    slot::Slot,
};

pub struct InventoryBuilder {
    pub id: u8,
    pub inventory_type: InventoryType,
    pub title: TextComponent,
    pub(crate) contents: InventoryContents,
    pub carried_item: Slot,
    pub is_synced: bool,
}

impl InventoryBuilder {
    pub fn new(id: u8) -> Self {
        let inventory_type = InventoryType::Chest(3);
        Self {
            id,
            inventory_type,
            title: TextComponentBuilder::new("").build(),
            contents: InventoryContents::empty(inventory_type),
            carried_item: Slot::empty(),
            is_synced: false,
        }
    }

    pub fn inventory_type(&mut self, inventory_type: InventoryType) -> &mut Self {
        self.inventory_type = inventory_type;
        self
    }

    pub fn title(&mut self, title: TextComponent) -> &mut Self {
        self.title = title;
        self
    }

    pub fn contents(&mut self, contents: InventoryContents) -> &mut Self {
        self.contents = contents;
        self
    }

    pub fn carried_item(&mut self, carried_item: Slot) -> &mut Self {
        self.carried_item = carried_item;
        self
    }

    pub fn is_synced(&mut self, is_synced: bool) -> &mut Self {
        self.is_synced = is_synced;
        self
    }

    pub fn build(&mut self) -> Inventory {
        Inventory {
            id: self.id,
            inventory_type: self.inventory_type,
            title: self.title.clone(),
            contents: self.contents.clone(),
            carried_item: self.carried_item,
            is_synced: self.is_synced,
        }
    }
}
