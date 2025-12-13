use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::fmt::Display;
use std::io::Read;
use tokio::io::AsyncRead;
use tracing::warn;
use typename::TypeName;

/// Client Information packet received during Play state.
/// 
/// This packet is sent by the client when the player changes their settings
/// (e.g., view distance, locale, skin parts, etc.).
/// 
/// The structure is identical to the configuration state version, but this
/// one is handled during active gameplay and can trigger chunk loading changes.
#[derive(TypeName, Debug, NetDecode)]
#[packet(packet_id = "client_information", state = "play")]
pub struct ClientInformationPlay {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
    pub particle_status: ParticleStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
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
                Ok(ChatMode::Enabled)
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
                Ok(ChatMode::Enabled)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainHand {
    Left,
    Right,
}

impl NetDecode for MainHand {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let value = VarInt::decode(reader, opts)?;
        match value.0 as u8 {
            0 => Ok(MainHand::Left),
            1 => Ok(MainHand::Right),
            _ => {
                warn!(
                    "Received unknown main hand value: {}, defaulting to Right",
                    value.0
                );
                Ok(MainHand::Right)
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
                    "Received unknown main hand value: {}, defaulting to Right",
                    value.0
                );
                Ok(MainHand::Right)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleStatus {
    All,
    Decreased,
    Minimal,
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
                Ok(ParticleStatus::All)
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
                Ok(ParticleStatus::Minimal)
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
