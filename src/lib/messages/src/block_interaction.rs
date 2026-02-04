//! Block interaction messages for the ECS-based interaction system.
//!
//! These messages flow through the message bus when players interact with blocks.
//! They are designed to be used with Bevy Observers for decoupled handling.

use bevy_ecs::prelude::{Entity, Message};

/// World coordinates for a block, stored as (x, y, z).
///
/// This is a simple coordinate type that avoids Debug issues with BlockPos.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockCoords {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockCoords {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

/// Message sent when a player attempts to interact with a block.
///
/// This is the primary entry point for the interaction pipeline. The packet
/// handler emits this message, and the interaction system processes it to
/// determine what action to take based on the block's components.
///
/// ## Message Flow
///
/// ```text
/// InteractPacket → BlockInteractMessage → InteractionSystem
///                                              ↓
///                            Query block entity by position
///                                              ↓
///                         Check InteractableBlock + capabilities
///                                              ↓
///                              Trigger appropriate Observer
/// ```
#[derive(Message, Clone, Debug)]
pub struct BlockInteractMessage {
    /// The player entity initiating the interaction
    pub player_entity: Entity,
    /// The world position of the target block
    pub block_coords: BlockCoords,
    /// Whether the player is sneaking (affects some interactions)
    pub sneaking: bool,
}

impl BlockInteractMessage {
    pub fn new(player_entity: Entity, x: i32, y: i32, z: i32, sneaking: bool) -> Self {
        Self {
            player_entity,
            block_coords: BlockCoords::new(x, y, z),
            sneaking,
        }
    }

    pub fn at(player_entity: Entity, coords: BlockCoords, sneaking: bool) -> Self {
        Self {
            player_entity,
            block_coords: coords,
            sneaking,
        }
    }
}

/// Message sent after a toggleable block changes state.
///
/// Listeners can use this to:
/// - Play sounds
/// - Update adjacent blocks (e.g., double doors)
/// - Sync state to clients
/// - Trigger redstone updates
#[derive(Message, Clone, Debug)]
pub struct BlockStateChangedMessage {
    /// The block entity that changed
    pub block_entity: Entity,
    /// World position of the block
    pub block_coords: BlockCoords,
    /// The new state (interpretation depends on block type)
    pub new_state: BlockState,
    /// The player who caused the change (if any)
    pub caused_by: Option<Entity>,
}

/// Represents the state of an interactive block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockState {
    /// For toggleable blocks: open/closed, active/inactive
    Toggle(bool),
    /// For redstone emitters: signal strength 0-15
    Signal(u8),
}

impl BlockState {
    pub fn open() -> Self {
        Self::Toggle(true)
    }

    pub fn closed() -> Self {
        Self::Toggle(false)
    }

    pub fn signal(strength: u8) -> Self {
        Self::Signal(strength.min(15))
    }

    pub fn is_active(&self) -> bool {
        match self {
            Self::Toggle(active) => *active,
            Self::Signal(strength) => *strength > 0,
        }
    }
}

/// Message sent when a container block is accessed.
///
/// The inventory system should listen for this to open the
/// appropriate container UI for the player.
#[derive(Message, Clone, Debug)]
pub struct ContainerAccessMessage {
    /// The container block entity
    pub block_entity: Entity,
    /// World position of the container
    pub block_coords: BlockCoords,
    /// The player accessing the container
    pub player_entity: Entity,
    /// Container type ID for the client UI
    pub container_type_id: u8,
    /// Number of slots in the container
    pub slot_count: u8,
}

impl ContainerAccessMessage {
    pub fn new(
        block_entity: Entity,
        coords: BlockCoords,
        player_entity: Entity,
        container_type_id: u8,
        slot_count: u8,
    ) -> Self {
        Self {
            block_entity,
            block_coords: coords,
            player_entity,
            container_type_id,
            slot_count,
        }
    }
}
