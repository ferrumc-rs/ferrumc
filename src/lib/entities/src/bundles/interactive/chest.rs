//! Chest block bundle for interactive containers.
//!
//! Chests are container blocks that open an inventory UI when interacted with.
//! The interaction system emits a ContainerOpenedEvent which the inventory
//! system should handle.

use bevy_ecs::prelude::Bundle;
use ferrumc_world::pos::BlockPos;

use crate::components::{
    BlockPosition, Chest, Container, InteractableBlock, InteractionCooldown,
};

/// Bundle for spawning a chest block entity.
///
/// When a player interacts with the chest, a ContainerOpenedEvent is emitted.
/// The inventory system should listen for this event to open the chest UI.
///
/// ## Components
///
/// - `block_pos` - World grid position
/// - `interactable` - Marks the block as interactive
/// - `container` - Container properties (slots, type)
/// - `cooldown` - Prevents spam-clicking
/// - `chest` - Type marker
#[derive(Bundle)]
pub struct ChestBlockBundle {
    pub block_pos: BlockPosition,
    pub interactable: InteractableBlock,
    pub container: Container,
    pub cooldown: InteractionCooldown,
    pub chest: Chest,
}

impl ChestBlockBundle {
    /// Creates a new single chest (27 slots) at the given block position.
    pub fn new(pos: BlockPos) -> Self {
        Self {
            block_pos: BlockPosition::from_block_pos(pos),
            interactable: InteractableBlock,
            container: Container::chest(),
            cooldown: InteractionCooldown::default(),
            chest: Chest,
        }
    }

    /// Creates a double chest (54 slots) at the given block position.
    pub fn double(pos: BlockPos) -> Self {
        Self {
            block_pos: BlockPosition::from_block_pos(pos),
            interactable: InteractableBlock,
            container: Container::double_chest(),
            cooldown: InteractionCooldown::default(),
            chest: Chest,
        }
    }

    /// Creates a chest at xyz coordinates.
    pub fn at(x: i32, y: i32, z: i32) -> Self {
        Self::new(BlockPos::of(x, y, z))
    }
}
