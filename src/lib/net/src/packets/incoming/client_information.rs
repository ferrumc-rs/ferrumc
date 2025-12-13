//! Client Information packet for Configuration state.
//!
//! This packet is sent by the client during the configuration phase to inform
//! the server of the player's client settings (locale, view distance, chat
//! preferences, skin parts, etc.).
//!
//! The enums defined here (`ChatMode`, `MainHand`, `ParticleStatus`) are also
//! re-exported by `client_information_play` for use during the Play state.

use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::fmt::Display;
use std::io::Read;
use tokio::io::AsyncRead;
use tracing::warn;
use typename::TypeName;

/// Client Information packet received during Configuration state.
///
/// Sent by the client during the configuration phase to inform the server
/// of the player's preferences and settings.
///
/// # Protocol
/// - Packet ID: `client_information`
/// - State: Configuration
/// - Bound to: Server
#[derive(TypeName, Debug, NetDecode)]
#[packet(packet_id = "client_information", state = "configuration")]
pub struct ClientInformation {
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

impl NetDecode for ChatMode {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let value = VarInt::decode(reader, opts)?;
        match value.0 as u8 {
            0 => Ok(ChatMode::Enabled),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            _ => {
                warn!(
                    "Received unknown chat mode value: {}, defaulting to Enabled",
                    value.0
                );
                Ok(ChatMode::Enabled) // Default to Enabled if unknown value
            }
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let value = VarInt::decode_async(reader, opts).await?;
        match value.0 as u8 {
            0 => Ok(ChatMode::Enabled),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            _ => {
                warn!(
                    "Received unknown chat mode value: {}, defaulting to Enabled",
                    value.0
                );
                Ok(ChatMode::Enabled) // Default to Enabled if unknown value
            }
        }
    }
}

impl Display for ChatMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatMode::Enabled => write!(f, "Enabled"),
            ChatMode::CommandsOnly => write!(f, "CommandsOnly"),
            ChatMode::Hidden => write!(f, "Hidden"),
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

impl NetDecode for MainHand {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let value = VarInt::decode(reader, opts)?;
        match value.0 as u8 {
            0 => Ok(MainHand::Left),
            1 => Ok(MainHand::Right),
            _ => {
                warn!(
                    "Received unknown main hand value: {}, defaulting to Left",
                    value.0
                );
                Ok(MainHand::Left) // Default to Left if unknown value
            }
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let value = VarInt::decode_async(reader, opts).await?;
        match value.0 as u8 {
            0 => Ok(MainHand::Left),
            1 => Ok(MainHand::Right),
            _ => {
                warn!(
                    "Received unknown main hand value: {}, defaulting to Left",
                    value.0
                );
                Ok(MainHand::Left) // Default to Left if unknown value
            }
        }
    }
}

impl Display for MainHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainHand::Left => write!(f, "Left"),
            MainHand::Right => write!(f, "Right"),
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

impl NetDecode for ParticleStatus {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let value = VarInt::decode(reader, opts)?;
        match value.0 as u8 {
            0 => Ok(ParticleStatus::All),
            1 => Ok(ParticleStatus::Decreased),
            2 => Ok(ParticleStatus::Minimal),
            _ => {
                warn!(
                    "Received unknown particle status value: {}, defaulting to All",
                    value.0
                );
                Ok(ParticleStatus::All) // Default to All if unknown value
            }
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let value = VarInt::decode_async(reader, opts).await?;
        match value.0 as u8 {
            0 => Ok(ParticleStatus::All),
            1 => Ok(ParticleStatus::Decreased),
            2 => Ok(ParticleStatus::Minimal),
            _ => {
                warn!(
                    "Received unknown particle status value: {}, defaulting to All",
                    value.0
                );
                Ok(ParticleStatus::All) // Default to All if unknown value
            }
        }
    }
}

impl Display for ParticleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParticleStatus::All => write!(f, "All"),
            ParticleStatus::Decreased => write!(f, "Decreased"),
            ParticleStatus::Minimal => write!(f, "Minimal"),
        }
    }
}

impl From<ParticleStatus> for u8 {
    fn from(status: ParticleStatus) -> Self {
        status as u8
    }
}
