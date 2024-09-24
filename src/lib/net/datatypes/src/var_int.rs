use std::io::{Cursor, Read};

use super::traits::Readable;

pub struct VarInt(pub i32);

impl Readable for VarInt {
    fn read<Read: io::Read>(buffer: &mut Read) -> Result<Self>
    where
        Self: Sized {
        buffer.read_u8();
        Ok(Self(0))
    }
}
