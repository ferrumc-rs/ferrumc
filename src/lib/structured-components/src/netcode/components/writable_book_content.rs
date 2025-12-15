use crate::netcode::errors::MaxLimitExceededError;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

const MAX_PAGES: usize = 100;
const MAX_PAGE_LENGTH: usize = 1024;

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct WritableBookContent {
    pub pages: LengthPrefixedVec<Page>,
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Page {
    pub raw_text: String,
    pub filtered_text: PrefixedOptional<String>,
}

fn throw_if_length_exceeds(
    value_count: usize,
    max_length: usize,
    limit_type: &'static str,
) -> Result<(), MaxLimitExceededError> {
    if value_count > max_length {
        Err(MaxLimitExceededError::new(
            limit_type,
            value_count,
            max_length,
        ))
    } else {
        Ok(())
    }
}

impl NetDecode for Page {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let raw_text = String::decode(reader, opts)?;

        throw_if_length_exceeds(raw_text.len(), MAX_PAGE_LENGTH, "raw_text")?;

        let filtered_text: PrefixedOptional<String> = PrefixedOptional::decode(reader, opts)?;

        if let PrefixedOptional::Some(text) = &filtered_text {
            throw_if_length_exceeds(text.len(), MAX_PAGE_LENGTH, "filtered_text")?;
        }

        Ok(Page {
            raw_text,
            filtered_text,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let raw_text = String::decode_async(reader, opts).await?;

        throw_if_length_exceeds(raw_text.len(), MAX_PAGE_LENGTH, "raw_text")?;

        let filtered_text: PrefixedOptional<String> =
            PrefixedOptional::decode_async(reader, opts).await?;

        if let PrefixedOptional::Some(text) = &filtered_text {
            throw_if_length_exceeds(text.len(), MAX_PAGE_LENGTH, "filtered_text")?;
        }

        Ok(Page {
            raw_text,
            filtered_text,
        })
    }
}

impl NetEncode for Page {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        throw_if_length_exceeds(self.raw_text.len(), MAX_PAGE_LENGTH, "raw_text")?;
        if let PrefixedOptional::Some(text) = &self.filtered_text {
            throw_if_length_exceeds(text.len(), MAX_PAGE_LENGTH, "filtered_text")?;
        }
        self.raw_text.encode(writer, opts)?;
        self.filtered_text.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        throw_if_length_exceeds(self.raw_text.len(), MAX_PAGE_LENGTH, "raw_text")?;
        if let PrefixedOptional::Some(text) = &self.filtered_text {
            throw_if_length_exceeds(text.len(), MAX_PAGE_LENGTH, "filtered_text")?;
        }
        self.raw_text.encode_async(writer, opts).await?;
        self.filtered_text.encode_async(writer, opts).await?;

        Ok(())
    }
}

impl NetDecode for WritableBookContent {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let pages = LengthPrefixedVec::decode(reader, opts)?;

        throw_if_length_exceeds(pages.data.len(), MAX_PAGES, "pages")?;

        Ok(WritableBookContent { pages })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let pages = LengthPrefixedVec::decode_async(reader, opts).await?;

        throw_if_length_exceeds(pages.data.len(), MAX_PAGES, "pages")?;

        Ok(WritableBookContent { pages })
    }
}

impl NetEncode for WritableBookContent {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        throw_if_length_exceeds(self.pages.data.len(), MAX_PAGES, "pages")?;

        self.pages.encode(writer, opts)?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        throw_if_length_exceeds(self.pages.data.len(), MAX_PAGES, "pages")?;

        self.pages.encode_async(writer, opts).await?;

        Ok(())
    }
}
