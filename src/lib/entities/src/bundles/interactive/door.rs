//! Door block bundle for interactive doors.
//!
//! This creates a door as an ECS entity linked to a block position in the world.
//! The interaction system handles open/close logic generically through the
//! `Toggleable` component—no door-specific code in the handler.
//!
//! ## Example
//!
//! ```ignore
//! use ferrumc_world::pos::BlockPos;
//!
//! // Spawn a closed door at block coordinates
//! commands.spawn(DoorBlockBundle::new(BlockPos::of(10, 64, 10)));
//!
//! // Spawn an open door
//! commands.spawn(DoorBlockBundle::new_open(BlockPos::of(12, 64, 10)));
//! ```

use bevy_ecs::prelude::Bundle;
use ferrumc_world::pos::BlockPos;

use crate::components::{BlockPosition, Door, InteractableBlock, InteractionCooldown, Toggleable};

/// Bundle for spawning a door block entity.
///
/// Links an ECS entity to a block position in the world grid.
/// The door can be toggled open/closed by player interaction.
///
/// ## Components
///
/// - `block_pos` - World grid position of the door
/// - `interactable` - Marks the block as interactive
/// - `toggleable` - Tracks open/closed state
/// - `cooldown` - Prevents spam-clicking (200ms)
/// - `door` - Type marker for door-specific queries
///
/// ## How It Works
///
/// 1. Player right-clicks on the door's block position
/// 2. Packet handler looks up entity by `BlockPosition`
/// 3. Interaction system checks for `InteractableBlock`
/// 4. `Toggleable` component is toggled
/// 5. Observer broadcasts state change to clients
#[derive(Bundle)]
pub struct DoorBlockBundle {
    pub block_pos: BlockPosition,
    pub interactable: InteractableBlock,
    pub toggleable: Toggleable,
    pub cooldown: InteractionCooldown,
    pub door: Door,
}

impl DoorBlockBundle {
    /// Creates a new door at the given block position, starting closed.
    pub fn new(pos: BlockPos) -> Self {
        Self {
            block_pos: BlockPosition::from_block_pos(pos),
            interactable: InteractableBlock,
            toggleable: Toggleable::closed(),
            cooldown: InteractionCooldown::default(),
            door: Door,
        }
    }

    /// Creates a new door at the given block position, starting open.
    pub fn new_open(pos: BlockPos) -> Self {
        Self {
            block_pos: BlockPosition::from_block_pos(pos),
            interactable: InteractableBlock,
            toggleable: Toggleable::open(),
            cooldown: InteractionCooldown::default(),
            door: Door,
        }
    }

    /// Creates a door at xyz coordinates.
    pub fn at(x: i32, y: i32, z: i32) -> Self {
        Self::new(BlockPos::of(x, y, z))
    }
}

// Keep the old DoorBundle as an alias for backward compatibility
pub use DoorBlockBundle as DoorBundle;
