use crate::netcode::components::potion_contents::PotionContents;
use crate::netcode::errors::{InvalidStructuredComponentEnumError, NotSupportedStructuredComponentError};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub enum StructuredComponent {
    #[default]
    Invalid,
    PotionContents(PotionContents),
}

impl StructuredComponent {
    fn to_id(&self) -> Result<VarInt, InvalidStructuredComponentEnumError> {
        match self {
            StructuredComponent::PotionContents(_) => Ok(VarInt::from(42)),
            StructuredComponent::Invalid => Err(InvalidStructuredComponentEnumError()),
        }
    }

    fn read<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode(reader, opts)?;

        let component = match id.0 {
            42 => StructuredComponent::PotionContents(PotionContents::decode(reader, opts)?),
            _ => Err(NotSupportedStructuredComponentError(id))?,
        };

        Ok(component)
    }

    async fn read_async<R: AsyncRead + Unpin>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode_async(reader, opts).await?;

        let component = match id.0 {
            42 => StructuredComponent::PotionContents(PotionContents::decode_async(reader, opts).await?),
            _ => Err(NotSupportedStructuredComponentError(id))?,
        };

        Ok(component)
    }
}

impl NetEncode for StructuredComponent {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        let id = self.to_id()?;

        id.encode(writer, opts)?;

        match self {
            StructuredComponent::PotionContents(potion_contents) => potion_contents.encode(writer, opts),
            StructuredComponent::Invalid => Err(InvalidStructuredComponentEnumError())?,
        }
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        let id = self.to_id()?;

        id.encode_async(writer, opts).await?;

        match self {
            StructuredComponent::PotionContents(potion_contents) => potion_contents.encode_async(writer, opts).await,
            StructuredComponent::Invalid => Err(InvalidStructuredComponentEnumError())?,
        }
    }
}

impl NetDecode for StructuredComponent {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        StructuredComponent::read(reader, opts)
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        StructuredComponent::read_async(reader, opts).await
    }
}
