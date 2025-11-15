use bevy_ecs::prelude::Component;

use crate::errors::InventoryError;
use crate::inventory::Inventory;
use crate::item::ItemID;
use crate::slot::InventorySlot;
use bevy_ecs::prelude::Entity;

#[derive(Component, Default)]
pub struct Hotbar {
    /// The currently selected hotbar slot (0-8).
    pub selected_slot: u8,
}

impl Hotbar {
    const HOTBAR_START_INDEX: usize = 36;
    const HOTBAR_SIZE: usize = 9;

    /// Returns the actual inventory slot index (36-44) for a given hotbar slot (0-8).
    /// This performs the (36 + slot) translation.
    pub fn get_inventory_index(&self, hotbar_slot: u8) -> Result<usize, InventoryError> {
        let slot = hotbar_slot as usize;
        if slot >= Self::HOTBAR_SIZE {
            // Return an error if the hotbar slot is > 8
            return Err(InventoryError::InvalidSlotIndex(slot));
        }
        Ok(Self::HOTBAR_START_INDEX + slot)
    }

    /// Returns the actual inventory slot index (36-44) for the *selected* hotbar slot.
    pub fn get_selected_inventory_index(&self) -> usize {
        // This should be safe, assuming selected_slot is always 0-8
        Self::HOTBAR_START_INDEX + self.selected_slot as usize
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
        // This function is now just a convenient shortcut
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
        for i in 0..Self::HOTBAR_SIZE {
            // We can use our own helper to get the real index
            if let Ok(index) = self.get_inventory_index(i as u8)
                && let Some(Some(slot)) = inventory.slots.get(index)
                && slot.item_id == Some(item_id)
            {
                return Some(i as u8);
            }
        }
        None
    }
}
