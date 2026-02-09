//! Block interaction components for the ECS-based interaction system.
//!
//! This module provides marker components and state components that define
//! how blocks respond to player interactions. The system is designed to be
//! fully decoupled: adding a new interaction type requires only creating
//! a new component and its corresponding Observer—no changes to the core
//! interaction handler.
//!
//! ## Architecture
//!
//! ```text
//! Player Click → InteractPacket → BlockInteractEvent → Observer dispatch
//!                                        ↓
//!                          Query for InteractableBlock
//!                                        ↓
//!                     Check capability components (Toggleable, Container, etc.)
//!                                        ↓
//!                          Trigger component-specific Observers
//! ```
//!
//! ## Required Components (Bevy 0.18)
//!
//! `InteractableBlock` uses Bevy's Required Components feature to ensure
//! every interactable block has necessary data like cooldown and position.

use bevy_ecs::prelude::{Component, Entity, Message};
use ferrumc_world::pos::BlockPos;
use std::fmt;
use std::time::{Duration, Instant};

// ============================================================================
// CORE INTERACTION COMPONENTS
// ============================================================================

/// Marker component for blocks that can be interacted with by players.
///
/// This is the entry point for the interaction system. Any block entity
/// with this component will be considered for interaction when a player
/// right-clicks on it.
///
/// ## Example
///
/// ```ignore
/// // A door block that can be toggled
/// commands.spawn((
///     BlockPosition(BlockPos::of(10, 64, 10)),
///     InteractableBlock,
///     Toggleable::closed(),
/// ));
/// ```
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct InteractableBlock;

/// Tracks the position of a block entity in the world grid.
///
/// This links an ECS entity to its corresponding block position in the world.
/// Used for spatial lookups and world synchronization.
#[derive(Component, Clone, Copy)]
pub struct BlockPosition(pub BlockPos);

impl fmt::Debug for BlockPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BlockPosition({}, {}, {})",
            self.0.pos.x, self.0.pos.y, self.0.pos.z
        )
    }
}

impl BlockPosition {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(BlockPos::of(x, y, z))
    }

    pub fn from_block_pos(pos: BlockPos) -> Self {
        Self(pos)
    }

    pub fn pos(&self) -> BlockPos {
        self.0
    }
}

/// Prevents interaction spam by enforcing a cooldown period.
///
/// After an interaction, further interactions are ignored until the
/// cooldown period expires. Default is 200ms.
#[derive(Component, Clone, Debug)]
pub struct InteractionCooldown {
    pub duration: Duration,
    pub last_interaction: Option<Instant>,
}

impl InteractionCooldown {
    /// Creates a new cooldown with the specified duration.
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            last_interaction: None,
        }
    }

    /// Creates a cooldown with the default 200ms duration.
    pub fn default_cooldown() -> Self {
        Self::new(Duration::from_millis(200))
    }

    /// Checks if the cooldown has expired and interaction is allowed.
    pub fn can_interact(&self) -> bool {
        match self.last_interaction {
            Some(last) => last.elapsed() >= self.duration,
            None => true,
        }
    }

    /// Records an interaction, resetting the cooldown timer.
    pub fn record_interaction(&mut self) {
        self.last_interaction = Some(Instant::now());
    }
}

impl Default for InteractionCooldown {
    fn default() -> Self {
        Self::default_cooldown()
    }
}

// ============================================================================
// CAPABILITY COMPONENTS
// ============================================================================

/// Component for blocks that can be toggled between two states.
///
/// Examples: doors, trapdoors, fence gates, levers, buttons.
///
/// The interaction system will automatically toggle the state when
/// an interaction event is received. Use an Observer to react to
/// state changes if you need custom behavior.
///
/// ## Usage
///
/// ```ignore
/// // Create a closed door
/// commands.spawn((
///     BlockPosition::new(10, 64, 10),
///     InteractableBlock,
///     Toggleable::closed(),
///     Door, // Type marker
/// ));
///
/// // The interaction system will handle toggling automatically
/// ```
#[derive(Component, Clone, Copy, Debug)]
pub struct Toggleable {
    pub is_active: bool,
}

