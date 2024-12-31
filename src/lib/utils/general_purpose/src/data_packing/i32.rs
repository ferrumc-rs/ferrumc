use crate::data_packing::errors::DataPackingError;

/// Reads a specified number of bits from a given offset in a 64-bit signed integer.
///
/// # Arguments
///
/// * `data` - A reference to the 64-bit signed integer to read from.
/// * `size` - The number of bits to read (must be 32 or less).
/// * `offset` - The bit offset from which to start reading.
///
/// # Returns
///
/// * `Ok(i32)` - The extracted bits as a 32-bit signed integer.
/// * `Err(DataPackingError)` - If the size exceeds 32 bits or the offset plus size exceeds 64 bits.
///
/// # Errors
///
/// * `DataPackingError::SizeExceedsMaxSize` - If `size` is greater than 32.
/// * `DataPackingError::NotEnoughBits` - If `offset + size` exceeds 64 bits.
///    Reads an n-bit integer from a packed `i64`.
pub fn read_nbit_i32(
    word: &i64,
    bit_size: usize,
    bit_offset: u32,
) -> Result<i32, DataPackingError> {
    if bit_size == 0 || bit_size > 32 {
        return Err(DataPackingError::SizeExceedsMaxSize(bit_size as u8, 32));
    }
    if bit_offset >= 64 {
        return Err(DataPackingError::SizeExceedsMaxSize(bit_size as u8, 32));
    }
    if bit_offset + bit_size as u32 > 64 {
        return Err(DataPackingError::NotEnoughBits(bit_size as u8, bit_offset));
    }

    // Create a mask for the n-bit value
    let mask = (1u64 << bit_size) - 1;

    // Extract the value from the word
    let value = ((*word as u64) >> bit_offset) & mask;

    // Cast to i32 and return
    Ok(value as i32)
}

/// Writes a specified number of bits to a given offset in a 64-bit signed integer.
///
/// # Arguments
///
/// * `data` - A mutable reference to the 64-bit signed integer to write to.
/// * `offset` - The bit offset from which to start writing.
/// * `value` - The 32-bit signed integer value to write.
/// * `size` - The number of bits to write (must be 32 or less).
///
/// # Returns
///
/// * `Ok(())` - If the bits were successfully written.
/// * `Err(DataPackingError)` - If the size exceeds 32 bits or the offset plus size exceeds 64 bits.
///
/// # Errors
///
/// * `DataPackingError::SizeExceedsMaxSize` - If `size` is greater than 32.
/// * `DataPackingError::NotEnoughBits` - If `offset + size` exceeds 64 bits.
pub fn write_nbit_i32(
    data: &mut i64,
    offset: u32,
    value: i32,
    size: u8,
) -> Result<(), DataPackingError> {
    if size > 32 {
        return Err(DataPackingError::SizeExceedsMaxSize(size, 32));
    }
    if offset + size as u32 > 64 {
        return Err(DataPackingError::NotEnoughBits(size, offset));
    }
    let mask = (1 << size) - 1;
    *data &= !(mask << offset);
    *data |= ((value as i64) & mask) << offset;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `read_nbit_i32` function with various inputs.
    #[test]
    fn test_read_nbit_i32() {
        let data: i64 = 0b110101011;
        assert_eq!(read_nbit_i32(&data, 3, 0).unwrap(), 0b011);
        assert_eq!(read_nbit_i32(&data, 3, 3).unwrap(), 0b101);
        assert_eq!(read_nbit_i32(&data, 3, 6).unwrap(), 0b110);
        assert_eq!(read_nbit_i32(&data, 3, 9).unwrap(), 0b000);
    }

    /// Tests the `write_nbit_i32` function with various inputs.
    #[test]
    fn test_write_nbit_i32() {
        let mut data: i64 = 0;
        write_nbit_i32(&mut data, 0, 0b011, 3).unwrap();
        assert_eq!(data, 0b011);
        write_nbit_i32(&mut data, 3, -3, 3).unwrap(); // 0b101 as i32 is -3
        assert_eq!(data, 0b101011);
        write_nbit_i32(&mut data, 6, -2, 3).unwrap(); // 0b110 as i32 is -2
        assert_eq!(data, 0b110101011);
        write_nbit_i32(&mut data, 9, 0b000, 3).unwrap();
        assert_eq!(data, 0b110101011);
    }
}
