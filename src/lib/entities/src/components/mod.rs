// Core entity components based on ferrumc-data
pub mod combat;
pub mod interaction;
pub mod last_synced_position;
pub mod metadata;
pub mod physical;
pub mod physical_registry;
pub mod spawn;

// Re-exports
pub use combat::CombatProperties;
pub use last_synced_position::LastSyncedPosition;
pub use metadata::EntityMetadata;
pub use physical::{BoundingBox, PhysicalProperties};
pub use physical_registry::PhysicalRegistry;
pub use spawn::SpawnProperties;

// Interaction components re-exports
pub use interaction::{
    // Core components
    BlockCoords,
    // Events
    BlockInteractEvent,
    BlockPosition,
    BlockToggledEvent,
    // Block type markers
    Button,
    Chest,
    // Capability components
    Container,
    ContainerOpenedEvent,
    ContainerType,
    Door,
    FenceGate,
    Furnace,
    // Aliases for backward compatibility
    Interactable,
    InteractableBlock,
    InteractionCooldown,
    Lever,
    Openable,
    RedstoneEmitter,
    Toggleable,
    Trapdoor,
};

// Marker component for baby entities
use bevy_ecs::prelude::Component;

/// Marker component for baby entities.
/// When present, physics systems will use baby-scaled properties.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Baby;
