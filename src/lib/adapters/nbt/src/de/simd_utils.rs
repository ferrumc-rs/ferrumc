use std::arch::x86_64::*;
use std::slice;

pub fn u8_slice_to_u8(input: &[u8]) -> &[u8] {
    input
}

pub fn u8_slice_to_i8(input: &[u8]) -> &[i8] {
    unsafe { slice::from_raw_parts(input.as_ptr() as *const i8, input.len()) }
}

#[target_feature(enable = "avx2")]
pub unsafe fn u8_slice_to_u32_be(input: &[u8]) -> Vec<u32> {
    debug_assert_eq!(
        input.len() % 4,
        0,
        "Input length must be a multiple of 4 for u32 conversion"
    );

    let num_elements = input.len() / 4;
    let mut output = Vec::with_capacity(num_elements);
    let mut i = 0;

    unsafe {
        let shuffle_mask = _mm256_setr_epi8(
            3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20,
            27, 26, 25, 24, 31, 30, 29, 28,
        );

        // Process 32 bytes (8 u32s) at a time
        while i + 32 <= input.len() {
            // Load 32 bytes into a 256-bit register AVX2 register
            let data = _mm256_loadu_si256(input.as_ptr().add(i) as *const __m256i);

            // Shuffle bytes to reverse within each u32
            let shuffled = _mm256_shuffle_epi8(data, shuffle_mask);

            // Store the shuffled bytes into a temporary buffer
            let mut temp = [0u8; 32];
            _mm256_storeu_si256(temp.as_mut_ptr() as *mut __m256i, shuffled);

            // Convert each reversed u32 to host endianness
            for j in 0..8 {
                let bytes: [u8; 4] = temp[j * 4..(j + 1) * 4]
                    .try_into()
                    .expect("simd: failed to convert bytes");
                let val = u32::from_le_bytes(bytes);
                output.push(val);
            }

            i += 32;
        }

        // Handle remaining bytes with scalar code
        while i + 4 <= input.len() {
            let bytes: [u8; 4] = input[i..i + 4]
                .try_into()
                .expect("simd: failed to convert bytes");
            let val = u32::from_be_bytes(bytes);
            output.push(val);
            i += 4;
        }
    }

    output
}

pub fn u8_slice_to_i32_be(input: &[u8]) -> Vec<i32> {
    let u32s = unsafe { u8_slice_to_u32_be(input) };

    unsafe { std::mem::transmute::<Vec<u32>, Vec<i32>>(u32s) }
}

#[target_feature(enable = "avx2")]
pub unsafe fn u8_slice_to_u64_be(input: &[u8]) -> Vec<u64> {
    debug_assert_eq!(
        input.len() % 8,
        0,
        "Input length must be a multiple of 8 for u64 conversion"
    );

    let num_elements = input.len() / 8;
    let mut output = Vec::with_capacity(num_elements);
    let mut i = 0;

    unsafe {
        // Shuffle mask to reverse bytes in each 8-byte word
        let shuffle_mask = _mm256_setr_epi8(
            7, 6, 5, 4, 3, 2, 1, 0, // Reverse first u64
            15, 14, 13, 12, 11, 10, 9, 8, // Reverse second u64
            23, 22, 21, 20, 19, 18, 17, 16, // Reverse third u64
            31, 30, 29, 28, 27, 26, 25, 24, // Reverse fourth u64
        );

        // Process 32 bytes (4 u64s) at a time
        while i + 32 <= input.len() {
            // Load 32 bytes into a 256-bit AVX2 register
            let data = _mm256_loadu_si256(input.as_ptr().add(i) as *const __m256i);

            // Shuffle bytes to reverse within each u64
            let shuffled = _mm256_shuffle_epi8(data, shuffle_mask);

            // Store the shuffled bytes into a temporary array
            let mut temp = [0u8; 32];
            _mm256_storeu_si256(temp.as_mut_ptr() as *mut __m256i, shuffled);

            // Convert each reversed u64 to host endianness
            for j in 0..4 {
                let bytes: [u8; 8] = temp[j * 8..(j + 1) * 8].try_into().unwrap();
                let val = u64::from_le_bytes(bytes);
                output.push(val);
            }

            i += 32;
        }

        // Handle remaining bytes with scalar code
        while i + 8 <= input.len() {
            let bytes: [u8; 8] = input[i..i + 8].try_into().unwrap();
            let val = u64::from_be_bytes(bytes);
            output.push(val);
            i += 8;
        }
    }

    output
}

pub fn u8_slice_to_i64_be(input: &[u8]) -> Vec<i64> {
    let u64s = unsafe { u8_slice_to_u64_be(input) };

    unsafe { std::mem::transmute::<Vec<u64>, Vec<i64>>(u64s) }
}
