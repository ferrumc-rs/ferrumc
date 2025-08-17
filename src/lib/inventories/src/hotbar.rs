use bevy_ecs::prelude::Component;

#[derive(Component, Default)]
pub struct Hotbar {
    pub selected_slot: u8,
}
