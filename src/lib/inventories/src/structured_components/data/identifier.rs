use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use regex::Regex;
use std::io::{Read, Write};
use std::str::FromStr;
use std::sync::LazyLock;
use tokio::io::{AsyncRead, AsyncWrite};

static NAMESPACE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z0-9._-]+$").unwrap());
static VALUE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z0-9._\-/]+$").unwrap());

#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct Identifier(String);

impl Identifier {
    pub const DEFAULT_NAMESPACE: &'static str = "minecraft";
    pub const MAX_LENGTH: usize = 32767;
    pub const SEPARATOR: char = ':';

    pub fn new(namespace: &str, value: &str) -> Result<Self, String> {
        if !NAMESPACE_REGEX.is_match(namespace) {
            return Err(format!("Invalid namespace. Expected \"^[a-z0-9._-]+$\" Found: {}", namespace));
        }

        if !VALUE_REGEX.is_match(value) {
            return Err(format!("Invalid value. Expected \"^[a-z0-9._-/]+$\" Found: {}", value));
        }

        let full = format!("{}:{}", namespace, value);

        if full.len() > Self::MAX_LENGTH {
            return Err("Identifier exceeds maximum length".into());
        }

        Ok(Self(full))
    }

    pub fn namespace(&self) -> &str {
        self.0
            .split_once(Self::SEPARATOR)
            .map_or(Self::DEFAULT_NAMESPACE, |(namespace, _)| namespace)
    }

    pub fn value(&self) -> &str {
        self.0
            .split_once(Self::SEPARATOR)
            .map_or(&self.0, |(_, val)| val)
    }
}

impl FromStr for Identifier {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.len() > Self::MAX_LENGTH {
            return Err("Identifier too long".into());
        }

        let (namespace, value) = string
            .split_once(Self::SEPARATOR)
            .unwrap_or((Self::DEFAULT_NAMESPACE, string));

        if !NAMESPACE_REGEX.is_match(namespace) {
            return Err(format!("Invalid namespace: {}", namespace));
        }

        if !VALUE_REGEX.is_match(value) {
            return Err(format!("Invalid value: {}", value));
        }

        Ok(Self(format!("{}:{}", namespace, value)))
    }
}

impl NetDecode for Identifier {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let raw = String::decode(reader, opts)?;

        Self::from_str(&raw).map_err(|e| NetDecodeError::ExternalError(e.into()))
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let raw = String::decode_async(reader, opts).await?;

        Self::from_str(&raw).map_err(|e| NetDecodeError::ExternalError(e.into()))
    }
}

impl NetEncode for Identifier {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.0.encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.0.encode_async(writer, opts).await
    }
}
