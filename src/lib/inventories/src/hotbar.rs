use bevy_ecs::prelude::Component;

/// The hotbar
#[derive(Component, Default)]
pub struct Hotbar {
    /// The currently selected slot in the hotbar.
    pub selected_slot: u8,
}
