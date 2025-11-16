use bevy_ecs::prelude::Component;

/// Stores all client-sent settings (locale, skin, etc.)
#[derive(Component, Debug, Clone, Default)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: u8, // can use an enum for this later
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: u8, // 0: Left, 1: Right
}
