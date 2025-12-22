use crate::structured_components::data::TextComponent;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct WrittenBookContent {
    pub raw_title: String,
    pub filtered_title: PrefixedOptional<String>,
    pub author: String,
    pub generation: VarInt,
    pub pages: LengthPrefixedVec<Page>,
    pub is_resolved: bool,
}

#[derive(Debug, Clone, Hash, Default, PartialEq)]
pub struct Page {
    pub raw_text: TextComponent,
    pub filtered_text: PrefixedOptional<TextComponent>,
}

impl NetEncode for WrittenBookContent {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.raw_title.encode(writer, opts)?;
        self.filtered_title.encode(writer, opts)?;
        self.author.encode(writer, opts)?;
        self.generation.encode(writer, opts)?;
        self.pages.encode(writer, opts)?;
        self.is_resolved.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.raw_title.encode_async(writer, opts).await?;
        self.filtered_title.encode_async(writer, opts).await?;
        self.author.encode_async(writer, opts).await?;
        self.generation.encode_async(writer, opts).await?;
        self.pages.encode_async(writer, opts).await?;
        self.is_resolved.encode_async(writer, opts).await?;

        Ok(())
    }
}

impl NetDecode for WrittenBookContent {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let raw_title = String::decode(reader, opts)?;
        let filtered_title = PrefixedOptional::decode(reader, opts)?;
        let author = String::decode(reader, opts)?;
        let generation = VarInt::decode(reader, opts)?;
        let pages = LengthPrefixedVec::decode(reader, opts)?;
        let is_resolved = bool::decode(reader, opts)?;

        Ok(Self {
            raw_title,
            filtered_title,
            author,
            generation,
            pages,
            is_resolved,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let raw_title = String::decode_async(reader, opts).await?;
        let filtered_title = PrefixedOptional::decode_async(reader, opts).await?;
        let author = String::decode_async(reader, opts).await?;
        let generation = VarInt::decode_async(reader, opts).await?;
        let pages = LengthPrefixedVec::decode_async(reader, opts).await?;
        let is_resolved = bool::decode_async(reader, opts).await?;

        Ok(Self {
            raw_title,
            filtered_title,
            author,
            generation,
            pages,
            is_resolved,
        })
    }
}

impl NetEncode for Page {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.raw_text.encode(writer, opts)?;
        self.filtered_text.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.raw_text.encode_async(writer, opts).await?;
        self.filtered_text.encode_async(writer, opts).await?;

        Ok(())
    }
}

impl NetDecode for Page {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let raw_text = TextComponent::decode(reader, opts)?;
        let filtered_text = PrefixedOptional::decode(reader, opts)?;

        Ok(Self {
            raw_text,
            filtered_text,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let raw_text = TextComponent::decode_async(reader, opts).await?;
        let filtered_text = PrefixedOptional::decode_async(reader, opts).await?;

        Ok(Self {
            raw_text,
            filtered_text,
        })
    }
}
