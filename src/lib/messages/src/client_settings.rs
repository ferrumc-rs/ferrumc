use bevy_ecs::prelude::*;

/// Fired when a player updates their client information (settings).
///
/// This includes changes to:
/// - View distance (triggers chunk recalculation)
/// - Locale
/// - Chat mode
/// - Skin parts visibility
/// - Main hand preference
/// - etc.
///
/// Fired by: `client_information` packet handler in play state
/// Listened for by: `client_settings_changed` system to handle view distance changes
#[derive(Message, Clone)]
pub struct ClientInformationUpdated {
    /// The entity (player) whose settings were updated
    pub entity: Entity,
    /// The old view distance before the update
    pub old_view_distance: u8,
    /// The new view distance after the update
    pub new_view_distance: u8,
}

impl ClientInformationUpdated {
    pub fn new(entity: Entity, old_view_distance: u8, new_view_distance: u8) -> Self {
        Self {
            entity,
            old_view_distance,
            new_view_distance,
        }
    }

    /// Returns true if the view distance changed
    pub fn view_distance_changed(&self) -> bool {
        self.old_view_distance != self.new_view_distance
    }
}
