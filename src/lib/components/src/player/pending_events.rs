//! Marker components for two-phase entity lifecycle events.
//!
//! These markers enable the "spawn then announce" pattern where:
//! 1. Entity is created with a pending marker (deferred via Commands)
//! 2. Commands are flushed (entity now exists)
//! 3. A dedicated system detects the marker and fires the actual event
//!
//! This ensures events about entities only fire after the entity is fully queryable.
//! See `docs/_internal/deferred-commands-event-timing.md` for details.

use bevy_ecs::prelude::Component;
use ferrumc_core::identity::player_identity::PlayerIdentity;

/// Marker component indicating a player entity was just created and needs
/// a `PlayerJoined` event to be fired.
///
/// Added by `accept_new_connections`, removed by `emit_player_joined`.
#[derive(Component)]
pub struct PendingPlayerJoin(pub PlayerIdentity);
