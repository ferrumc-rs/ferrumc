use std::ops::{Deref, DerefMut};

use ferrumc_net::packets::outgoing::set_equipment::EquipmentSlot;

use crate::{
    inventory::{Inventory, InventoryError, InventoryType},
    slot::Slot,
};

#[derive(Default, Debug, Clone)]
pub struct EquipmentContent {
    pub helmet: Slot,
    pub chestplate: Slot,
    pub leggings: Slot,
    pub boots: Slot,
}

#[derive(Debug, Clone)]
pub struct PlayerInventory {
    inventory: Inventory,
    pub equipment_content: EquipmentContent,
    pub main_hand: Slot,
    pub off_hand: Slot,
}

impl Deref for PlayerInventory {
    type Target = Inventory;

    fn deref(&self) -> &Self::Target {
        &self.inventory
    }
}

impl DerefMut for PlayerInventory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inventory
    }
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self::new(
            EquipmentContent::default(),
            Slot::default(),
            Slot::default(),
        )
    }
}

impl PlayerInventory {
    pub fn new(equipment_content: EquipmentContent, main_hand: Slot, off_hand: Slot) -> Self {
        Self {
            inventory: Inventory::new(0, "", InventoryType::Chest(5)),
            equipment_content,
            main_hand,
            off_hand,
        }
    }

    pub fn set_equipment_slot(
        &mut self,
        equipment_slot: EquipmentSlot,
        slot: Slot,
    ) -> Result<(), InventoryError> {
        let slot_id = match equipment_slot {
            EquipmentSlot::Boots => {
                self.equipment_content.boots = slot;
                8
            }
            EquipmentSlot::Leggings => {
                self.equipment_content.leggings = slot;
                7
            }
            EquipmentSlot::Chestplate => {
                self.equipment_content.chestplate = slot;
                6
            }
            EquipmentSlot::Helmet => {
                self.equipment_content.helmet = slot;
                5
            }
            EquipmentSlot::OffHand => 45,
            _ => return Err(InventoryError::InvalidEquipmentSlot),
        };

        self.inventory.set_slot(slot_id, slot);
        Ok(())
    }
}