impl Toggleable {
    pub fn new(is_active: bool) -> Self {
        Self { is_active }
    }

    pub fn closed() -> Self {
        Self::new(false)
    }

    pub fn open() -> Self {
        Self::new(true)
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    pub fn is_open(&self) -> bool {
        self.is_active
    }

    pub fn is_closed(&self) -> bool {
        !self.is_active
    }
}

impl Default for Toggleable {
    fn default() -> Self {
        Self::closed()
    }
}

/// Component for blocks that have an inventory/container.
///
/// Examples: chests, furnaces, hoppers, dispensers, barrels.
///
/// This component marks that the block should open a container UI
/// when interacted with. The actual inventory data is stored separately.
#[derive(Component, Clone, Debug)]
pub struct Container {
    /// Number of inventory slots
    pub slots: u8,
    /// Container type identifier for client UI
    pub container_type: ContainerType,
}

impl Container {
    pub fn new(slots: u8, container_type: ContainerType) -> Self {
        Self {
            slots,
            container_type,
        }
    }

    pub fn chest() -> Self {
        Self::new(27, ContainerType::Chest)
    }

    pub fn double_chest() -> Self {
        Self::new(54, ContainerType::DoubleChest)
    }

    pub fn furnace() -> Self {
        Self::new(3, ContainerType::Furnace)
    }

    pub fn hopper() -> Self {
        Self::new(5, ContainerType::Hopper)
    }

    pub fn dispenser() -> Self {
        Self::new(9, ContainerType::Dispenser)
    }

    pub fn barrel() -> Self {
        Self::new(27, ContainerType::Barrel)
    }
}

/// Type of container for UI rendering on the client.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerType {
    Chest,
    DoubleChest,
    Furnace,
    BlastFurnace,
    Smoker,
    Hopper,
    Dispenser,
    Dropper,
    Barrel,
    Shulker,
    Anvil,
    EnchantingTable,
    Brewing,
    Beacon,
    Loom,
    Cartography,
    Grindstone,
    Stonecutter,
    Smithing,
}

/// Component for blocks that emit a redstone signal when activated.
///
/// Examples: levers, buttons, pressure plates, tripwires.
#[derive(Component, Clone, Copy, Debug)]
pub struct RedstoneEmitter {
    /// Current signal strength (0-15)
    pub signal_strength: u8,
    /// Whether this is a momentary switch (button) or toggle (lever)
    pub momentary: bool,
    /// Duration for momentary switches (buttons typically 1 second)
    pub active_duration: Option<Duration>,
}

impl RedstoneEmitter {
    pub fn lever() -> Self {
        Self {
            signal_strength: 0,
            momentary: false,
            active_duration: None,
        }
    }

    pub fn button() -> Self {
        Self {
            signal_strength: 0,
            momentary: true,
            active_duration: Some(Duration::from_secs(1)),
        }
    }

    pub fn wooden_button() -> Self {
        Self {
            signal_strength: 0,
            momentary: true,
            active_duration: Some(Duration::from_millis(1500)),
        }
    }

    pub fn is_active(&self) -> bool {
        self.signal_strength > 0
    }

    pub fn activate(&mut self) {
        self.signal_strength = 15;
    }

    pub fn deactivate(&mut self) {
        self.signal_strength = 0;
    }

    pub fn toggle(&mut self) {
        if self.is_active() {
            self.deactivate();
        } else {
            self.activate();
        }
    }
}

// ============================================================================
// BLOCK TYPE MARKERS
// ============================================================================

/// Marker component for door blocks.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Door;

/// Marker component for trapdoor blocks.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Trapdoor;

/// Marker component for fence gate blocks.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct FenceGate;

/// Marker component for lever blocks.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Lever;

/// Marker component for button blocks.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Button;

/// Marker component for chest blocks.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Chest;

/// Marker component for furnace blocks.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Furnace;

// ============================================================================
// INTERACTION EVENTS
// ============================================================================

