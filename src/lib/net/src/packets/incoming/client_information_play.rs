//! Client Information packet for Play state.
//!
//! This packet is sent by the client when the player changes their settings
//! during gameplay (e.g., view distance, locale, skin parts, etc.).
//!
//! The structure is identical to the configuration state version, but is
//! handled separately during active gameplay and can trigger chunk loading changes.

use ferrumc_macros::{packet, NetDecode};
use typename::TypeName;

// Re-export the shared enums from the configuration packet
pub use super::client_information::{ChatMode, MainHand, ParticleStatus};

/// Client Information packet received during Play state.
///
/// Sent by the client when the player changes their settings in the options menu.
/// This triggers updates to the player's ECS component and may cause chunk
/// recalculation if view distance changed.
///
/// # Protocol
/// - Packet ID: `client_information` (0x0D in protocol 773)
/// - State: Play
/// - Bound to: Server
#[derive(TypeName, Debug, NetDecode)]
#[packet(packet_id = "client_information", state = "play")]
pub struct ClientInformationPlay {
    /// The client's locale (e.g., "en_us", "de_de").
    pub locale: String,
    /// The client's render distance in chunks (2-32).
    pub view_distance: i8,
    /// Chat visibility mode.
    pub chat_mode: ChatMode,
    /// Whether chat colors are enabled.
    pub chat_colors: bool,
    /// Bitmask of displayed skin parts.
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
