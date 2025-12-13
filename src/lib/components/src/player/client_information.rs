use bevy_ecs::prelude::Component;

/// Stores all client-sent settings (locale, skin, etc.)
/// 
/// This component is updated whenever the client sends a ClientInformation packet,
/// which can happen both during configuration and during active play when the
/// player changes their settings.
#[derive(Component, Debug, Clone, Default)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: u8, // 0: Enabled, 1: CommandsOnly, 2: Hidden
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: u8, // 0: Left, 1: Right
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
    pub particle_status: u8, // 0: All, 1: Decreased, 2: Minimal
}

impl ClientInformation {
    /// Creates a new ClientInformation with default values.
    pub fn new() -> Self {
        Self {
            locale: "en_us".to_string(),
            view_distance: 8,
            chat_mode: 0,
            chat_colors: true,
            displayed_skin_parts: 0x7F, // All parts enabled
            main_hand: 1, // Right hand
            enable_text_filtering: false,
            allow_server_listings: true,
            particle_status: 0, // All
        }
    }
}
