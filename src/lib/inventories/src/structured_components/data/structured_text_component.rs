use ferrumc_nbt::de::borrow::NbtTag;
use ferrumc_nbt::de::converter::readers::async_reader::read_nbt_content_async;
use ferrumc_nbt::de::converter::readers::sync_reader::read_nbt_content_recursive;
use ferrumc_nbt::{FromNbt, NBTSerializable, NBTSerializeOptions, NbtTape};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_text::TextComponent;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};

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

fn create_compound_buffer() -> Vec<u8> {
    let mut buffer = Vec::with_capacity(256);
    buffer.push(10);
    buffer.push(0);
    buffer.push(0);

    buffer
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
        let tag_id = u8::decode(reader, opts)?;

        match tag_id {
            8 => {
                let length = u16::decode(reader, opts)?;
                let mut buf = vec![0u8; length as usize];
                reader.read_exact(&mut buf)?;
                let string = String::from_utf8(buf)?; //todo: from mutf
                Ok(StructuredTextComponent::Text(string))
            }
            10 => {
                let mut buffer = create_compound_buffer();

                let tag = NbtTag::Compound;
                read_nbt_content_recursive(&tag, reader, &mut buffer)
                    .map_err(|e| NetDecodeError::ExternalError(Box::new(e)))?;

                create_text_component_from_tape(&buffer)
            }
            _ => Err(NetDecodeError::ExternalError(
                "Invalid TextComponent Tag".into(),
            )),
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let tag_id = u8::decode_async(reader, opts).await?;

        match tag_id {
            8 => {
                let length = u16::decode_async(reader, opts).await?;
                let mut buf = vec![0u8; length as usize];
                reader.read_exact(&mut buf).await?;
                let string = String::from_utf8(buf)?; //todo: from mutf
                Ok(StructuredTextComponent::Text(string))
            }
            10 => {
                let mut buffer = create_compound_buffer();

                let tag = NbtTag::Compound;
                read_nbt_content_async(&tag, reader, &mut buffer)
                    .await
                    .map_err(|e| NetDecodeError::ExternalError(Box::new(e)))?;

                create_text_component_from_tape(&buffer)
            }
            _ => Err(NetDecodeError::ExternalError(
                format!("Unexpected NBT Tag: {}", tag_id).into(),
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
            StructuredTextComponent::Text(s) => {
                s.serialize_async(writer, &NBTSerializeOptions::Network)
                    .await;
            }
            StructuredTextComponent::Compound(c) => {
                c.serialize_async(writer, &NBTSerializeOptions::Network)
                    .await;
            }
        }
        Ok(())
    }
}
