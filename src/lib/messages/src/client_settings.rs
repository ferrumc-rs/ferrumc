//! Client settings update messages.
//!
//! This module contains the ECS message types related to client settings changes.

use bevy_ecs::prelude::*;

/// Fired when a player updates their client information (settings).
///
/// This message is emitted when the server receives a `ClientInformation` packet
/// during the play state and the player's view distance has changed. Other systems
/// can listen for this message to react to view distance changes.
///
/// # Usage
///
/// Systems that need to react to view distance changes should add a
/// `MessageReader<ClientInformationUpdated>` parameter and process incoming events.
///
/// # Example
///
/// ```ignore
/// fn handle_view_distance_changes(
///     events: MessageReader<ClientInformationUpdated>,
/// ) {
///     for event in events.read() {
///         if event.view_distance_changed() {
///             // Handle the change
///         }
///     }
/// }
/// ```
#[derive(Message, Debug, Clone)]
pub struct ClientInformationUpdated {
    /// The entity (player) whose settings were updated.
    pub entity: Entity,
    /// The view distance before the update.
    pub old_view_distance: u8,
    /// The view distance after the update.
    pub new_view_distance: u8,
}

impl ClientInformationUpdated {
    /// Creates a new `ClientInformationUpdated` message.
    #[must_use]
    pub const fn new(entity: Entity, old_view_distance: u8, new_view_distance: u8) -> Self {
        Self {
            entity,
            old_view_distance,
            new_view_distance,
        }
    }

    /// Returns `true` if the view distance has changed.
    #[must_use]
    pub const fn view_distance_changed(&self) -> bool {
        self.old_view_distance != self.new_view_distance
    }

    /// Returns the difference in view distance (positive = increased, negative = decreased).
    #[must_use]
    pub const fn view_distance_delta(&self) -> i16 {
        self.new_view_distance as i16 - self.old_view_distance as i16
    }
}
