//! Inventory synchronization components for ECS-based sync detection.
//!
//! This module provides marker components and state tracking for:
//! - Initial inventory sync on player join
//! - Equipment change detection for broadcasting to other players

use bevy_ecs::prelude::Component;

use crate::defined_slots::player;
use crate::hotbar::Hotbar;
use crate::inventory::Inventory;
use crate::slot::InventorySlot;

/// Marker component: player needs initial inventory sync.
/// Inserted when a player spawns, removed after `SetContainerContent` is sent.
#[derive(Component, Default)]
pub struct NeedsInventorySync;

/// Equipment slot identifiers for the `SetEquipment` packet.
/// Protocol values: https://minecraft.wiki/w/Protocol#Set_Equipment
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EquipmentSlot {
    MainHand = 0,
    OffHand = 1,
    Feet = 2,
    Legs = 3,
    Chest = 4,
    Head = 5,
}

impl EquipmentSlot {
    /// Returns all equipment slots in protocol order.
    pub const ALL: [EquipmentSlot; 6] = [
        EquipmentSlot::MainHand,
        EquipmentSlot::OffHand,
        EquipmentSlot::Feet,
        EquipmentSlot::Legs,
        EquipmentSlot::Chest,
        EquipmentSlot::Head,
    ];

    /// Protocol ID for this slot.
    pub fn protocol_id(self) -> u8 {
        self as u8
    }
}

/// Cached equipment state for change detection.
/// Stores the last-known equipment for each slot, allowing efficient diffing.
#[derive(Component, Clone, Default, Debug)]
pub struct EquipmentState {
    pub head: Option<InventorySlot>,
    pub chest: Option<InventorySlot>,
    pub legs: Option<InventorySlot>,
    pub feet: Option<InventorySlot>,
    pub main_hand: Option<InventorySlot>,
    pub off_hand: Option<InventorySlot>,
}

impl EquipmentState {
    /// Builds an EquipmentState from the player's inventory and hotbar selection.
    pub fn from_inventory(inventory: &Inventory, hotbar: &Hotbar) -> Self {
        let get_slot = |idx: u8| -> Option<InventorySlot> {
            inventory.get_item(idx as usize).ok().flatten().cloned()
        };

        let main_hand_idx = hotbar.get_selected_inventory_index() as u8;

        Self {
            head: get_slot(player::HEAD_SLOT),
            chest: get_slot(player::CHEST_SLOT),
            legs: get_slot(player::LEGS_SLOT),
            feet: get_slot(player::FEET_SLOT),
            main_hand: get_slot(main_hand_idx),
            off_hand: get_slot(player::OFFHAND_SLOT),
        }
    }

    /// Returns the item for a given equipment slot.
    pub fn get(&self, slot: EquipmentSlot) -> Option<&InventorySlot> {
        match slot {
            EquipmentSlot::MainHand => self.main_hand.as_ref(),
            EquipmentSlot::OffHand => self.off_hand.as_ref(),
            EquipmentSlot::Feet => self.feet.as_ref(),
            EquipmentSlot::Legs => self.legs.as_ref(),
            EquipmentSlot::Chest => self.chest.as_ref(),
            EquipmentSlot::Head => self.head.as_ref(),
        }
    }

    /// Sets the item for a given equipment slot.
    pub fn set(&mut self, slot: EquipmentSlot, item: Option<InventorySlot>) {
        match slot {
            EquipmentSlot::MainHand => self.main_hand = item,
            EquipmentSlot::OffHand => self.off_hand = item,
            EquipmentSlot::Feet => self.feet = item,
            EquipmentSlot::Legs => self.legs = item,
            EquipmentSlot::Chest => self.chest = item,
            EquipmentSlot::Head => self.head = item,
        }
    }

    /// Compares two EquipmentStates and returns which slots differ.
    /// Uses item_id comparison for efficiency (ignores component differences).
    pub fn diff(&self, other: &EquipmentState) -> Vec<EquipmentSlot> {
        let mut changed = Vec::new();
        for slot in EquipmentSlot::ALL {
            let old_id = self.get(slot).and_then(|s| s.item_id);
            let new_id = other.get(slot).and_then(|s| s.item_id);
            if old_id != new_id {
                changed.push(slot);
            }
        }
        changed
    }

    /// Returns an iterator over all non-empty (slot, item) pairs.
    /// Useful for building equipment packets.
    pub fn non_empty_slots(&self) -> impl Iterator<Item = (EquipmentSlot, &InventorySlot)> {
        EquipmentSlot::ALL
            .into_iter()
            .filter_map(|slot| self.get(slot).map(|item| (slot, item)))
    }

    /// Returns true if all equipment slots are empty.
    pub fn is_empty(&self) -> bool {
        EquipmentSlot::ALL
            .iter()
            .all(|&slot| self.get(slot).is_none())
    }
}
