use std::{mem, simd::prelude::*};

mod private {
    pub trait Sealed {}

    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}
pub trait SwappableNumber: private::Sealed {}
impl<T: private::Sealed> SwappableNumber for T {}

#[inline]
fn swap_endianness_16bit(bytes: &mut [u8], num: usize) {
    for i in 0..num / 32 {
        let simd: u8x64 = Simd::from_slice(bytes[i * 32 * 2..(i + 1) * 32 * 2].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            1, 0,
            3, 2,
            5, 4,
            7, 6,
            9, 8,
            11, 10,
            13, 12,
            15, 14,
            17, 16,
            19, 18,
            21, 20,
            23, 22,
            25, 24,
            27, 26,
            29, 28,
            31, 30,
            33, 32,
            35, 34,
            37, 36,
            39, 38,
            41, 40,
            43, 42,
            45, 44,
            47, 46,
            49, 48,
            51, 50,
            53, 52,
            55, 54,
            57, 56,
            59, 58,
            61, 60,
            63, 62,
        ]);
        bytes[i * 32 * 2..(i + 1) * 32 * 2].copy_from_slice(simd.as_array());
    }

    let mut i = num / 32 * 32;
    if i + 16 <= num {
        let simd: u8x32 = Simd::from_slice(bytes[i * 2..i * 2 + 32].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            1, 0,
            3, 2,
            5, 4,
            7, 6,
            9, 8,
            11, 10,
            13, 12,
            15, 14,
            17, 16,
            19, 18,
            21, 20,
            23, 22,
            25, 24,
            27, 26,
            29, 28,
            31, 30,
        ]);
        bytes[i * 2..i * 2 + 32].copy_from_slice(simd.as_array());
        i += 16;
    }
    if i + 8 <= num {
        let simd: u8x16 = Simd::from_slice(bytes[i * 2..i * 2 + 16].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            1, 0,
            3, 2,
            5, 4,
            7, 6,
            9, 8,
            11, 10,
            13, 12,
            15, 14,
        ]);
        bytes[i * 2..i * 2 + 16].copy_from_slice(simd.as_array());
        i += 8;
    }
    if i + 4 <= num {
        let simd: u8x8 = Simd::from_slice(bytes[i * 2..i * 2 + 8].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            1, 0,
            3, 2,
            5, 4,
            7, 6,
        ]);
        bytes[i * 2..i * 2 + 8].copy_from_slice(simd.as_array());
        i += 4;
    }
    if i + 2 <= num {
        let simd: u8x4 = Simd::from_slice(bytes[i * 2..i * 2 + 4].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            1, 0,
            3, 2,
        ]);
        bytes[i * 2..i * 2 + 4].copy_from_slice(simd.as_array());
        i += 2;
    }
    if i < num {
        let simd: u8x2 = Simd::from_slice(bytes[i * 2..i * 2 + 2].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            1, 0,
        ]);
        bytes[i * 2..i * 2 + 2].copy_from_slice(simd.as_array());
    }
}

#[inline]
fn swap_endianness_32bit(bytes: &mut [u8], num: usize) {
    for i in 0..num / 16 {
        let simd: u8x64 = Simd::from_slice(bytes[i * 16 * 4..(i + 1) * 16 * 4].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            3, 2, 1, 0,
            7, 6, 5, 4,
            11, 10, 9, 8,
            15, 14, 13, 12,
            19, 18, 17, 16,
            23, 22, 21, 20,
            27, 26, 25, 24,
            31, 30, 29, 28,
            35, 34, 33, 32,
            39, 38, 37, 36,
            43, 42, 41, 40,
            47, 46, 45, 44,
            51, 50, 49, 48,
            55, 54, 53, 52,
            59, 58, 57, 56,
            63, 62, 61, 60,
        ]);
        bytes[i * 16 * 4..(i + 1) * 16 * 4].copy_from_slice(simd.as_array());
    }

    let mut i = num / 16 * 16;
    if i + 8 <= num {
        let simd: u8x32 = Simd::from_slice(bytes[i * 4..i * 4 + 32].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            3, 2, 1, 0,
            7, 6, 5, 4,
            11, 10, 9, 8,
            15, 14, 13, 12,
            19, 18, 17, 16,
            23, 22, 21, 20,
            27, 26, 25, 24,
            31, 30, 29, 28,
        ]);
        bytes[i * 4..i * 4 + 32].copy_from_slice(simd.as_array());
        i += 8;
    }
    if i + 4 <= num {
        let simd: u8x16 = Simd::from_slice(bytes[i * 4..i * 4 + 16].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            3, 2, 1, 0,
            7, 6, 5, 4,
            11, 10, 9, 8,
            15, 14, 13, 12,
        ]);
        bytes[i * 4..i * 4 + 16].copy_from_slice(simd.as_array());
        i += 4;
    }
    if i + 2 <= num {
        let simd: u8x8 = Simd::from_slice(bytes[i * 4..i * 4 + 8].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            3, 2, 1, 0,
            7, 6, 5, 4,
        ]);
        bytes[i * 4..i * 4 + 8].copy_from_slice(simd.as_array());
        i += 2;
    }
    if i < num {
        let simd: u8x4 = Simd::from_slice(bytes[i * 4..i * 4 + 4].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            3, 2, 1, 0,
        ]);
        bytes[i * 4..i * 4 + 4].copy_from_slice(simd.as_array());
    }
}