/// World coordinates for a block, stored as (x, y, z).
///
/// This wrapper avoids Debug issues with BlockPos while still
/// providing easy conversion.
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

    pub fn from_block_pos(pos: BlockPos) -> Self {
        Self {
            x: pos.pos.x,
            y: pos.pos.y,
            z: pos.pos.z,
        }
    }

    pub fn to_block_pos(self) -> BlockPos {
        BlockPos::of(self.x, self.y, self.z)
    }
}

impl From<BlockPos> for BlockCoords {
    fn from(pos: BlockPos) -> Self {
        Self::from_block_pos(pos)
    }
}

impl From<BlockCoords> for BlockPos {
    fn from(coords: BlockCoords) -> Self {
        coords.to_block_pos()
    }
}

/// Event emitted when a player interacts with a block.
///
/// This is the central event that triggers the interaction pipeline.
/// Systems and Observers listen for this event to react to player actions.
///
/// ## Event Flow
///
/// 1. Player sends interact packet
/// 2. Packet handler creates `BlockInteractEvent`
/// 3. Event is triggered on the target block entity
/// 4. Observers react based on the block's capability components
#[derive(Message, Clone, Debug)]
pub struct BlockInteractEvent {
    /// The player entity that initiated the interaction
    pub player: Entity,
    /// The block entity being interacted with
    pub block_entity: Entity,
    /// The world position of the block
    pub block_coords: BlockCoords,
    /// Whether the player is sneaking
    pub sneaking: bool,
}

impl BlockInteractEvent {
    pub fn new(player: Entity, block_entity: Entity, block_pos: BlockPos, sneaking: bool) -> Self {
        Self {
            player,
            block_entity,
            block_coords: BlockCoords::from_block_pos(block_pos),
            sneaking,
        }
    }

    /// Get the block position as BlockPos.
    pub fn block_pos(&self) -> BlockPos {
        self.block_coords.to_block_pos()
    }
}

/// Event emitted after a toggleable block changes state.
///
/// Observers can listen for this to trigger side effects like
/// playing sounds, updating connected blocks, or syncing to clients.
#[derive(Message, Clone, Debug)]
pub struct BlockToggledEvent {
    /// The block entity that was toggled
    pub block_entity: Entity,
    /// The world position of the block
    pub block_coords: BlockCoords,
    /// The new state (true = open/active)
    pub new_state: bool,
    /// The player who triggered the toggle
    pub triggered_by: Entity,
}

impl BlockToggledEvent {
    pub fn new(
        block_entity: Entity,
        block_pos: BlockPos,
        new_state: bool,
        triggered_by: Entity,
    ) -> Self {
        Self {
            block_entity,
            block_coords: BlockCoords::from_block_pos(block_pos),
            new_state,
            triggered_by,
        }
    }

    pub fn block_pos(&self) -> BlockPos {
        self.block_coords.to_block_pos()
    }
}

/// Event emitted when a container block is opened.
///
/// The inventory system should listen for this to open the container UI.
#[derive(Message, Clone, Debug)]
pub struct ContainerOpenedEvent {
    /// The container block entity
    pub block_entity: Entity,
    /// The world position of the block
    pub block_coords: BlockCoords,
    /// The player opening the container
    pub player: Entity,
    /// Container type for UI
    pub container_type: ContainerType,
}

impl ContainerOpenedEvent {
    pub fn new(
        block_entity: Entity,
        block_pos: BlockPos,
        player: Entity,
        container_type: ContainerType,
    ) -> Self {
        Self {
            block_entity,
            block_coords: BlockCoords::from_block_pos(block_pos),
            player,
            container_type,
        }
    }

    pub fn block_pos(&self) -> BlockPos {
        self.block_coords.to_block_pos()
    }
}

// ============================================================================
// ALIASES FOR BACKWARD COMPATIBILITY
// ============================================================================

/// Alias for `Toggleable` - used in the existing DoorBundle.
pub type Openable = Toggleable;

/// Alias for `InteractableBlock` - generic interactable marker.
pub type Interactable = InteractableBlock;
