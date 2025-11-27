<<<<<<<< HEAD:src/lib/messages/src/player_digging.rs
use bevy_ecs::prelude::{Entity, Message};
========
use bevy_ecs::prelude::{Entity, Event};

use ferrumc_core::items::item_id::ItemID;
>>>>>>>> origin/master:src/lib/messages/src/player/interaction.rs
use ferrumc_protocol::codec::net_types::network_position::NetworkPosition;
use ferrumc_protocol::codec::net_types::var_int::VarInt;

/// Fired when the client sends "start digging" (status 0).
///
/// Fired by: `player_action` packet handler.
/// Listened for by: A new `digging_system`.
#[derive(Message)]
#[allow(unused)]
pub struct PlayerStartedDigging {
    pub player: Entity,
    pub position: NetworkPosition,
    pub sequence: VarInt,
}

/// Fired when the client sends "cancel digging" (status 1).
///
/// Fired by: `player_action` packet handler.
/// Listened for by: `digging_system` (to remove the `PlayerDigging` component).
#[derive(Message)]
#[allow(unused)]
pub struct PlayerCancelledDigging {
    pub player: Entity,
    pub sequence: VarInt,
}

/// Fired when the client sends "finish digging" (status 2).
///
/// Fired by: `player_action` packet handler.
/// Listened for by: `digging_system` (to check time, break the block, and spawn items).
#[derive(Message)]
#[allow(unused)]
pub struct PlayerFinishedDigging {
    pub player: Entity,
    pub position: NetworkPosition,
    pub sequence: VarInt,
<<<<<<<< HEAD:src/lib/messages/src/player_digging.rs
========
}

/// Fired when a player successfully eats an item.
///
/// Fired by: `use_item` packet handler.
/// Listened for by: A `hunger_system` to update the `Hunger` component.
#[derive(Event)]
#[allow(unused)]
pub struct PlayerEatEvent {
    pub player: Entity,
    pub item_id: ItemID, // The food item they ate
    // we can pre-calculate these from the registry
    pub food_value: u8,
    pub saturation_value: f32,
>>>>>>>> origin/master:src/lib/messages/src/player/interaction.rs
}
