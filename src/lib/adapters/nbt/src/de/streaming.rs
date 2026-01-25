//! Streaming NBT reader for reading NBT values from network streams.
//!
//! This module provides functions to read a single NBT value from a `Read` stream,
//! properly handling the nameless network NBT format used in the Minecraft protocol.
//!
//! Unlike NbtTape (which requires all bytes upfront), these functions read exactly
//! the bytes needed for one NBT value and no more.

use std::io::{self, Read};

// NBT tag type constants (to avoid using the panicking NbtTag::from)
const TAG_END: u8 = 0;
const TAG_BYTE: u8 = 1;
const TAG_SHORT: u8 = 2;
const TAG_INT: u8 = 3;
const TAG_LONG: u8 = 4;
const TAG_FLOAT: u8 = 5;
const TAG_DOUBLE: u8 = 6;
const TAG_BYTE_ARRAY: u8 = 7;
const TAG_STRING: u8 = 8;
const TAG_LIST: u8 = 9;
const TAG_COMPOUND: u8 = 10;
const TAG_INT_ARRAY: u8 = 11;
const TAG_LONG_ARRAY: u8 = 12;

/// Reads a single NBT value from a reader and returns the raw bytes.
///
/// This handles the "nameless" network NBT format where compound tags have no root name.
/// Returns the complete NBT bytes including the tag type byte.
///
/// # Network NBT Format
/// - Tag type (1 byte)
/// - If TAG_End (0): done
/// - Otherwise: payload bytes (no name for root)
pub fn read_nbt_bytes<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut tag_byte = [0u8; 1];
    reader.read_exact(&mut tag_byte)?;
    bytes.push(tag_byte[0]);

    // TAG_End means no value
    if tag_byte[0] == TAG_END {
        return Ok(bytes);
    }

    // For network NBT, the root has no name - just read the payload
    read_payload(reader, tag_byte[0], &mut bytes)?;
    Ok(bytes)
}

/// Reads the payload for an NBT tag (after the tag type byte).
///
/// This function is recursive for compound and list tags.
fn read_payload<R: Read>(reader: &mut R, tag: u8, bytes: &mut Vec<u8>) -> io::Result<()> {
    match tag {
        TAG_END => {
            // End has no payload
        }
        TAG_BYTE => {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf)?;
            bytes.extend_from_slice(&buf);
        }
        TAG_SHORT => {
            let mut buf = [0u8; 2];
            reader.read_exact(&mut buf)?;
            bytes.extend_from_slice(&buf);
        }
        TAG_INT => {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            bytes.extend_from_slice(&buf);
        }
        TAG_LONG => {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;
            bytes.extend_from_slice(&buf);
        }
        TAG_FLOAT => {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            bytes.extend_from_slice(&buf);
        }
        TAG_DOUBLE => {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;
            bytes.extend_from_slice(&buf);
        }
        TAG_BYTE_ARRAY => {
            // 4-byte length (BE i32) followed by that many bytes
            let mut len_buf = [0u8; 4];
            reader.read_exact(&mut len_buf)?;
            bytes.extend_from_slice(&len_buf);
            let len = i32::from_be_bytes(len_buf) as usize;
            let mut data = vec![0u8; len];
            reader.read_exact(&mut data)?;
            bytes.extend_from_slice(&data);
        }
        TAG_STRING => {
            // 2-byte length (BE u16) followed by UTF-8 bytes
            let mut len_buf = [0u8; 2];
            reader.read_exact(&mut len_buf)?;
            bytes.extend_from_slice(&len_buf);
            let len = u16::from_be_bytes(len_buf) as usize;
            let mut data = vec![0u8; len];
            reader.read_exact(&mut data)?;
            bytes.extend_from_slice(&data);
        }
        TAG_LIST => {
            // Element type (1 byte) + count (BE i32) + elements
            let mut type_buf = [0u8; 1];
            reader.read_exact(&mut type_buf)?;
            bytes.push(type_buf[0]);
            let elem_type = type_buf[0];

            let mut len_buf = [0u8; 4];
            reader.read_exact(&mut len_buf)?;
            bytes.extend_from_slice(&len_buf);
            let count = i32::from_be_bytes(len_buf);

            for _ in 0..count {
                read_payload(reader, elem_type, bytes)?;
            }
        }
        TAG_COMPOUND => {
            // Named tags until TAG_End
            loop {
                let mut tag_buf = [0u8; 1];
                reader.read_exact(&mut tag_buf)?;
                bytes.push(tag_buf[0]);

                if tag_buf[0] == TAG_END {
                    break;
                }

                // Read name (string format: u16 length + UTF-8)
                let mut name_len_buf = [0u8; 2];
                reader.read_exact(&mut name_len_buf)?;
                bytes.extend_from_slice(&name_len_buf);
                let name_len = u16::from_be_bytes(name_len_buf) as usize;
                let mut name_data = vec![0u8; name_len];
                reader.read_exact(&mut name_data)?;
                bytes.extend_from_slice(&name_data);

                // Read payload
                read_payload(reader, tag_buf[0], bytes)?;
            }
        }
        TAG_INT_ARRAY => {
            // 4-byte count (BE i32) followed by count * 4 bytes
            let mut len_buf = [0u8; 4];
            reader.read_exact(&mut len_buf)?;
            bytes.extend_from_slice(&len_buf);
            let count = i32::from_be_bytes(len_buf) as usize;
            let mut data = vec![0u8; count * 4];
            reader.read_exact(&mut data)?;
            bytes.extend_from_slice(&data);
        }
        TAG_LONG_ARRAY => {
            // 4-byte count (BE i32) followed by count * 8 bytes
            let mut len_buf = [0u8; 4];
            reader.read_exact(&mut len_buf)?;
            bytes.extend_from_slice(&len_buf);
            let count = i32::from_be_bytes(len_buf) as usize;
            let mut data = vec![0u8; count * 8];
            reader.read_exact(&mut data)?;
            bytes.extend_from_slice(&data);
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid NBT tag type: {} (0x{:02X})", tag, tag),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_end_tag() {
        let data = [0u8]; // TAG_End
        let mut cursor = Cursor::new(&data);
        let result = read_nbt_bytes(&mut cursor).unwrap();
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_read_string() {
        // TAG_String(8) + length(2) + "Hi"
        let data = [8, 0, 2, b'H', b'i'];
        let mut cursor = Cursor::new(&data);
        let result = read_nbt_bytes(&mut cursor).unwrap();
        assert_eq!(result, data.to_vec());
    }

    #[test]
    fn test_read_compound_with_string() {
        // TAG_Compound(10) + TAG_String(8) + name_len(2) + "t" + str_len(2) + "Hi" + TAG_End(0)
        let data = [10, 8, 0, 1, b't', 0, 2, b'H', b'i', 0];
        let mut cursor = Cursor::new(&data);
        let result = read_nbt_bytes(&mut cursor).unwrap();
        assert_eq!(result, data.to_vec());
    }

    #[test]
    fn test_invalid_tag() {
        let data = [15u8]; // Invalid tag
        let mut cursor = Cursor::new(&data);
        let result = read_nbt_bytes(&mut cursor);
        assert!(result.is_err());
    }
}
