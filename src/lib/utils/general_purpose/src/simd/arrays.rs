#[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
use std::arch::x86_64::*;

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn has_avx2() -> bool {
    is_x86_feature_detected!("avx2")
}

/// Converts a slice of `u8` to a slice of `i8` without copying.
pub const fn u8_slice_to_i8(input: &[u8]) -> &[i8] {
    // SAFETY: u8 and i8 have the same size, alignment and valid bit-patterns
    unsafe { std::mem::transmute(input) }
}

/// Converts a slice of `u8` to a `Vec<u32>` in big-endian order.
pub fn u8_slice_to_u32_be(input: &[u8]) -> Vec<u32> {
    assert_eq!(
        input.len() % 4,
        0,
        "Input length must be a multiple of 4 for u32 conversion"
    );

    #[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
    if has_avx2() {
        return unsafe { u8_slice_to_u32_be_simd(input) };
    }
    u8_slice_to_u32_be_normal(input)
}

fn u8_slice_to_u32_be_normal(input: &[u8]) -> Vec<u32> {
    input
        .chunks_exact(4)
        .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
        .collect()
}

#[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
unsafe fn u8_slice_to_u32_be_simd(input: &[u8]) -> Vec<u32> { unsafe {
    debug_assert_eq!(
        input.len() % 4,
        0,
        "Input length must be a multiple of 4 for u32 conversion"
    );

    let mut output: Vec<u32> = Vec::new();
    output.reserve_exact(input.len() / 4);

    let shuffle_mask = _mm256_setr_epi8(
        3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20, 27,
        26, 25, 24, 31, 30, 29, 28,
    );
    let mut input = input.chunks_exact(32);
    for (i, chunk) in input.by_ref().enumerate() {
        let out = output.as_mut_ptr().cast::<__m256i>().add(i);
        let data = _mm256_loadu_si256(chunk.as_ptr().cast());
        let shuffled = _mm256_shuffle_epi8(data, shuffle_mask);
        _mm256_storeu_si256(out, shuffled);
        output.set_len((i + 1) * 8);
    }

    let input = input.remainder();
    let input = input.chunks_exact(8);

    for chunk in input {
        let bytes: [u8; 4] = chunk.try_into().unwrap();
        let val = u32::from_be_bytes(bytes);
        output.push(val);
    }

    output
}}

pub fn u8_slice_to_i32_be(input: &[u8]) -> Vec<i32> {
    let out = u8_slice_to_u32_be(input);
    // SAFETY: u32 and i32 have the same size, alignment and valid bit-patterns
    unsafe { std::mem::transmute(out) }
}

