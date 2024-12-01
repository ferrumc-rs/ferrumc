use std::slice;

#[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
use std::arch::x86_64::*;

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn has_avx2() -> bool {
    is_x86_feature_detected!("avx2")
}

/// Converts a slice of `u8` to a slice of `i8` without copying.
pub fn u8_slice_to_i8(input: &[u8]) -> &[i8] {
    unsafe { slice::from_raw_parts(input.as_ptr() as *const i8, input.len()) }
}

/// Converts a slice of `u8` to a `Vec<u32>` in big-endian order.
pub fn u8_slice_to_u32_be(input: &[u8]) -> Vec<u32> {
    assert_eq!(
        input.len() % 4,
        0,
        "Input length must be a multiple of 4 for u32 conversion"
    );

    #[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
    {
        if has_avx2() {
            return unsafe { u8_slice_to_u32_be_simd(input) };
        }
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "avx2",
        not(target_os = "macos")
    )))]
    {
        u8_slice_to_u32_be_normal(input)
    }
}

fn u8_slice_to_u32_be_normal(input: &[u8]) -> Vec<u32> {
    input
        .chunks_exact(4)
        .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
        .collect()
}

#[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
#[target_feature(enable = "avx2")]
unsafe fn u8_slice_to_u32_be_simd(input: &[u8]) -> Vec<u32> {
    use std::mem;

    debug_assert_eq!(
        input.len() % 4,
        0,
        "Input length must be a multiple of 4 for u32 conversion"
    );

    let num_elements = input.len() / 4;
    let mut output = Vec::with_capacity(num_elements);
    let mut i = 0;

    let shuffle_mask = _mm256_setr_epi8(
        3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20, 27,
        26, 25, 24, 31, 30, 29, 28,
    );

    while i + 32 <= input.len() {
        let data = _mm256_loadu_si256(input.as_ptr().add(i) as *const __m256i);
        let shuffled = _mm256_shuffle_epi8(data, shuffle_mask);
        let mut temp = mem::MaybeUninit::<[u8; 32]>::uninit();
        _mm256_storeu_si256(temp.as_mut_ptr() as *mut __m256i, shuffled);
        let temp = temp.assume_init();

        for j in 0..8 {
            let bytes: [u8; 4] = temp[j * 4..(j + 1) * 4]
                .try_into()
                .expect("SIMD conversion failed");
            let val = u32::from_le_bytes(bytes);
            output.push(val);
        }

        i += 32;
    }

    while i + 4 <= input.len() {
        let bytes: [u8; 4] = input[i..i + 4].try_into().expect("SIMD conversion failed");
        let val = u32::from_be_bytes(bytes);
        output.push(val);
        i += 4;
    }

    output
}

pub fn u8_slice_to_i32_be(input: &[u8]) -> Vec<i32> {
    let u32s = u8_slice_to_u32_be(input);
    unsafe { std::mem::transmute::<Vec<u32>, Vec<i32>>(u32s) }
}

pub fn u8_slice_to_u64_be(input: &[u8]) -> Vec<u64> {
    assert_eq!(
        input.len() % 8,
        0,
        "Input length must be a multiple of 8 for u64 conversion"
    );

    #[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
    {
        if has_avx2() {
            return unsafe { u8_slice_to_u64_be_simd(input) };
        }
    }
    u8_slice_to_u64_be_normal(input)
}

fn u8_slice_to_u64_be_normal(input: &[u8]) -> Vec<u64> {
    input
        .chunks_exact(8)
        .map(|chunk| u64::from_be_bytes(chunk.try_into().unwrap()))
        .collect()
}

#[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
#[target_feature(enable = "avx2")]
unsafe fn u8_slice_to_u64_be_simd(input: &[u8]) -> Vec<u64> {
    use std::mem;

    debug_assert_eq!(
        input.len() % 8,
        0,
        "Input length must be a multiple of 8 for u64 conversion"
    );

    let num_elements = input.len() / 8;
    let mut output = Vec::with_capacity(num_elements);
    let mut i = 0;

    let shuffle_mask = _mm256_setr_epi8(
        7, 6, 5, 4, 3, 2, 1, 0, // Reverse first u64
        15, 14, 13, 12, 11, 10, 9, 8, // Reverse second u64
        23, 22, 21, 20, 19, 18, 17, 16, // Reverse third u64
        31, 30, 29, 28, 27, 26, 25, 24, // Reverse fourth u64
    );

    while i + 32 <= input.len() {
        let data = _mm256_loadu_si256(input.as_ptr().add(i) as *const __m256i);
        let shuffled = _mm256_shuffle_epi8(data, shuffle_mask);
        let mut temp = mem::MaybeUninit::<[u8; 32]>::uninit();
        _mm256_storeu_si256(temp.as_mut_ptr() as *mut __m256i, shuffled);
        let temp = temp.assume_init();

        for j in 0..4 {
            let bytes: [u8; 8] = temp[j * 8..(j + 1) * 8]
                .try_into()
                .expect("SIMD conversion failed");
            let val = u64::from_le_bytes(bytes);
            output.push(val);
        }

        i += 32;
    }

    while i + 8 <= input.len() {
        let bytes: [u8; 8] = input[i..i + 8].try_into().expect("SIMD conversion failed");
        let val = u64::from_be_bytes(bytes);
        output.push(val);
        i += 8;
    }

    output
}