#[inline]
fn swap_endianness_64bit(bytes: &mut [u8], num: usize) {
    for i in 0..num / 8 {
        let simd: u8x64 = Simd::from_slice(bytes[i * 64..i * 64 + 64].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            7, 6, 5, 4, 3, 2, 1, 0,
            15, 14, 13, 12, 11, 10, 9, 8,
            23, 22, 21, 20, 19, 18, 17, 16,
            31, 30, 29, 28, 27, 26, 25, 24,
            39, 38, 37, 36, 35, 34, 33, 32,
            47, 46, 45, 44, 43, 42, 41, 40,
            55, 54, 53, 52, 51, 50, 49, 48,
            63, 62, 61, 60, 59, 58, 57, 56,
        ]);
        bytes[i * 64..i * 64 + 64].copy_from_slice(simd.as_array());
    }

    let mut i = num / 8 * 8;
    if i + 4 <= num {
        let simd: u8x32 = Simd::from_slice(bytes[i * 8..i * 8 + 32].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            7, 6, 5, 4, 3, 2, 1, 0,
            15, 14, 13, 12, 11, 10, 9, 8,
            23, 22, 21, 20, 19, 18, 17, 16,
            31, 30, 29, 28, 27, 26, 25, 24,
        ]);
        bytes[i * 8..i * 8 + 32].copy_from_slice(simd.as_array());
        i += 4;
    }
    if i + 2 <= num {
        let simd: u8x16 = Simd::from_slice(bytes[i * 8..i * 8 + 16].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            7, 6, 5, 4, 3, 2, 1, 0,
            15, 14, 13, 12, 11, 10, 9, 8,
        ]);
        bytes[i * 8..i * 8 + 16].copy_from_slice(simd.as_array());
        i += 2;
    }
    if i < num {
        let simd: u8x8 = Simd::from_slice(bytes[i * 8..i * 8 + 8].as_ref());
        #[rustfmt::skip]
        let simd = simd_swizzle!(simd, [
            7, 6, 5, 4, 3, 2, 1, 0,
        ]);
        bytes[i * 8..i * 8 + 8].copy_from_slice(simd.as_array());
    }
}

/// Swap the endianness of the given array (unless we're on a big-endian system) in-place depending
/// on the width of the given type.
fn swap_endianness_from_type<T: SwappableNumber>(items: &mut [u8]) {
    let item_width = mem::size_of::<T>();
    let length = items.len() / item_width;

    if cfg!(target_endian = "little") {
        match item_width {
            2 => swap_endianness_16bit(items, length),
            4 => swap_endianness_32bit(items, length),
            8 => swap_endianness_64bit(items, length),
            _ => panic!("unsupported size of type"),
        }
    }
}

/// Swaps the endianness of the given data and return it as a `Vec<u8>`.
#[inline]
pub fn swap_endianness_as_u8<T: SwappableNumber>(data: &[u8]) -> Vec<u8> {
    let mut items = data.to_vec();
    swap_endianness_from_type::<T>(&mut items);

    items
}

#[inline]
pub fn swap_endianness<T: SwappableNumber>(data: &[u8]) -> Vec<T> {
    let width_of_t = mem::size_of::<T>();
    let length_of_vec_t = data.len() / width_of_t;

    // the data must be a multiple of the item width, otherwise it's UB
    assert_eq!(data.len() % width_of_t, 0);

    // have the vec be of T initially so it's aligned
    let mut vec_t = Vec::<T>::with_capacity(length_of_vec_t);
    let mut vec_u8: Vec<u8> = {
        let ptr = vec_t.as_mut_ptr() as *mut u8;
        mem::forget(vec_t);
        // SAFETY: the new capacity is correct since we checked that data.len() is a multiple of width_of_t
        unsafe { Vec::from_raw_parts(ptr, 0, data.len()) }
    };
    vec_u8.extend_from_slice(data);

    swap_endianness_from_type::<T>(&mut vec_u8);

    // now convert our Vec<u8> back to Vec<T>

    let ptr = vec_u8.as_mut_ptr() as *mut T;
    mem::forget(vec_u8);
    // SAFETY: The length won't be greater than the length of the original data
    unsafe { Vec::from_raw_parts(ptr, length_of_vec_t, length_of_vec_t) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_endianness_u16() {
        assert_eq!(
            swap_endianness_as_u8::<u16>(&[1, 2, 3, 4, 5, 6, 7, 8]),
            [2, 1, 4, 3, 6, 5, 8, 7]
        );
    }
    #[test]
    fn test_swap_endianness_u32() {
        assert_eq!(
            swap_endianness_as_u8::<u32>(&[1, 2, 3, 4, 5, 6, 7, 8]),
            [4, 3, 2, 1, 8, 7, 6, 5]
        );
    }
    #[test]
    fn test_swap_endianness_u64() {
        assert_eq!(
            swap_endianness_as_u8::<u64>(&[1, 2, 3, 4, 5, 6, 7, 8]),
            [8, 7, 6, 5, 4, 3, 2, 1]
        );
    }

    #[test]
    fn test_swap_endianness_u64_vec() {
        assert_eq!(
            swap_endianness::<u64>(&[1, 2, 3, 4, 5, 6, 7, 8, 8, 7, 6, 5, 4, 3, 2, 1]),
            vec![
                u64::from_le_bytes([8, 7, 6, 5, 4, 3, 2, 1]),
                u64::from_le_bytes([1, 2, 3, 4, 5, 6, 7, 8])
            ]
        );
    }
}
