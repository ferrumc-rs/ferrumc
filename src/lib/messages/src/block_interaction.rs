use bevy_ecs::prelude::{Entity, Message};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// World coordinates for a block, stored as (x, y, z).
///
/// This is a simple coordinate type that avoids Debug issues with BlockPos.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockCoords {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Message sent when a player right-clicks an interactive block (door, lever, etc.)
/// and is NOT sneaking.
///
/// Emitted by the PlaceBlock packet handler; consumed by the interaction listener.
#[derive(Message, Clone, Debug)]
pub struct BlockInteractMessage {
    pub player: Entity,
    pub position: BlockCoords,
    pub sequence: VarInt,
}
