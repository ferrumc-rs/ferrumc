use crate::decode::errors::NetDecodeError;
use crate::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use crate::encode::errors::NetEncodeError;
use crate::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use crate::net_types::NetTypesError;
use bitcode::{Decode, Encode};
use deepsize::DeepSizeOf;
use std::io::{Read, Write};

#[derive(Debug, Encode, Decode, Clone, DeepSizeOf, Eq)]
pub struct VarInt {
    /// The value of the VarInt.
    pub val: i32,
    /// The length of the VarInt in bytes.
    pub len: usize,
}

mod adapters {
    use crate::net_types::var_int::VarInt;
    use std::ops::Add;
    use std::ops::Sub;

    impl From<usize> for VarInt {
        fn from(value: usize) -> Self {
            Self::new(value as i32)
        }
    }

    impl From<VarInt> for u8 {
        fn from(value: VarInt) -> Self {
            value.val as u8
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
            self.val == *other as i32
        }
    }

    impl Add<Self> for VarInt {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self::new(self.val + other.val)
        }
    }

    impl Sub<Self> for VarInt {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Self::new(self.val - other.val)
        }
    }
}

impl PartialEq for VarInt {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

impl VarInt {
    pub fn new(value: i32) -> Self {
        Self {
            val: value,
            len: Self::calculate_len(value),
        }
    }

    pub fn calculate_len(value: i32) -> usize {
        if (-128..128).contains(&value) {
            1
        } else if (-16384..16384).contains(&value) {
            2
        } else if (-2097152..2097152).contains(&value) {
            3
        } else if (-268435456..268435456).contains(&value) {
            4
        } else {
            5
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
                return Ok(Self { val, len: i + 1 });
            }
        }

        Err(NetTypesError::InvalidVarInt)
    }

    pub fn write<W: Write>(&self, cursor: &mut W) -> Result<(), NetTypesError> {
        let mut val = self.val;
        loop {
            if (val & !SEGMENT_BITS) == 0 {
                cursor.write_all(&[val as u8])?;
                return Ok(());
            }

            cursor.write_all(&[((val & SEGMENT_BITS) | CONTINUE_BIT) as u8])?;
            val = ((val as u32) >> 7) as i32; // Rust equivalent of Java's >>> operator
        }
    }
}

impl NetDecode for VarInt {
    fn decode<R: Read>(reader: &mut R, _opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        VarInt::read(reader).map_err(|e| NetDecodeError::ExternalError(e.into()))
    }
}

impl NetEncode for VarInt {
    fn encode<W: Write>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        self.write(writer)
            .map_err(|e| NetEncodeError::ExternalError(e.into()))
    }
}
