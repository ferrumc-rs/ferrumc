use crate::structured_components::data::utils;
use ferrumc_nbt::{FromNbt, NBTSerializable, NBTSerializeOptions, NbtTape};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_text::TextComponent;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum StructuredTextComponent {
    Text(String),
    Compound(TextComponent),
}

impl Default for StructuredTextComponent {
    fn default() -> Self {
        StructuredTextComponent::Text(String::new())
    }
}

fn create_text_component_from_tape(
    buffer: &[u8],
) -> Result<StructuredTextComponent, NetDecodeError> {
    let mut tape = NbtTape::new(buffer);
    tape.parse();

    if let Some(root) = &tape.root {
        let compound = TextComponent::from_nbt(&tape, &root.1)
            .map_err(|e| NetDecodeError::ExternalError(Box::new(e)))?;

        Ok(StructuredTextComponent::Compound(compound))
    } else {
        Ok(StructuredTextComponent::default())
    }
}

impl NetDecode for StructuredTextComponent {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        match u8::decode(reader, opts)? {
            8 => Ok(StructuredTextComponent::Text(utils::read_mutf8_string(
                reader,
            )?)),
            10 => {
                let buffer = utils::decode_compound_to_buffer_raw(reader)?;
                create_text_component_from_tape(&buffer)
            }
            tag => Err(NetDecodeError::ExternalError(
                format!("Invalid Tag {}", tag).into(),
            )),
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        match u8::decode_async(reader, opts).await? {
            8 => Ok(StructuredTextComponent::Text(
                utils::read_mutf8_string_async(reader).await?,
            )),
            10 => {
                let buffer = utils::decode_compound_to_buffer_raw_async(reader).await?;
                create_text_component_from_tape(&buffer)
            }
            tag => Err(NetDecodeError::ExternalError(
                format!("Invalid Tag {}", tag).into(),
            )),
        }
    }
}

impl NetEncode for StructuredTextComponent {
    fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match self {
            StructuredTextComponent::Text(s) => {
                s.serialize(writer, &NBTSerializeOptions::Network);
            }
            StructuredTextComponent::Compound(c) => {
                c.serialize(writer, &NBTSerializeOptions::Network);
            }
        }
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        match self {
            StructuredTextComponent::Text(string) => {
                string
                    .serialize_async(writer, &NBTSerializeOptions::Network)
                    .await;
            }
            StructuredTextComponent::Compound(component) => {
                component
                    .serialize_async(writer, &NBTSerializeOptions::Network)
                    .await;
            }
        }
        Ok(())
    }
}