pub fn u8_slice_to_i64_be(input: &[u8]) -> Vec<i64> {
    let u64s = u8_slice_to_u64_be(input);
    u64s.into_iter().map(|x| x as i64).collect()
}

pub fn u32_slice_to_u8_be(input: &[u32]) -> Vec<u8> {
    #[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
    {
        if has_avx2() {
            println!("Using SIMD");
            return unsafe { u32_slice_to_u8_be_simd(input) };
        }
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "avx2",
        not(target_os = "macos")
    )))]
    {
        println!("Not using SIMD");
        u32_slice_to_u8_be_normal(input)
    }
}

fn u32_slice_to_u8_be_normal(input: &[u32]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len() * 4);
    for val in input {
        output.extend_from_slice(&val.to_be_bytes());
    }
    output
}

#[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
#[target_feature(enable = "avx2")]
unsafe fn u32_slice_to_u8_be_simd(input: &[u32]) -> Vec<u8> {
    use std::mem;

    let num_elements = input.len();
    let mut output = Vec::with_capacity(num_elements * 4);
    let mut i = 0;

    let shuffle_mask = _mm256_setr_epi8(
        3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20, 27,
        26, 25, 24, 31, 30, 29, 28,
    );

    while i + 8 <= num_elements {
        let data = _mm256_loadu_si256(input.as_ptr().add(i) as *const __m256i);
        let shuffled = _mm256_shuffle_epi8(data, shuffle_mask);
        let mut temp = mem::MaybeUninit::<[u8; 32]>::uninit();
        _mm256_storeu_si256(temp.as_mut_ptr() as *mut __m256i, shuffled);
        let temp = temp.assume_init();
        output.extend_from_slice(&temp);
        i += 8;
    }

    while i < num_elements {
        let val = input[i].to_be_bytes();
        output.extend_from_slice(&val);
        i += 1;
    }

    output
}

pub fn u64_slice_to_u8_be(input: &[u64]) -> Vec<u8> {
    #[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
    {
        if has_avx2() {
            return unsafe { u64_slice_to_u8_be_simd(input) };
        }
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "avx2",
        not(target_os = "macos")
    )))]
    {
        u64_slice_to_u8_be_normal(input)
    }
}

fn u64_slice_to_u8_be_normal(input: &[u64]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len() * 8);
    for val in input {
        output.extend_from_slice(&val.to_be_bytes());
    }
    output
}

#[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
#[target_feature(enable = "avx2")]
unsafe fn u64_slice_to_u8_be_simd(input: &[u64]) -> Vec<u8> {
    use std::mem;

    let num_elements = input.len();
    let mut output = Vec::with_capacity(num_elements * 8);
    let mut i = 0;

    let shuffle_mask = _mm256_setr_epi8(
        7, 6, 5, 4, 3, 2, 1, 0, // Reverse first u64
        15, 14, 13, 12, 11, 10, 9, 8, // Reverse second u64
        23, 22, 21, 20, 19, 18, 17, 16, // Reverse third u64
        31, 30, 29, 28, 27, 26, 25, 24, // Reverse fourth u64
    );

    while i + 4 <= num_elements {
        let data = _mm256_loadu_si256(input.as_ptr().add(i) as *const __m256i);
        let shuffled = _mm256_shuffle_epi8(data, shuffle_mask);
        let mut temp = mem::MaybeUninit::<[u8; 32]>::uninit();
        _mm256_storeu_si256(temp.as_mut_ptr() as *mut __m256i, shuffled);
        let temp = temp.assume_init();
        output.extend_from_slice(&temp);
        i += 4;
    }

    while i < num_elements {
        let val = input[i].to_be_bytes();
        output.extend_from_slice(&val);
        i += 1;
    }

    output
}
