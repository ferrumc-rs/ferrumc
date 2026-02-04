//! Block Interaction System for Bevy 0.18
//!
//! This module implements a decoupled, extensible interaction system.
//! The system is designed to handle player-block interactions without
//! hardcoding specific block types in the core handler.
//!
//! ## Architecture Overview
//!
//! ```text
//! ┌─────────────────┐     ┌────────────────────┐     ┌─────────────────────┐
//! │ Interact Packet │────▶│ BlockInteractEvent │────▶│ handle_block_inter. │
//! │ (from client)   │     │ (via EventWriter)  │     │ (capability-based)  │
//! └─────────────────┘     └────────────────────┘     └─────────────────────┘
//!                                                              │
//!                                                              ▼
//!                                                    ┌─────────────────────┐
//!                                                    │ BlockToggledEvent   │
//!                                                    │ ContainerOpenedEvent│
//!                                                    │ (side effects)      │
//!                                                    └─────────────────────┘
//! ```
//!
//! ## How to Add a New Interactive Block Type
//!
//! 1. **Create a bundle** in `entities/src/bundles/interactive/`:
//!    ```ignore
//!    #[derive(Bundle)]
//!    pub struct MyBlockBundle {
//!        pub block_pos: BlockPosition,
//!        pub interactable: InteractableBlock,
//!        pub my_capability: MyCapability, // e.g., Toggleable, Container
//!        pub cooldown: InteractionCooldown,
//!        pub marker: MyBlock,
//!    }
//!    ```
//!
//! 2. **Spawn the block entity** and register it in the BlockEntityIndex:
//!    ```ignore
//!    let entity = commands.spawn(MyBlockBundle::new(pos)).id();
//!    index.insert(pos, entity);
//!    ```
//!
//! 3. **That's it!** The core system handles cooldowns and dispatch automatically.
//!    If you need type-specific side effects, add a system that reads
//!    `BlockToggledEvent` and queries for your marker component.

pub mod block_entity_index;

use bevy_ecs::message::MessageRegistry;
use bevy_ecs::prelude::*;
use ferrumc_entities::components::{
    BlockCoords, BlockInteractEvent, BlockPosition, BlockToggledEvent, Container,
    ContainerOpenedEvent, InteractableBlock, InteractionCooldown, Toggleable,
};
use tracing::{debug, trace};

pub use block_entity_index::BlockEntityIndex;

/// The core interaction handler system.
///
/// This system processes `BlockInteractEvent` and dispatches to the appropriate
/// capability handlers based on the target entity's components.
///
/// ## Responsibilities
///
/// 1. Look up block entity from BlockEntityIndex
/// 2. Check cooldown (skip if too soon)
/// 3. Verify entity has `InteractableBlock` component
/// 4. Dispatch to capability-specific logic:
///    - `Toggleable` → toggle state, emit `BlockToggledEvent`
///    - `Container` → emit `ContainerOpenedEvent`
/// 5. Update cooldown timer
///
/// ## Note
///
/// This system does NOT know about specific block types (Door, Lever, etc.).
/// It only knows about capabilities (Toggleable, Container). Block-specific
/// behavior should be implemented as separate systems that read the
/// side-effect events and query for type marker components.
pub fn handle_block_interaction(
    mut events: MessageReader<BlockInteractEvent>,
    mut query: Query<(
        &BlockPosition,
        &mut InteractionCooldown,
        Option<&mut Toggleable>,
        Option<&Container>,
    ), With<InteractableBlock>>,
    mut toggle_writer: MessageWriter<BlockToggledEvent>,
    mut container_writer: MessageWriter<ContainerOpenedEvent>,
) {
    for event in events.read() {
        let target = event.block_entity;

        // Query the target entity
        let Ok((block_pos, mut cooldown, toggleable, container)) = query.get_mut(target) else {
            trace!("Block interaction target {:?} is not interactable", target);
            continue;
        };

        // Check cooldown
        if !cooldown.can_interact() {
            trace!("Block interaction on cooldown for {:?}", target);
            continue;
        }

        // Record interaction time
        cooldown.record_interaction();

        let coords = BlockCoords::from_block_pos(block_pos.pos());

        debug!(
            "Block interaction at ({}, {}, {}) by player {:?}, sneaking: {}",
            coords.x, coords.y, coords.z, event.player, event.sneaking
        );

        // Handle Toggleable capability
        if let Some(mut toggle) = toggleable {
            toggle.toggle();
            let new_state = toggle.is_active;

            debug!(
                "Toggled block at ({}, {}, {}) to state: {}",
                coords.x, coords.y, coords.z, new_state
            );

            // Emit BlockToggledEvent for side effects (sound, sync, etc.)
            toggle_writer.write(BlockToggledEvent::new(
                target,
                block_pos.pos(),
                new_state,
                event.player,
            ));
        }

        // Handle Container capability
        if let Some(container) = container {
            debug!(
                "Opening container at ({}, {}, {}) for player {:?}",
                coords.x, coords.y, coords.z, event.player
            );

            // Emit ContainerOpenedEvent for inventory system
            container_writer.write(ContainerOpenedEvent::new(
                target,
                block_pos.pos(),
                event.player,
                container.container_type,
            ));
        }
    }
}

