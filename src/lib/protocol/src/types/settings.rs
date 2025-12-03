use crate::codec::decode::errors::NetDecodeError;
use crate::codec::decode::{NetDecode, NetDecodeOpts};
use crate::codec::net_types::var_int::VarInt;
use ferrumc_core::player::settings::{ChatMode, MainHand, ParticleStatus};
use std::io::Read;
use tokio::io::AsyncRead;
use tracing::warn;

impl NetDecode for ChatMode {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let value = VarInt::decode(reader, opts)?;
        match value.0 {
            0 => Ok(ChatMode::Enabled),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            _ => {
                warn!("Unknown chat mode: {}, defaulting to Enabled", value.0);
                Ok(ChatMode::Enabled)
            }
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let value = VarInt::decode_async(reader, opts).await?;
        match value.0 {
            0 => Ok(ChatMode::Enabled),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            _ => {
                warn!("Unknown chat mode: {}, defaulting to Enabled", value.0);
                Ok(ChatMode::Enabled)
            }
        }
    }
}

// Implement for MainHand and ParticleStatus similarly...
// (The logic is identical to what you had, just moved here)
impl NetDecode for MainHand {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let value = VarInt::decode(reader, opts)?;
        match value.0 {
            0 => Ok(MainHand::Left),
            1 => Ok(MainHand::Right),
            _ => {
                warn!("Unknown main hand: {}, defaulting to Right", value.0);
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
                    "Received unknown main hand value: {}, defaulting to Left",
                    value.0
                );
                Ok(MainHand::Left) // Default to Left if unknown value
            }
        }
    }
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
