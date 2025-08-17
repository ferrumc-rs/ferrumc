use crate::errors::InventoryError;
use crate::slot::InventorySlot;
use crate::{INVENTORY_UPDATES_QUEUE, InventoryUpdate};
use bevy_ecs::prelude::{Component, Entity};

#[derive(Component)]
pub struct Inventory {
    pub slots: Box<[Option<InventorySlot>]>,
}

impl Inventory {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![None; size].into_boxed_slice(),
        }
    }

    pub fn clear(&mut self) {
        for slot in &mut self.slots {
            *slot = None;
        }
    }

    pub fn contains_item(&self, item_id: i32) -> bool {
        self.slots.iter().any(|slot| {
            if let Some(slot) = slot {
                if let Some(item) = &slot.item_id {
                    item.0 == item_id
                } else {
                    false
                }
            } else {
                false
            }
        })
    }

    pub fn add_item(&mut self, item: InventorySlot) -> Result<(), InventoryError> {
        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                *slot = Some(item);
                return Ok(());
            }
        }
        Err(InventoryError::InventoryFull)
    }

    pub fn add_item_with_update(
        &mut self,
        item: InventorySlot,
        entity: Entity,
    ) -> Result<(), InventoryError> {
        for (index, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(item.clone());
                INVENTORY_UPDATES_QUEUE.push(InventoryUpdate {
                    slot_index: index as u8,
                    slot: item,
                    entity,
                });
                return Ok(());
            }
        }
        Err(InventoryError::InventoryFull)
    }

    pub fn set_item(&mut self, index: usize, item: InventorySlot) -> Result<(), InventoryError> {
        if index >= self.slots.len() {
            return Err(InventoryError::InvalidSlotIndex(index));
        }
        self.slots[index] = Some(item);
        Ok(())
    }

    pub fn set_item_with_update(
        &mut self,
        index: usize,
        item: InventorySlot,
        entity: Entity,
    ) -> Result<(), InventoryError> {
        if index >= self.slots.len() {
            return Err(InventoryError::InvalidSlotIndex(index));
        }
        self.slots[index] = Some(item.clone());
        INVENTORY_UPDATES_QUEUE.push(InventoryUpdate {
            slot_index: index as u8,
            slot: item,
            entity,
        });
        Ok(())
    }

    pub fn get_item(&self, index: usize) -> Result<Option<&InventorySlot>, InventoryError> {
        if index >= self.slots.len() {
            return Err(InventoryError::InvalidSlotIndex(index));
        }
        Ok(self.slots[index].as_ref())
    }

    pub fn remove_item(&mut self, index: usize) -> Result<(), InventoryError> {
        if index >= self.slots.len() {
            return Err(InventoryError::InvalidSlotIndex(index));
        }
        if self.slots[index].is_none() {
            return Err(InventoryError::ItemNotFound);
        }
        self.slots[index] = None;
        Ok(())
    }

    pub fn remove_item_with_update(
        &mut self,
        index: usize,
        entity: Entity,
    ) -> Result<(), InventoryError> {
        if index >= self.slots.len() {
            return Err(InventoryError::InvalidSlotIndex(index));
        }
        if self.slots[index].is_none() {
            return Err(InventoryError::ItemNotFound);
        }
        self.slots[index] = None;
        INVENTORY_UPDATES_QUEUE.push(InventoryUpdate {
            slot_index: index as u8,
            slot: InventorySlot::default(),
            entity,
        });
        Ok(())
    }
}
