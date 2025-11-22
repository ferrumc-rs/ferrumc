use bevy_ecs::prelude::{Entity, Message};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;

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
}