pub fn u8_slice_to_u64_be(input: &[u8]) -> Vec<u64> {
    assert_eq!(
        input.len() % 8,
        0,
        "Input length must be a multiple of 8 for u64 conversion"
    );

    #[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
    if has_avx2() {
        return unsafe { u8_slice_to_u64_be_simd(input) };
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
unsafe fn u8_slice_to_u64_be_simd(input: &[u8]) -> Vec<u64> { unsafe {
    debug_assert_eq!(
        input.len() % 8,
        0,
        "Input length must be a multiple of 8 for u64 conversion"
    );

    let mut output: Vec<u64> = Vec::new();
    output.reserve_exact(input.len() / 8);

    let mut input = input.chunks_exact(32);

    let shuffle_mask = _mm256_setr_epi8(
        7, 6, 5, 4, 3, 2, 1, 0, // Reverse first u64
        15, 14, 13, 12, 11, 10, 9, 8, // Reverse second u64
        23, 22, 21, 20, 19, 18, 17, 16, // Reverse third u64
        31, 30, 29, 28, 27, 26, 25, 24, // Reverse fourth u64
    );

    for (i, chunk) in input.by_ref().enumerate() {
        let out = output.as_mut_ptr().cast::<__m256i>().add(i);
        let data = _mm256_loadu_si256(chunk.as_ptr().cast());
        let shuffled = _mm256_shuffle_epi8(data, shuffle_mask);
        _mm256_storeu_si256(out, shuffled);
        output.set_len((i + 1) * 4);
    }
    let input = input.remainder();

    for chunk in input.chunks_exact(8) {
        let bytes: [u8; 8] = chunk.try_into().unwrap();
        let val = u64::from_be_bytes(bytes);
        output.push(val);
    }

    output
}}

pub fn u8_slice_to_i64_be(input: &[u8]) -> Vec<i64> {
    let out = u8_slice_to_u64_be(input);
    // SAFETY: u64 and i64 have the same size, alignment and valid bit-patterns
    unsafe { std::mem::transmute(out) }
}

pub fn u32_slice_to_u8_be(input: &[u32]) -> Vec<u8> {
    #[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
    if has_avx2() {
        return unsafe { u32_slice_to_u8_be_simd(input) };
    }
    u32_slice_to_u8_be_normal(input)
}

fn u32_slice_to_u8_be_normal(input: &[u32]) -> Vec<u8> {
    input.iter().flat_map(|val| val.to_be_bytes()).collect()
}

#[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
#[target_feature(enable = "avx2")]
unsafe fn u32_slice_to_u8_be_simd(input: &[u32]) -> Vec<u8> { unsafe {
    let mut output: Vec<u8> = Vec::new();
    output.reserve_exact(input.len() * 4);

    let shuffle_mask = _mm256_setr_epi8(
        3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20, 27,
        26, 25, 24, 31, 30, 29, 28,
    );

    let mut input = input.chunks_exact(8);
    for (i, chunk) in input.by_ref().enumerate() {
        let out = output.as_mut_ptr().cast::<__m256i>().add(i);
        let data = _mm256_loadu_si256(chunk.as_ptr().cast());
        let shuffled = _mm256_shuffle_epi8(data, shuffle_mask);
        _mm256_storeu_si256(out, shuffled);
        output.set_len((i + 1) * 32);
    }

    let input = input.remainder();

    for val in input {
        let val = val.to_be_bytes();
        output.extend_from_slice(&val);
    }

    output
}}

pub fn u64_slice_to_u8_be(input: &[u64]) -> Vec<u8> {
    #[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
    if has_avx2() {
        return unsafe { u64_slice_to_u8_be_simd(input) };
    }
    u64_slice_to_u8_be_normal(input)
}

fn u64_slice_to_u8_be_normal(input: &[u64]) -> Vec<u8> {
    input.iter().flat_map(|val| val.to_be_bytes()).collect()
}

#[cfg(all(target_arch = "x86_64", not(target_os = "macos")))]
#[target_feature(enable = "avx2")]
unsafe fn u64_slice_to_u8_be_simd(input: &[u64]) -> Vec<u8> { unsafe {
    let mut output: Vec<u8> = Vec::new();
    output.reserve_exact(input.len() * 8);

    let shuffle_mask = _mm256_setr_epi8(
        7, 6, 5, 4, 3, 2, 1, 0, // Reverse first u64
        15, 14, 13, 12, 11, 10, 9, 8, // Reverse second u64
        23, 22, 21, 20, 19, 18, 17, 16, // Reverse third u64
        31, 30, 29, 28, 27, 26, 25, 24, // Reverse fourth u64
    );

    let mut input = input.chunks_exact(4);

    for (i, chunk) in input.by_ref().enumerate() {
        let out = output.as_mut_ptr().cast::<__m256i>().add(i);
        let data = _mm256_loadu_si256(chunk.as_ptr().cast());
        let shuffled = _mm256_shuffle_epi8(data, shuffle_mask);
        _mm256_storeu_si256(out, shuffled);
        output.set_len((i + 1) * 32);
    }

    for val in input.remainder() {
        let val = val.to_be_bytes();
        output.extend_from_slice(&val);
    }

    output
}}
