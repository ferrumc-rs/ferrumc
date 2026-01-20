//! Emits `PlayerJoined` events after player entities are fully materialized.
//!
//! This system implements the "announce" phase of two-phase entity creation:
//! 1. `accept_new_connections` spawns entity + adds `PendingPlayerJoin` marker
//! 2. `apply_deferred` flushes commands (entity now exists)
//! 3. This system detects the marker and fires the actual event
//!
//! This ensures `PlayerJoined` events only fire when the entity is queryable.

use bevy_ecs::prelude::{Added, Commands, Entity, MessageWriter, Query};
use ferrumc_components::player::pending_events::PendingPlayerJoin;
use ferrumc_messages::player_join::PlayerJoined;
use tracing::trace;

/// Fires `PlayerJoined` events for newly spawned player entities.
///
/// Must run after `apply_deferred` to ensure entities are fully created.
/// Scheduled via `.chain()` in the game loop.
pub fn emit_player_joined(
    query: Query<(Entity, &PendingPlayerJoin), Added<PendingPlayerJoin>>,
    mut events: MessageWriter<PlayerJoined>,
    mut commands: Commands,
) {
    for (entity, pending) in query.iter() {
        trace!(
            "Emitting PlayerJoined event for {} ({:?})",
            pending.0.username,
            entity
        );

        events.write(PlayerJoined {
            identity: pending.0.clone(),
            entity,
        });

        // Remove marker so we don't fire again.
        // This removal is deferred but that's fine - Added<T> only fires once per addition.
        commands.entity(entity).remove::<PendingPlayerJoin>();
    }
}
