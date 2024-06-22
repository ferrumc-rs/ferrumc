use std::io::{Read, Seek};

use crate::encoding::varint::{read_varint, VarInt};
use crate::error::Error;

pub trait Decode {
    #[allow(unused)]
    fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: Read + Seek;
}

impl Decode for bool {
    fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: Read + Seek,
    {
        let mut buf = [0u8; 1];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| Error::Generic("Failed to read bool".parse().unwrap()))?;
        Ok(Box::from(buf[0] != 0))
    }
}

impl Decode for u8 {
    fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: Read + Seek,
    {
        let mut buf = [0u8; 1];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| Error::Generic("Failed to read u8".parse().unwrap()))?;
        Ok(Box::from(buf[0]))
    }
}

impl Decode for i8 {
    fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: Read + Seek,
    {
        let mut buf = [0u8; 1];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| Error::Generic("Failed to read i8".parse().unwrap()))?;
        Ok(Box::from(buf[0] as i8))
    }
}

impl Decode for u16 {
    fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: Read + Seek,
    {
        let mut buf = [0u8; 2];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| Error::Generic("Failed to read u16".parse().unwrap()))?;
        Ok(Box::from(u16::from_be_bytes(buf)))
    }
}

impl Decode for i16 {
    fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: Read + Seek,
    {
        let mut buf = [0u8; 2];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| Error::Generic("Failed to read i16".parse().unwrap()))?;
        Ok(Box::from(i16::from_be_bytes(buf)))
    }
}

impl Decode for u32 {
    fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: Read + Seek,
    {
        let mut buf = [0u8; 4];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| Error::Generic("Failed to read u32".parse().unwrap()))?;
        Ok(Box::from(u32::from_be_bytes(buf)))
    }
}

impl Decode for i32 {
    fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: Read + Seek,
    {
        let mut buf = [0u8; 4];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| Error::Generic("Failed to read i32".parse().unwrap()))?;
        Ok(Box::from(i32::from_be_bytes(buf)))
    }
}

impl Decode for i64 {
    fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: Read + Seek,
    {
        let mut buf = [0u8; 8];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| Error::Generic("Failed to read i64".parse().unwrap()))?;
        Ok(Box::from(i64::from_be_bytes(buf)))
    }
}

impl Decode for f32 {
    fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: Read + Seek,
    {
        let mut buf = [0u8; 4];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| Error::Generic("Failed to read f32".parse().unwrap()))?;
        Ok(Box::from(f32::from_be_bytes(buf)))
    }
}

impl Decode for f64 {
    fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: Read + Seek,
    {
        let mut buf = [0u8; 8];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| Error::Generic("Failed to read f64".parse().unwrap()))?;
        Ok(Box::from(f64::from_be_bytes(buf)))
    }
}

impl Decode for String {
    // Now this one is a bit more complicated. The first few bytes are the len as a varint, but we
    // can't be sure how many bytes it will take up. We also can't be sure the entire varint has
    // arrived yet. So we need to read bytes until we have the entire varint, then read the string.
    fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: Read + Seek,
    {
        let remaining_bytes = read_varint(bytes)?;
        let mut string_buf = vec![0u8; remaining_bytes.into()];
        bytes.read_exact(&mut string_buf)?;
        Ok(Box::from(String::from_utf8(string_buf)?))
    }
}

impl Decode for VarInt {
    fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: Read + Seek,
    {
        Ok(Box::from(read_varint(bytes)?))
    }
}
