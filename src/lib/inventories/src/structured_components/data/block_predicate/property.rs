use std::io::Read;
use tokio::io::AsyncRead;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Property {
    pub name: String,
    pub matcher: PropertyMatcher,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum PropertyMatcher {
    Exact(String),
    Range {
        min: String,
        max: String,
    },
}

impl NetDecode for Property {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let name = String::decode(reader, opts)?;
        let is_exact = bool::decode(reader, opts)?;

        let matcher = if is_exact {
            PropertyMatcher::Exact(String::decode(reader, opts)?)
        } else {
            let min = String::decode(reader, opts)?.into();
            let max = String::decode(reader, opts)?.into();
            PropertyMatcher::Range { min, max }
        };

        Ok(Property { name, matcher })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let name = String::decode_async(reader, opts).await?;
        let is_exact = bool::decode_async(reader, opts).await?;

        let matcher = if is_exact {
            PropertyMatcher::Exact(String::decode_async(reader, opts).await?)
        } else {
            let min = String::decode_async(reader, opts).await?;
            let max = String::decode_async(reader, opts).await?;
            PropertyMatcher::Range { min, max }
        };

        Ok(Property { name, matcher })
    }
}