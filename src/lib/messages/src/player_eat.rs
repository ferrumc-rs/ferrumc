use bevy_ecs::prelude::{Entity, Message};
use ferrumc_inventories::item::ItemID;

/// Fired when a player successfully eats an item.
///
/// Fired by: `use_item` packet handler.
/// Listened for by: A `hunger_system` to update the `Hunger` component.
#[derive(Message)]
#[allow(unused)]
pub struct PlayerEating {
    pub player: Entity,
    pub item_id: ItemID, // The food item they ate
    // we can pre-calculate these from the registry
    pub food_value: u8,
    pub saturation_value: f32,
}
