//! Streaming NBT reader for reading NBT values from network streams.
//!
//! This module provides functions to read a single NBT value from a `Read` stream,
//! properly handling the nameless network NBT format used in the Minecraft protocol.
//!
//! Unlike NbtTape (which requires all bytes upfront), these functions read exactly
//! the bytes needed for one NBT value and no more.
//!
//! # Security
//!
//! This reader includes protections against malicious NBT data:
//! - Size limits prevent memory exhaustion from oversized allocations
//! - Depth limits prevent stack overflow from deeply nested structures
//! - Negative length validation prevents integer overflow attacks

use crate::limits::{MAX_NBT_DEPTH, MAX_NBT_SIZE};
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

/// Tracks state during NBT reading for security validation.
struct NbtReadState {
    /// Total bytes read so far
    bytes_read: usize,
    /// Current nesting depth (compound/list)
    depth: usize,
}

impl NbtReadState {
    fn new() -> Self {
        Self {
            bytes_read: 0,
            depth: 0,
        }
    }

    /// Validates and adds to bytes_read, returning error if limit exceeded.
    fn add_bytes(&mut self, count: usize) -> io::Result<()> {
        self.bytes_read = self.bytes_read.saturating_add(count);
        if self.bytes_read > MAX_NBT_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "NBT size {} exceeds maximum {}",
                    self.bytes_read, MAX_NBT_SIZE
                ),
            ));
        }
        Ok(())
    }

    /// Increments depth and validates limit.
    fn push_depth(&mut self) -> io::Result<()> {
        self.depth += 1;
        if self.depth > MAX_NBT_DEPTH {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("NBT depth {} exceeds maximum {}", self.depth, MAX_NBT_DEPTH),
            ));
        }
        Ok(())
    }

    /// Decrements depth.
    fn pop_depth(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }
}

/// Validates that a length is non-negative and converts to usize safely.
fn validate_length(len: i32, context: &str) -> io::Result<usize> {
    if len < 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Negative {} length: {}", context, len),
        ));
    }
    Ok(len as usize)
}

/// Reads a single NBT value from a reader and returns the raw bytes.
///
/// This handles the "nameless" network NBT format where compound tags have no root name.
/// Returns the complete NBT bytes including the tag type byte.
///
/// # Network NBT Format
/// - Tag type (1 byte)
/// - If TAG_End (0): done
/// - Otherwise: payload bytes (no name for root)
///
/// # Security
///
/// This function enforces size and depth limits to prevent:
/// - Memory exhaustion from maliciously large length fields
/// - Stack overflow from deeply nested structures
pub fn read_nbt_bytes<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut state = NbtReadState::new();

    let mut tag_byte = [0u8; 1];
    reader.read_exact(&mut tag_byte)?;
    bytes.push(tag_byte[0]);
    state.add_bytes(1)?;

    // TAG_End means no value
    if tag_byte[0] == TAG_END {
        return Ok(bytes);
    }

    // For network NBT, the root has no name - just read the payload
    read_payload(reader, tag_byte[0], &mut bytes, &mut state)?;
    Ok(bytes)
}

