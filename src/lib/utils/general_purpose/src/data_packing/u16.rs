use crate::data_packing::errors::DataPackingError;

/// Reads a specified number of bits from a given offset in a 64-bit unsigned integer.
///
/// # Arguments
///
/// * `data` - A reference to the 64-bit unsigned integer to read from.
/// * `size` - The number of bits to read (must be 16 or less).
/// * `offset` - The bit offset from which to start reading.
///
/// # Returns
///
/// * `Ok(u16)` - The extracted bits as a 16-bit unsigned integer.
/// * `Err(DataPackingError)` - If the size exceeds 16 bits or the offset plus size exceeds 64 bits.
///
/// # Errors
///
/// * `DataPackingError::SizeExceedsMaxSize` - If `size` is greater than 16.
/// * `DataPackingError::NotEnoughBits` - If `offset + size` exceeds 64 bits.
pub fn read_nbit_u16(data: &u64, size: u8, offset: u32) -> Result<u16, DataPackingError> {
    if size > 16 {
        return Err(DataPackingError::SizeExceedsMaxSize(size, 16));
    }
    if offset + size as u32 > 64 {
        return Err(DataPackingError::NotEnoughBits(size, offset));
    }
    Ok(((data >> offset) & ((1 << size) - 1)) as u16)
}

/// Writes a specified number of bits to a given offset in a 64-bit unsigned integer.
///
/// # Arguments
///
/// * `data` - A mutable reference to the 64-bit unsigned integer to write to.
/// * `offset` - The bit offset from which to start writing.
/// * `value` - The 16-bit unsigned integer value to write.
/// * `size` - The number of bits to write (must be 16 or less).
///
/// # Returns
///
/// * `Ok(())` - If the bits were successfully written.
/// * `Err(DataPackingError)` - If the size exceeds 16 bits or the offset plus size exceeds 64 bits.
///
/// # Errors
///
/// * `DataPackingError::SizeExceedsMaxSize` - If `size` is greater than 16.
/// * `DataPackingError::NotEnoughBits` - If `offset + size` exceeds 64 bits.
pub fn write_nbit_u16(
    data: &mut u64,
    offset: u32,
    value: u16,
    size: u8,
) -> Result<(), DataPackingError> {
    if size > 16 {
        return Err(DataPackingError::SizeExceedsMaxSize(size, 16));
    }
    if offset + size as u32 > 64 {
        return Err(DataPackingError::NotEnoughBits(size, offset));
    }
    let mask = (1 << size) - 1;
    *data &= !((mask) << offset);
    *data |= ((value as u64) & mask) << offset;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `read_nbit_u16` function with various inputs.
    #[test]
    fn test_read_nbit_u16() {
        let data: u64 = 0b110101011;
        assert_eq!(read_nbit_u16(&data, 3, 0).unwrap(), 0b011);
        assert_eq!(read_nbit_u16(&data, 3, 3).unwrap(), 0b101);
        assert_eq!(read_nbit_u16(&data, 3, 6).unwrap(), 0b110);
        assert_eq!(read_nbit_u16(&data, 3, 9).unwrap(), 0b000);
    }

    /// Tests the `write_nbit_u16` function with various inputs.
    #[test]
    fn test_write_nbit_u16() {
        let mut data: u64 = 0;
        write_nbit_u16(&mut data, 0, 0b011, 3).unwrap();
        assert_eq!(data, 0b011);
        write_nbit_u16(&mut data, 3, 0b101, 3).unwrap();
        assert_eq!(data, 0b101011);
        write_nbit_u16(&mut data, 6, 0b110, 3).unwrap();
        assert_eq!(data, 0b110101011);
        write_nbit_u16(&mut data, 9, 0b000, 3).unwrap();
        assert_eq!(data, 0b110101011);
    }
}
