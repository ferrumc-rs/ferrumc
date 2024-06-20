use std::io::{Cursor, Read, Write};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
use crate::error::Error;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use tokio::runtime::Runtime;
    use crate::error::Error;

    #[test]
    fn read_varint_valid_input() {
        let mut runtime = Runtime::new().unwrap();
        let mut cursor = Cursor::new(vec![0x80, 0x80, 0x80, 0x80, 0x08]);
        let result = runtime.block_on(read_varint(&mut cursor));
        assert_eq!(result.unwrap(), -2147483648);
    }

    #[test]
    fn read_varint_too_big() {
        let mut runtime = Runtime::new().unwrap();
        let mut cursor = Cursor::new(vec![0b10000000; 6]);
        let result = runtime.block_on(read_varint(&mut cursor));
        assert!(result.is_err());
    }

    #[test]
    fn write_varint_valid_input() {
        let mut runtime = Runtime::new().unwrap();
        let mut cursor = Cursor::new(Vec::new());
        let result = runtime.block_on(write_varint(2097151, &mut cursor));
        assert!(result.is_ok());
        assert_eq!(cursor.into_inner(), vec![0xff, 0xff, 0x7f]);
    }

    #[test]
    fn write_varint_zero() {
        let mut runtime = Runtime::new().unwrap();
        let mut cursor = Cursor::new(Vec::new());
        let result = runtime.block_on(write_varint(0, &mut cursor));
        assert!(result.is_ok());
        assert_eq!(cursor.into_inner(), vec![0b00000000]);
    }

    #[test]
    fn read_varint_empty_input() {
        let mut runtime = Runtime::new().unwrap();
        let mut cursor = Cursor::new(vec![]);
        let result = runtime.block_on(read_varint(&mut cursor));
        assert!(result.is_err());
    }

    #[test]
    fn read_varint_single_byte() {
        let mut runtime = Runtime::new().unwrap();
        let mut cursor = Cursor::new(vec![0b00000001]);
        let result = runtime.block_on(read_varint(&mut cursor));
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn write_varint_negative_input() {
        let mut runtime = Runtime::new().unwrap();
        let mut cursor = Cursor::new(Vec::new());
        let result = runtime.block_on(write_varint(-1, &mut cursor));
        assert!(result.is_ok());
        assert_eq!(cursor.into_inner(), vec![0xff, 0xff, 0xff, 0xff, 0x0f]);
    }
}

// Read a VarInt from the given cursor.
// Yoinked from valence: https://github.com/valence-rs/valence/blob/main/crates/valence_protocol/src/var_int.rs#L69
pub async fn read_varint<T>(cursor: &mut T) -> crate::prelude::Result<i32>
where
    T: Read + AsyncRead + Unpin
{
    let mut val = 0;
    for i in 0..5 {
        let byte = cursor.read_u8().await?;
        val |= (i32::from(byte) & 0b01111111) << (i * 7);
        if byte & 0b10000000 == 0 {
            return Ok(val);
        }
    }
    Err(Error::Generic("VarInt is too big".to_string()))
}


// Write a VarInt to the given cursor.
// Yoinked from valence: https://github.com/valence-rs/valence/blob/main/crates/valence_protocol/src/var_int.rs#L98
pub async fn write_varint<T>(value: i32, cursor: &mut T) -> crate::prelude::Result<()>
where
    T: Write + Unpin + AsyncWrite
{
    let x = value as u64;
    let stage1 = (x & 0x000000000000007f)
        | ((x & 0x0000000000003f80) << 1)
        | ((x & 0x00000000001fc000) << 2)
        | ((x & 0x000000000fe00000) << 3)
        | ((x & 0x00000000f0000000) << 4);

    let leading = stage1.leading_zeros();

    let unused_bytes = (leading - 1) >> 3;
    let bytes_needed = 8 - unused_bytes;

    // set all but the last MSBs
    let msbs = 0x8080808080808080;
    let msbmask = 0xffffffffffffffff >> (((8 - bytes_needed + 1) << 3) - 1);

    let merged = stage1 | (msbs & msbmask);
    let bytes = merged.to_le_bytes();

    cursor.write_all(unsafe { bytes.get_unchecked(..bytes_needed as usize) })?;

    Ok(())
}