/// Reads the payload for an NBT tag (after the tag type byte).
///
/// This function is recursive for compound and list tags.
fn read_payload<R: Read>(
    reader: &mut R,
    tag: u8,
    bytes: &mut Vec<u8>,
    state: &mut NbtReadState,
) -> io::Result<()> {
    match tag {
        TAG_END => {
            // End has no payload
        }
        TAG_BYTE => {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf)?;
            bytes.extend_from_slice(&buf);
            state.add_bytes(1)?;
        }
        TAG_SHORT => {
            let mut buf = [0u8; 2];
            reader.read_exact(&mut buf)?;
            bytes.extend_from_slice(&buf);
            state.add_bytes(2)?;
        }
        TAG_INT => {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            bytes.extend_from_slice(&buf);
            state.add_bytes(4)?;
        }
        TAG_LONG => {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;
            bytes.extend_from_slice(&buf);
            state.add_bytes(8)?;
        }
        TAG_FLOAT => {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            bytes.extend_from_slice(&buf);
            state.add_bytes(4)?;
        }
        TAG_DOUBLE => {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;
            bytes.extend_from_slice(&buf);
            state.add_bytes(8)?;
        }
        TAG_BYTE_ARRAY => {
            // 4-byte length (BE i32) followed by that many bytes
            let mut len_buf = [0u8; 4];
            reader.read_exact(&mut len_buf)?;
            bytes.extend_from_slice(&len_buf);
            state.add_bytes(4)?;

            let len_i32 = i32::from_be_bytes(len_buf);
            let len = validate_length(len_i32, "byte array")?;

            // Check size limit before allocation
            state.add_bytes(len)?;

            let mut data = vec![0u8; len];
            reader.read_exact(&mut data)?;
            bytes.extend_from_slice(&data);
        }
        TAG_STRING => {
            // 2-byte length (BE u16) followed by UTF-8 bytes
            let mut len_buf = [0u8; 2];
            reader.read_exact(&mut len_buf)?;
            bytes.extend_from_slice(&len_buf);
            state.add_bytes(2)?;

            let len = u16::from_be_bytes(len_buf) as usize;

            // Check size limit before allocation
            state.add_bytes(len)?;

            let mut data = vec![0u8; len];
            reader.read_exact(&mut data)?;
            bytes.extend_from_slice(&data);
        }
        TAG_LIST => {
            // Push depth for list recursion
            state.push_depth()?;

            // Element type (1 byte) + count (BE i32) + elements
            let mut type_buf = [0u8; 1];
            reader.read_exact(&mut type_buf)?;
            bytes.push(type_buf[0]);
            state.add_bytes(1)?;
            let elem_type = type_buf[0];

            let mut len_buf = [0u8; 4];
            reader.read_exact(&mut len_buf)?;
            bytes.extend_from_slice(&len_buf);
            state.add_bytes(4)?;

            let count_i32 = i32::from_be_bytes(len_buf);
            let count = validate_length(count_i32, "list")?;

            for _ in 0..count {
                read_payload(reader, elem_type, bytes, state)?;
            }

            state.pop_depth();
        }
        TAG_COMPOUND => {
            // Push depth for compound recursion
            state.push_depth()?;

            // Named tags until TAG_End
            loop {
                let mut tag_buf = [0u8; 1];
                reader.read_exact(&mut tag_buf)?;
                bytes.push(tag_buf[0]);
                state.add_bytes(1)?;

                if tag_buf[0] == TAG_END {
                    break;
                }

                // Read name (string format: u16 length + UTF-8)
                let mut name_len_buf = [0u8; 2];
                reader.read_exact(&mut name_len_buf)?;
                bytes.extend_from_slice(&name_len_buf);
                state.add_bytes(2)?;

                let name_len = u16::from_be_bytes(name_len_buf) as usize;

                // Check size limit before allocation
                state.add_bytes(name_len)?;

                let mut name_data = vec![0u8; name_len];
                reader.read_exact(&mut name_data)?;
                bytes.extend_from_slice(&name_data);

                // Read payload
                read_payload(reader, tag_buf[0], bytes, state)?;
            }

            state.pop_depth();
        }
        TAG_INT_ARRAY => {
            // 4-byte count (BE i32) followed by count * 4 bytes
            let mut len_buf = [0u8; 4];
            reader.read_exact(&mut len_buf)?;
            bytes.extend_from_slice(&len_buf);
            state.add_bytes(4)?;

            let count_i32 = i32::from_be_bytes(len_buf);
            let count = validate_length(count_i32, "int array")?;

            // Check size limit before allocation (count * 4 bytes)
            let data_size = count.checked_mul(4).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Int array size overflow")
            })?;
            state.add_bytes(data_size)?;

            let mut data = vec![0u8; data_size];
            reader.read_exact(&mut data)?;
            bytes.extend_from_slice(&data);
        }
        TAG_LONG_ARRAY => {
            // 4-byte count (BE i32) followed by count * 8 bytes
            let mut len_buf = [0u8; 4];
            reader.read_exact(&mut len_buf)?;
            bytes.extend_from_slice(&len_buf);
            state.add_bytes(4)?;

            let count_i32 = i32::from_be_bytes(len_buf);
            let count = validate_length(count_i32, "long array")?;

            // Check size limit before allocation (count * 8 bytes)
            let data_size = count.checked_mul(8).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Long array size overflow")
            })?;
            state.add_bytes(data_size)?;

            let mut data = vec![0u8; data_size];
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

    #[test]
    fn test_negative_byte_array_length() {
        // TAG_ByteArray(7) + negative length (-1 in BE)
        let data = [7u8, 0xFF, 0xFF, 0xFF, 0xFF];
        let mut cursor = Cursor::new(&data);
        let result = read_nbt_bytes(&mut cursor);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Negative byte array length"));
    }

    #[test]
    fn test_negative_list_length() {
        // TAG_List(9) + element_type(1) + negative length (-1 in BE)
        let data = [9u8, 1, 0xFF, 0xFF, 0xFF, 0xFF];
        let mut cursor = Cursor::new(&data);
        let result = read_nbt_bytes(&mut cursor);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Negative list length"));
    }

    #[test]
    fn test_size_limit_exceeded() {
        // TAG_ByteArray(7) + length exceeding MAX_NBT_SIZE
        let huge_len = (MAX_NBT_SIZE + 1) as i32;
        let len_bytes = huge_len.to_be_bytes();
        let data = [7u8, len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]];
        let mut cursor = Cursor::new(&data);
        let result = read_nbt_bytes(&mut cursor);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds maximum"));
    }

    #[test]
    fn test_depth_limit() {
        // Create deeply nested compounds (MAX_NBT_DEPTH + 1 levels)
        let mut data = Vec::new();

        // Open MAX_NBT_DEPTH + 1 compounds
        for i in 0..=MAX_NBT_DEPTH {
            data.push(TAG_COMPOUND);
            if i > 0 {
                // Add name for nested compounds
                data.extend_from_slice(&[0, 1, b'a']); // name "a"
            }
        }

        let mut cursor = Cursor::new(&data);
        let result = read_nbt_bytes(&mut cursor);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("depth"));
    }
}
