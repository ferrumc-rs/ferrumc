//! Client Information component for player entities.
//!
//! This module contains the ECS component that stores client-sent settings
//! for each player, such as locale, view distance, and display preferences.

use bevy_ecs::prelude::Component;

/// Chat visibility mode for the client.
///
/// Controls which chat messages the client wants to receive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ChatMode {
    /// All chat messages are shown.
    #[default]
    Enabled = 0,
    /// Only command feedback is shown.
    CommandsOnly = 1,
    /// All chat messages are hidden.
    Hidden = 2,
}

impl From<u8> for ChatMode {
    fn from(value: u8) -> Self {
        match value {
            0 => ChatMode::Enabled,
            1 => ChatMode::CommandsOnly,
            2 => ChatMode::Hidden,
            _ => ChatMode::Enabled,
        }
    }
}

impl From<ChatMode> for u8 {
    fn from(mode: ChatMode) -> Self {
        mode as u8
    }
}

/// The player's dominant hand preference.
///
/// Used for item placement and attack animations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum MainHand {
    /// Left hand is dominant.
    Left = 0,
    /// Right hand is dominant.
    #[default]
    Right = 1,
}

impl From<u8> for MainHand {
    fn from(value: u8) -> Self {
        match value {
            0 => MainHand::Left,
            1 => MainHand::Right,
            _ => MainHand::Right,
        }
    }
}

impl From<MainHand> for u8 {
    fn from(hand: MainHand) -> Self {
        hand as u8
    }
}

/// Particle rendering level preference.
///
/// Controls how many particles the client renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ParticleStatus {
    /// All particles are rendered.
    #[default]
    All = 0,
    /// Reduced particle rendering.
    Decreased = 1,
    /// Minimal particle rendering.
    Minimal = 2,
}

impl From<u8> for ParticleStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => ParticleStatus::All,
            1 => ParticleStatus::Decreased,
            2 => ParticleStatus::Minimal,
            _ => ParticleStatus::All,
        }
    }
}

impl From<ParticleStatus> for u8 {
    fn from(status: ParticleStatus) -> Self {
        status as u8
    }
}

/// Stores all client-sent settings (locale, skin parts, preferences, etc.).
///
/// This component is updated whenever the client sends a `ClientInformation` packet,
/// which can happen both during configuration and during active play when the
/// player changes their settings in the options menu.
///
/// # View Distance
/// The `view_distance` field stores the effective render distance for chunk loading,
/// which is the minimum of the server's configured distance and the client's requested
/// distance. This ensures optimal performance for both server and client.
///
/// # Skin Parts
/// The `displayed_skin_parts` field is a bitmask where each bit represents a
/// toggleable skin layer:
/// - Bit 0 (0x01): Cape
/// - Bit 1 (0x02): Jacket
/// - Bit 2 (0x04): Left Sleeve
/// - Bit 3 (0x08): Right Sleeve
/// - Bit 4 (0x10): Left Pants
/// - Bit 5 (0x20): Right Pants
/// - Bit 6 (0x40): Hat
#[derive(Component, Debug, Clone)]
pub struct ClientInformation {
    /// The client's locale (e.g., "en_us", "de_de").
    pub locale: String,
    /// Effective render distance in chunks.
    pub view_distance: u8,
    /// Chat visibility mode.
    pub chat_mode: ChatMode,
    /// Whether chat colors are enabled.
    pub chat_colors: bool,
    /// Bitmask of displayed skin parts (see module docs).
    pub displayed_skin_parts: u8,
    /// The player's main hand preference.
    pub main_hand: MainHand,
    /// Whether text filtering is enabled.
    pub enable_text_filtering: bool,
    /// Whether the player appears in server listings.
    pub allow_server_listings: bool,
    /// Particle rendering level.
    pub particle_status: ParticleStatus,
}

impl Default for ClientInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientInformation {
    /// Creates a new `ClientInformation` with sensible default values.
    ///
    /// Defaults to:
    /// - Locale: "en_us"
    /// - View distance: 8 chunks
    /// - Chat mode: Enabled
    /// - Chat colors: true
    /// - All skin parts displayed
    /// - Right hand dominant
    /// - Text filtering disabled
    /// - Server listings allowed
    /// - All particles shown
    #[must_use]
    pub fn new() -> Self {
        Self {
            locale: "en_us".to_string(),
            view_distance: 8,
            chat_mode: ChatMode::default(),
            chat_colors: true,
            displayed_skin_parts: 0x7F, // All parts enabled
            main_hand: MainHand::default(),
            enable_text_filtering: false,
            allow_server_listings: true,
            particle_status: ParticleStatus::default(),
        }
    }
}
