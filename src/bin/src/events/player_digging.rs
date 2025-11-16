use bevy_ecs::prelude::{Entity, Event};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;

/// Fired when the client sends "start digging" (status 0).
///
/// Fired by: `player_action` packet handler.
/// Listened for by: A new `digging_system`.
#[derive(Event)]
pub struct PlayerStartDiggingEvent {
    pub player: Entity,
    pub position: NetworkPosition,
}

/// Fired when the client sends "cancel digging" (status 1).
///
/// Fired by: `player_action` packet handler.
/// Listened for by: `digging_system` (to remove the `PlayerDigging` component).
#[derive(Event)]
pub struct PlayerCancelDiggingEvent {
    pub player: Entity,
}

/// Fired when the client sends "finish digging" (status 2).
///
/// Fired by: `player_action` packet handler.
/// Listened for by: `digging_system` (to check time, break the block, and spawn items).
#[derive(Event)]
pub struct PlayerFinishDiggingEvent {
    pub player: Entity,
    pub position: NetworkPosition,
}
