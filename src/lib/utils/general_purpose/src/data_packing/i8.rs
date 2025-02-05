use crate::data_packing::errors::DataPackingError;

/// Reads a specified number of bits from a given offset in a 64-bit signed integer.
///
/// # Arguments
///
/// * `data` - A reference to the 64-bit signed integer to read from.
/// * `size` - The number of bits to read (must be 8 or less).
/// * `offset` - The bit offset from which to start reading.
///
/// # Returns
///
/// * `Ok(i8)` - The extracted bits as an 8-bit signed integer.
/// * `Err(DataPackingError)` - If the size exceeds 8 bits or the offset plus size exceeds 64 bits.
///
/// # Errors
///
/// * `DataPackingError::SizeExceedsMaxSize` - If `size` is greater than 8.
/// * `DataPackingError::NotEnoughBits` - If `offset + size` exceeds 64 bits.
pub fn read_nbit_i8(data: &i64, size: u8, offset: u32) -> Result<i8, DataPackingError> {
    if size > 8 {
        return Err(DataPackingError::SizeExceedsMaxSize(size, 8));
    }
    if offset + size as u32 > 64 {
        return Err(DataPackingError::NotEnoughBits(size, offset));
    }
    let mask = (1 << size) - 1;
    let extracted_bits = ((data >> offset) & mask) as i8;
    // Sign extend if the extracted bits represent a negative number
    let sign_bit = 1 << (size - 1);
    if extracted_bits & sign_bit != 0 {
        Ok(extracted_bits | !mask as i8)
    } else {
        Ok(extracted_bits)
    }
}

/// Writes a specified number of bits to a given offset in a 64-bit signed integer.
///
/// # Arguments
///
/// * `data` - A mutable reference to the 64-bit signed integer to write to.
/// * `offset` - The bit offset from which to start writing.
/// * `value` - The 8-bit signed integer value to write.
/// * `size` - The number of bits to write (must be 8 or less).
///
/// # Returns
///
/// * `Ok(())` - If the bits were successfully written.
/// * `Err(DataPackingError)` - If the size exceeds 8 bits or the offset plus size exceeds 64 bits.
///
/// # Errors
///
/// * `DataPackingError::SizeExceedsMaxSize` - If `size` is greater than 8.
/// * `DataPackingError::NotEnoughBits` - If `offset + size` exceeds 64 bits.
pub fn write_nbit_i8(
    data: &mut i64,
    offset: u32,
    value: i8,
    size: u8,
) -> Result<(), DataPackingError> {
    if size > 8 {
        return Err(DataPackingError::SizeExceedsMaxSize(size, 8));
    }
    if offset + size as u32 > 64 {
        return Err(DataPackingError::NotEnoughBits(size, offset));
    }
    let mask = (1 << size) - 1;
    *data &= !((mask) << offset);
    *data |= ((value as i64) & mask) << offset;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `write_nbit_i8` function with various inputs.
    #[test]
    fn test_write_nbit_i8() {
        let mut data: i64 = 0;
        write_nbit_i8(&mut data, 0, 0b011, 3).unwrap();
        assert_eq!(data, 0b011);
        write_nbit_i8(&mut data, 3, -3, 3).unwrap(); // 0b101 as i8 is -3
        assert_eq!(data, 0b101011);
        write_nbit_i8(&mut data, 6, -2, 3).unwrap(); // 0b110 as i8 is -2
        assert_eq!(data, 0b110101011);
        write_nbit_i8(&mut data, 9, 0b000, 3).unwrap();
        assert_eq!(data, 0b110101011);
    }

    /// Tests the `read_nbit_i8` function with various inputs.
    #[test]
    fn test_read_nbit_i8() {
        let data: i64 = 0b110101011;
        assert_eq!(read_nbit_i8(&data, 3, 0).unwrap(), 0b011);
        assert_eq!(read_nbit_i8(&data, 3, 3).unwrap(), -3); // 0b101 as i8 is -3
        assert_eq!(read_nbit_i8(&data, 3, 6).unwrap(), -2); // 0b110 as i8 is -2
        assert_eq!(read_nbit_i8(&data, 3, 9).unwrap(), 0b000);
    }
}
