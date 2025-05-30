use crate::decode::errors::NetDecodeError;
use crate::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use crate::encode::errors::NetEncodeError;
use crate::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use crate::net_types::NetTypesError;
use bitcode::{Decode, Encode};
use deepsize::DeepSizeOf;
use std::io::{Read, Write};
use tokio::io::AsyncRead;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncReadExt, AsyncWrite};

#[derive(Debug, Encode, Decode, Clone, DeepSizeOf, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct VarInt(pub i32);

mod adapters {
    use crate::net_types::var_int::VarInt;
    use std::ops::Add;
    use std::ops::Sub;

    impl From<usize> for VarInt {
        fn from(value: usize) -> Self {
            Self::new(value as i32)
        }
    }

    impl From<u8> for VarInt {
        fn from(value: u8) -> Self {
            Self::new(value as i32)
        }
    }

    impl From<i32> for VarInt {
        fn from(value: i32) -> Self {
            Self::new(value)
        }
    }

    impl Default for VarInt {
        fn default() -> Self {
            Self::new(0)
        }
    }

    impl PartialEq<usize> for VarInt {
        fn eq(&self, other: &usize) -> bool {
            let Ok(other) = i32::try_from(*other) else {
                return false;
            };
            self.0 == other
        }
    }

    impl Add<Self> for VarInt {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self::new(self.0 + other.0)
        }
    }

    impl Sub<Self> for VarInt {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Self::new(self.0 - other.0)
        }
    }
}

const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

impl VarInt {
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    #[expect(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        match self.0 {
            -128..128 => 1,
            -16384..16384 => 2,
            -2097152..2097152 => 3,
            -268435456..268435456 => 4,
            _ => 5,
        }
    }

    pub fn read<R: Read>(cursor: &mut R) -> Result<Self, NetTypesError> {
        let mut val = 0;
        for i in 0..5 {
            let byte = {
                let mut buf = [0u8; 1];
                cursor.read_exact(&mut buf)?;
                buf[0]
            } as i32;

            val |= (byte & SEGMENT_BITS) << (7 * i);
            if byte & CONTINUE_BIT == 0 {
                return Ok(Self::new(val));
            }
        }

        Err(NetTypesError::InvalidVarInt)
    }

    pub async fn read_async<R: AsyncRead + Unpin>(cursor: &mut R) -> Result<Self, NetTypesError> {
        let mut val = 0;
        for i in 0..5 {
            let byte = {
                let mut buf = [0u8; 1];
                cursor.read_exact(&mut buf).await?;
                buf[0]
            } as i32;

            val |= (byte & SEGMENT_BITS) << (7 * i);
            if byte & CONTINUE_BIT == 0 {
                return Ok(Self::new(val));
            }
        }

        Err(NetTypesError::InvalidVarInt)
    }

    pub fn write<W: Write>(&self, cursor: &mut W) -> Result<(), NetTypesError> {
        let VarInt(mut val) = self;
        loop {
            if (val & !SEGMENT_BITS) == 0 {
                cursor.write_all(&[val as u8])?;
                return Ok(());
            }

            cursor.write_all(&[((val & SEGMENT_BITS) | CONTINUE_BIT) as u8])?;
            val = ((val as u32) >> 7) as i32;
        }
    }

    pub async fn write_async<W: AsyncWrite + Unpin>(
        &self,
        cursor: &mut W,
    ) -> Result<(), NetTypesError> {
        let VarInt(mut val) = self;
        loop {
            if (val & !SEGMENT_BITS) == 0 {
                cursor.write_all(&[val as u8]).await?;
                return Ok(());
            }

            cursor
                .write_all(&[((val & SEGMENT_BITS) | CONTINUE_BIT) as u8])
                .await?;
            val = ((val as u32) >> 7) as i32; // Rust equivalent of Java's >>> operator
        }
    }
}

impl NetDecode for VarInt {
    fn decode<R: Read>(reader: &mut R, _opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        VarInt::read(reader).map_err(|e| NetDecodeError::ExternalError(e.into()))
    }
    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> NetDecodeResult<Self> {
        VarInt::read_async(reader)
            .await
            .map_err(|e| NetDecodeError::ExternalError(e.into()))
    }
}

impl NetEncode for VarInt {
    fn encode<W: Write>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        self.write(writer)
            .map_err(|e| NetEncodeError::ExternalError(e.into()))
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        self.write_async(writer)
            .await
            .map_err(|e| NetEncodeError::ExternalError(e.into()))
    }
}
