//! Lever block bundle for interactive levers.
//!
//! Levers are toggleable blocks that emit a redstone signal when activated.
//! The interaction system handles the toggle automatically.

use bevy_ecs::prelude::Bundle;
use ferrumc_world::pos::BlockPos;

use crate::components::{
    BlockPosition, InteractableBlock, InteractionCooldown, Lever, RedstoneEmitter, Toggleable,
};

/// Bundle for spawning a lever block entity.
///
/// The lever can be toggled on/off by player interaction.
/// When active, it emits a redstone signal (strength 15).
///
/// ## Components
///
/// - `block_pos` - World grid position
/// - `interactable` - Marks the block as interactive
/// - `toggleable` - Tracks on/off state
/// - `redstone` - Emits redstone signal when active
/// - `cooldown` - Prevents spam-clicking
/// - `lever` - Type marker
#[derive(Bundle)]
pub struct LeverBlockBundle {
    pub block_pos: BlockPosition,
    pub interactable: InteractableBlock,
    pub toggleable: Toggleable,
    pub redstone: RedstoneEmitter,
    pub cooldown: InteractionCooldown,
    pub lever: Lever,
}

impl LeverBlockBundle {
    /// Creates a new lever at the given block position, starting off.
    pub fn new(pos: BlockPos) -> Self {
        Self {
            block_pos: BlockPosition::from_block_pos(pos),
            interactable: InteractableBlock,
            toggleable: Toggleable::closed(),
            redstone: RedstoneEmitter::lever(),
            cooldown: InteractionCooldown::default(),
            lever: Lever,
        }
    }

    /// Creates a lever that starts in the on position.
    pub fn new_on(pos: BlockPos) -> Self {
        let mut bundle = Self::new(pos);
        bundle.toggleable = Toggleable::open();
        bundle.redstone.activate();
        bundle
    }

    /// Creates a lever at xyz coordinates.
    pub fn at(x: i32, y: i32, z: i32) -> Self {
        Self::new(BlockPos::of(x, y, z))
    }
}
