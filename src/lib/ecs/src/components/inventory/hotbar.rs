use bevy_ecs::prelude::{Component, Entity};

// Adjust these imports based on where you landed with the errors file.
// Assuming FerrumC-Inventories still exists for errors, otherwise update path.
use crate::inventory::errors::InventoryError;

use crate::inventory::storage::Inventory;
use ferrumc_core::items::inventory_slot::InventorySlot;
use ferrumc_core::items::item_id::ItemID;
use ferrumc_core::player::inventory_slots_consts::player_inventory_slots; // Use the core module

#[derive(Component, Default, Debug, Clone)]
pub struct Hotbar {
    /// The currently selected hotbar slot (0-8).
    pub selected_slot: u8,
}

impl Hotbar {
    /// The size of the hotbar (0-8 = 9 slots)
    const SIZE: u8 = 9;

    /// Returns the actual inventory slot index (36-44) for a given hotbar slot (0-8).
    pub fn get_inventory_index(&self, hotbar_slot: u8) -> Result<usize, InventoryError> {
        if hotbar_slot >= Self::SIZE {
            return Err(InventoryError::InvalidSlotIndex(hotbar_slot as usize));
        }
        // Use the Core constant for the offset (36)
        Ok(player_inventory_slots::HOTBAR_START + hotbar_slot as usize)
    }

    /// Returns the actual inventory slot index (36-44) for the *selected* hotbar slot.
    pub fn get_selected_inventory_index(&self) -> usize {
        // This is safe because we assume selected_slot is kept within bounds (0-8)
        player_inventory_slots::HOTBAR_START + self.selected_slot as usize
    }

    /// Gets an item from a specific hotbar slot (0-8).
    pub fn get_item<'a>(
        &self,
        inventory: &'a Inventory,
        hotbar_slot: u8,
    ) -> Result<Option<&'a InventorySlot>, InventoryError> {
        let index = self.get_inventory_index(hotbar_slot)?;
        inventory.get_item(index)
    }

    /// Gets the item from the *currently selected* hotbar slot.
    pub fn get_selected_item<'a>(
        &self,
        inventory: &'a Inventory,
    ) -> Result<Option<&'a InventorySlot>, InventoryError> {
        let index = self.get_selected_inventory_index();
        inventory.get_item(index)
    }

    /// Sets an item in a specific hotbar slot (0-8).
    pub fn set_item_with_update(
        &self,
        inventory: &mut Inventory,
        hotbar_slot: u8,
        slot: InventorySlot,
        entity: Entity,
    ) -> Result<(), InventoryError> {
        let index = self.get_inventory_index(hotbar_slot)?;
        inventory.set_item_with_update(index, slot, entity)
    }

    /// Wraps `set_item_with_update` to set the *currently selected* hotbar slot.
    pub fn set_selected_item(
        &self,
        inventory: &mut Inventory,
        slot: InventorySlot,
        entity: Entity,
    ) -> Result<(), InventoryError> {
        self.set_item_with_update(inventory, self.selected_slot, slot, entity)
    }

    /// Removes an item from a specific hotbar slot (0-8).
    pub fn remove_item_with_update(
        &self,
        inventory: &mut Inventory,
        hotbar_slot: u8,
        entity: Entity,
    ) -> Result<(), InventoryError> {
        let index = self.get_inventory_index(hotbar_slot)?;
        inventory.remove_item_with_update(index, entity)
    }

    /// Wraps `Inventory::swap_slots_with_update` to swap an inventory slot
    /// with the currently selected hotbar slot.
    pub fn swap_with_inventory_slot(
        &self,
        inventory: &mut Inventory,
        inventory_slot_index: usize,
        entity: Entity,
    ) -> Result<(), InventoryError> {
        let hotbar_index = self.get_selected_inventory_index();
        inventory.swap_slots_with_update(inventory_slot_index, hotbar_index, entity)
    }

    /// Finds an item within the 9 hotbar slots.
    /// Returns the hotbar index (0-8) if found.
    pub fn find_item(&self, inventory: &Inventory, item_id: ItemID) -> Option<u8> {
        for i in 0..Self::SIZE {
            // We use the get_inventory_index helper to ensure we map correctly
            if let Ok(index) = self.get_inventory_index(i) {
                // Direct slot access or get_item is fine here
                if let Ok(Some(slot)) = inventory.get_item(index) {
                    if let Some(current_id) = slot.item_id {
                        if current_id == item_id {
                            return Some(i);
                        }
                    }
                }
            }
        }
        None
    }

    /// Finds the lowest-index open (empty) hotbar slot.
    ///
    /// Returns the hotbar index (0-8) if an empty slot is found.
    pub fn get_lowest_open_slot(&self, inventory: &Inventory) -> Option<u8> {
        for i in 0..Self::SIZE {
            // Get main inventory index
            let inventory_index = match self.get_inventory_index(i) {
                Ok(idx) => idx,
                Err(_) => continue,
            };

            // Check if empty
            match inventory.get_item(inventory_index) {
                Ok(None) => return Some(i), // Found empty slot
                Ok(Some(_)) => continue,    // Slot occupied
                Err(_) => continue,
            }
        }

        None
    }
}
