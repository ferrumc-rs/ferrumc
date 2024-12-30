use crate::contents::InventoryContents;
use crate::slot::Slot;
use crate::viewers::InventoryView;
use dashmap::DashMap;
use ferrumc_ecs::entities::Entity;
use ferrumc_net::{connection::StreamWriter, errors::NetError};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_text::{TextComponent, TextComponentBuilder};

#[derive(Debug, Clone, Copy)]
pub enum InventoryType {
    Chest(i8),
    Anvil,
    Beacon,
    BlastFurnace,
    BrewingStand,
    CraftingTable,
    EnchantmentTable,
    Furnace,
    Grindstone,
    Hopper,
    Dispenser,
    Dropper,
    Lectern,
    Loom,
    ShulkerBox,
    SmithingTable,
    Smoker,
    Cartography,
    Stonecutter,
}

impl InventoryType {
    pub fn get_id(&self) -> VarInt {
        let id = match self {
            InventoryType::Chest(i) => {
                let value = i32::from(*i);
                if (1..=6).contains(&value) {
                    value - 1
                } else {
                    0 // defaults to 1 row chest
                }
            }
            InventoryType::Anvil => 8,
            InventoryType::Beacon => 9,
            InventoryType::BlastFurnace => 10,
            InventoryType::BrewingStand => 11,
            InventoryType::CraftingTable => 12,
            InventoryType::EnchantmentTable => 13,
            InventoryType::Furnace => 14,
            InventoryType::Grindstone => 15,
            InventoryType::Hopper => 16,
            InventoryType::Dispenser | InventoryType::Dropper => 6,
            InventoryType::Lectern => 17,
            InventoryType::Loom => 18,
            InventoryType::ShulkerBox => 20,
            InventoryType::SmithingTable => 21,
            InventoryType::Smoker => 22,
            InventoryType::Cartography => 23,
            InventoryType::Stonecutter => 24,
        };

        VarInt::new(id)
    }

    pub fn get_size(&self) -> i32 {
        match self {
            InventoryType::Chest(i) => i32::from(*i) * 9,
            InventoryType::Anvil
            | InventoryType::BlastFurnace
            | InventoryType::Furnace
            | InventoryType::Smoker
            | InventoryType::Cartography
            | InventoryType::Grindstone => 2,
            InventoryType::Stonecutter | InventoryType::EnchantmentTable => 1,
            InventoryType::Dispenser | InventoryType::Dropper => 8,
            InventoryType::Loom | InventoryType::SmithingTable => 3,
            InventoryType::Beacon => 0,
            InventoryType::Hopper => 4,
            InventoryType::ShulkerBox => 26,
            InventoryType::CraftingTable => 9,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InventoryData {
    pub id: VarInt,
    pub inventory_type: InventoryType,
    pub title: TextComponent,
    pub(crate) contents: InventoryContents,
}

impl InventoryData {
    fn new(id: VarInt, inventory_type: InventoryType, title: TextComponent) -> Self {
        Self {
            id,
            inventory_type,
            title,
            contents: InventoryContents::empty(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub data: InventoryData,
    pub view: InventoryView,
}

impl Inventory {
    pub fn new<S: Into<String>>(id: i32, title: S, inventory_type: InventoryType) -> Self {
        Self {
            data: InventoryData::new(
                VarInt::new(id),
                inventory_type,
                TextComponentBuilder::new(title).build(),
            ),
            view: InventoryView::new(),
        }
    }

    pub fn set_slot(&mut self, slot_id: i32, slot: Slot) -> &mut Self {
        let size = self.data.inventory_type.get_size();
        if size >= 0 && size <= slot_id {
            self.data.contents.set_slot(slot_id, slot);
        }

        self
    }

    pub fn get_slot(&self, slot_id: i32) -> Option<Slot> {
        let size = self.data.inventory_type.get_size();
        if size >= 0 && size <= slot_id {
            self.data.contents.get_slot(slot_id)
        } else {
            None
        }
    }

    pub async fn add_viewer(
        &mut self,
        viewer: (Entity, &mut StreamWriter),
    ) -> Result<(), NetError> {
        self.view.add_viewer(&self.data, viewer).await?;
        Ok(())
    }

    pub async fn remove_viewer(
        &mut self,
        viewer: (Entity, &mut StreamWriter),
    ) -> Result<(), NetError> {
        self.view.remove_viewer(&self.data, viewer).await?;
        Ok(())
    }

    pub fn get_viewers(&self) -> &Vec<Entity> {
        &self.view.viewers
    }

    pub fn get_contents(&self) -> &DashMap<i32, Slot> {
        &self.data.contents.contents
    }

    pub fn clear(&mut self) {
        self.get_contents().clear();
    }

    pub fn contains(&self, item: i32) -> bool {
        self.get_contents()
            .iter()
            .any(|slot| slot.value().item == item)
    }

    pub fn contains_slot(&self, slot: Slot) -> bool {
        self.contains(slot.item)
    }

    pub fn get_first_empty(&self) -> i32 {
        let contents = self.get_contents();
        for i in 0..self.get_size() {
            if contents.get(&i).is_none() {
                return i;
            }
        }

        0
    }

    pub fn get_size(&self) -> i32 {
        self.data.inventory_type.get_size()
    }

    pub fn is_empty(&self) -> bool {
        self.get_contents().is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.get_contents().len() == self.get_size() as usize
    }
}
