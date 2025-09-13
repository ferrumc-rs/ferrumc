use crate::decode::errors::NetDecodeError;
use crate::decode::{NetDecode, NetDecodeOpts};
use crate::encode::errors::NetEncodeError;
use crate::encode::{NetEncode, NetEncodeOpts};
use bitcode::{Decode, Encode};
use std::fmt::Display;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrefixedOptional<T> {
    None,
    Some(T),
}

impl<T: NetEncode> NetEncode for PrefixedOptional<T> {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match self {
            PrefixedOptional::None => {
                false.encode(writer, opts)?;
            }
            PrefixedOptional::Some(value) => {
                true.encode(writer, opts)?;
                value.encode(writer, opts)?;
            }
        }
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        match self {
            PrefixedOptional::None => {
                false.encode_async(writer, opts).await?;
            }
            PrefixedOptional::Some(value) => {
                true.encode_async(writer, opts).await?;
                value.encode_async(writer, opts).await?;
            }
        }
        Ok(())
    }
}

impl<T> PrefixedOptional<T> {
    pub fn new(value: Option<T>) -> Self {
        match value {
            Some(v) => PrefixedOptional::Some(v),
            None => PrefixedOptional::None,
        }
    }

    pub fn is_some(&self) -> bool {
        matches!(self, PrefixedOptional::Some(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, PrefixedOptional::None)
    }

    pub fn unwrap(self) -> T {
        match self {
            PrefixedOptional::Some(value) => value,
            PrefixedOptional::None => panic!("Called `unwrap` on a `PrefixedOptional::None`"),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            PrefixedOptional::Some(value) => value,
            PrefixedOptional::None => default,
        }
    }

    pub fn to_option(self) -> Option<T> {
        match self {
            PrefixedOptional::Some(value) => Some(value),
            PrefixedOptional::None => None,
        }
    }
}

impl<T: NetDecode> NetDecode for PrefixedOptional<T> {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let exists = bool::decode(reader, opts)?;
        if !exists {
            Ok(PrefixedOptional::None)
        } else {
            let value = T::decode(reader, opts)?;
            Ok(PrefixedOptional::Some(value))
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let exists = bool::decode_async(reader, opts).await?;
        if !exists {
            Ok(PrefixedOptional::None)
        } else {
            let value = T::decode_async(reader, opts).await?;
            Ok(PrefixedOptional::Some(value))
        }
    }
}

impl<T: Display> Display for PrefixedOptional<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrefixedOptional::None => write!(f, "None"),
            PrefixedOptional::Some(value) => write!(f, "Some({})", value),
        }
    }
}