/// System that handles block toggle side effects.
///
/// This system reads `BlockToggledEvent` and can trigger type-specific
/// behavior by querying for marker components.
///
/// In production, this would:
/// - Play door open/close sounds
/// - Update the block state in the world
/// - Sync to clients
pub fn handle_block_toggled(
    mut events: MessageReader<BlockToggledEvent>,
    doors: Query<&ferrumc_entities::components::Door>,
    levers: Query<(&ferrumc_entities::components::Lever, &ferrumc_entities::components::RedstoneEmitter)>,
) {
    for event in events.read() {
        let coords = &event.block_coords;
        let target = event.block_entity;

        // Check if it's a door
        if doors.get(target).is_ok() {
            let state_str = if event.new_state { "opened" } else { "closed" };
            debug!(
                "Door {} at ({}, {}, {})",
                state_str, coords.x, coords.y, coords.z
            );

            // TODO: Play door sound, update world, sync to clients
        }

        // Check if it's a lever (note: levers need mutable access for redstone)
        // This would need to be in a separate system with mutable query
        if levers.contains(target) {
            let state_str = if event.new_state { "activated" } else { "deactivated" };
            debug!(
                "Lever {} at ({}, {}, {})",
                state_str, coords.x, coords.y, coords.z
            );

            // TODO: Update redstone signal, trigger propagation
        }
    }
}

/// System that handles container open events.
///
/// In production, the inventory system would handle this to open the UI.
pub fn handle_container_opened(
    mut events: MessageReader<ContainerOpenedEvent>,
) {
    for event in events.read() {
        let coords = &event.block_coords;

        debug!(
            "Container opened: entity={:?}, pos=({}, {}, {}), type={:?}, by={:?}",
            event.block_entity, coords.x, coords.y, coords.z, event.container_type, event.player
        );

        // TODO: Open inventory UI for player
        // This would typically send a packet to the client
    }
}

/// Registers all interaction-related messages with the app.
///
/// Call this during app setup to enable the interaction system.
///
/// ## Example
///
/// ```ignore
/// let mut world = World::new();
/// register_interaction_messages(&mut world);
/// ```
pub fn register_interaction_messages(world: &mut World) {
    MessageRegistry::register_message::<BlockInteractEvent>(world);
    MessageRegistry::register_message::<BlockToggledEvent>(world);
    MessageRegistry::register_message::<ContainerOpenedEvent>(world);
    world.init_resource::<BlockEntityIndex>();

    debug!("Block interaction messages registered");
}

/// Adds the interaction systems to a schedule.
///
/// ## Example
///
/// ```ignore
/// let mut schedule = Schedule::default();
/// add_interaction_systems(&mut schedule);
/// ```
pub fn add_interaction_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    use bevy_ecs::schedule::IntoScheduleConfigs;

    schedule.add_systems((
        handle_block_interaction,
        handle_block_toggled,
        handle_container_opened,
        block_entity_index::index_new_block_entities,
    ).chain());
}
