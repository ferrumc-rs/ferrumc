use crate::utils::error::Error;

/// Read an arbitrary amount of bits from a i64 at a given position and convert it to a u8.
/// Expects the bits to be in the least significant bits of the i64 (big-endian)
///
/// # Arguments
/// * `bytes` - The i64 to read from
/// * `pos` - The position to start reading from in bits
/// * `n` - The number of bits to read
///
/// # Example
/// ```rs
/// let bytes = 0b1110011;
/// let pos = 0;
/// let n = 5;
/// let result = read_n_bits_u8(bytes, pos, n).unwrap();
/// assert_eq!(result, 28u8);
/// ```
pub fn read_n_bits_u8(bytes: &i64, pos: usize, n: usize) -> Result<u8, Error> {
    if n > 8 {
        return Err(Error::BitOutputOverflow(n, 8));
    }
    if pos + n > 64 {
        return Err(Error::BitReadOverflow(pos + n, 64));
    }
    let mask = (1 << n) - 1;
    Ok(((u64::try_from(*bytes).expect("Failed to convert i64 to u64") >> pos) & mask) as u8)
}

/// Read an arbitrary amount of bits from a i64 at a given position and convert it to a u16.
/// Expects the bits to be in the least significant bits of the i64 (big-endian)
///
/// # Arguments
/// * `bytes` - The i64 to read from
/// * `pos` - The position to start reading from in bits
/// * `n` - The number of bits to read
///
/// # Example
/// ```rs
/// let bytes = 0b100111000100000;
/// let pos = 0;
/// let n = 9;
/// let result = read_n_bits_u16(bytes, pos, n).unwrap();
/// assert_eq!(result, 312u16);
/// ```
pub fn read_n_bits_u16(bytes: &i64, pos: usize, n: usize) -> Result<u16, Error> {
    if n > 16 {
        return Err(Error::BitOutputOverflow(n, 16));
    }
    if pos + n > 64 {
        return Err(Error::BitReadOverflow(pos + n, 64));
    }
    let mask = (1 << n) - 1;
    Ok(((u64::try_from(*bytes).expect("Failed to convert i64 to u64") >> pos) & mask) as u16)
}

/// Read an arbitrary amount of bits from a i64 at a given position and convert it to a u32.
/// Expects the bits to be in the least significant bits of the i64 (big-endian)
///
/// # Arguments
/// * `bytes` - The i64 to read from
/// * `pos` - The position to start reading from in bits
/// * `n` - The number of bits to read
///
/// # Example
/// ```rs
/// let bytes = 0b100111110001111001010100101011;
/// let pos = 2;
/// let n = 28;
/// let result = read_n_bits_u32(bytes, pos, n).unwrap();
/// assert_eq!(result, 32630090u32);
/// ```
pub fn read_n_bits_u32(bytes: &i64, pos: usize, n: usize) -> Result<u32, Error> {
    if n > 32 {
        return Err(Error::BitOutputOverflow(n, 32));
    }
    if pos + n > 64 {
        return Err(Error::BitReadOverflow(pos + n, 64));
    }
    let mask = (1 << n) - 1;
    Ok(((u64::try_from(*bytes).expect("Failed to convert i64 to u64") >> pos) & mask) as u32)
}
