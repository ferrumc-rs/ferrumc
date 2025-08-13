use bevy_ecs::prelude::Component;
use ferrumc_net::packets::incoming::set_creative_mode_slot::InventorySlot;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;

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
                if let PrefixedOptional::Some(item) = &slot.item_id {
                    item.0 == item_id
                } else {
                    false
                }
            } else {
                false
            }
        })
    }
}
