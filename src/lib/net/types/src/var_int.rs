use std::io::{Cursor, Read, Write};

pub struct VarInt {
    /// The value of the VarInt.
    pub val: i32,
    /// The length of the VarInt in bytes.
    pub len: usize,
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

    pub fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self, VarIntError> {
        let mut val = 0;
        for i in 0..5 {
            // let mut byte: [u8 ;1] = [0];
            // cursor.read_exact(&mut byte)?;
            let byte = {
                let mut buf = [0];
                cursor.read_exact(&mut buf)?;
                buf[0]
            } as i32;

            val |= (byte & SEGMENT_BITS) << (7 * i);
            if byte & CONTINUE_BIT == 0 {
                return Ok(Self { val, len: i + 1 });
            }
        }

        Err(VarIntError::InvalidVarInt)
    }

    pub fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), VarIntError> {
        write_varint(self.val, cursor)
    }
}

pub fn write_varint(
    value: impl TryInto<i32>,
    cursor: &mut Cursor<Vec<u8>>,
) -> Result<(), VarIntError> {
    let mut val = value.try_into().map_err(|_| VarIntError::InvalidVarInt)?;

    loop {
        if (val & !SEGMENT_BITS) == 0 {
            cursor.write_all(&[val as u8])?;
            return Ok(());
        }

        cursor.write_all(&[((val & SEGMENT_BITS) | CONTINUE_BIT) as u8])?;
        val = ((val as u32) >> 7) as i32; // Rust equivalent of Java's >>> operator
    }
}

#[derive(thiserror::Error, Debug)]
pub enum VarIntError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid VarInt")]
    InvalidVarInt,
    #[error("I couldn't convert the value into a valid i32")]
    InvalidInputI32,
}
