use crate::palette::MIN_BITS_PER_ENTRY;

pub(in crate::palette) fn read_index(data: &[i64], bits_per_entry: u8, index: usize) -> i64 {
    let entries_per_u64 = 64 / bits_per_entry as usize;
    let u64_index = index / entries_per_u64;
    let bit_offset = (index % entries_per_u64) * bits_per_entry as usize;
    let packed = data[u64_index];
    (packed >> bit_offset) & ((1i64 << bits_per_entry) - 1)
}

pub(in crate::palette) fn write_index(
    data: &mut [i64],
    bits_per_entry: u8,
    index: usize,
    value: i64,
) {
    let entries_per_u64 = 64 / bits_per_entry as usize;
    let u64_index = index / entries_per_u64;
    let bit_offset = (index % entries_per_u64) * bits_per_entry as usize;
    let mask = ((1i64 << bits_per_entry) - 1) << bit_offset;
    data[u64_index] = (data[u64_index] & !mask) | ((value << bit_offset) & mask);
}

pub(crate) fn calculate_bits_per_entry(palette_size: usize) -> u8 {
    match palette_size {
        0..=16 => MIN_BITS_PER_ENTRY,
        17..=32 => 5,
        33..=64 => 6,
        65..=128 => 7,
        129..=256 => 8,
        257..=512 => 9,
        513..=1024 => 10,
        1025..=2048 => 11,
        2049..=4096 => 12,
        4097..=8192 => 13,
        8193..=16384 => 14,
        _ => 15,
    }
}

pub(crate) fn calculate_unique_values<T: Eq + std::hash::Hash>(values: &[T]) -> usize {
    use std::collections::HashSet;
    HashSet::<&T>::from_iter(values.iter()).len()
}

#[allow(dead_code)]
pub(crate) fn pack_indices(indices: &[u16], bits_per_entry: u8) -> Vec<i64> {
    let entries_per_u64 = 64 / bits_per_entry as usize;
    let data_len = indices.len().div_ceil(entries_per_u64);
    let mut data = vec![0i64; data_len];
    for (i, &idx) in indices.iter().enumerate() {
        write_index(&mut data, bits_per_entry, i, idx as i64);
    }
    data
}
