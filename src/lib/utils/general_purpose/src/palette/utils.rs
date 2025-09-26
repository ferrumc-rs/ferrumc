use crate::palette::MIN_BITS_PER_ENTRY;

pub(in crate::palette) fn read_index(data: &[u64], bits_per_entry: u8, index: usize) -> u64 {
    let entries_per_i64 = (64f64 / bits_per_entry as f64).floor() as usize;
    let i64_index = index / entries_per_i64;
    let packed_u64 = data[i64_index];
    let offset = (index % entries_per_i64) * bits_per_entry as usize;
    (packed_u64 >> offset) & ((1u64 << bits_per_entry) - 1)
}

pub(in crate::palette) fn write_index(
    data: &mut [u64],
    bits_per_entry: u8,
    index: usize,
    value: u64,
) {
    let u64_index = (index * bits_per_entry as usize) / 64;
    let bit_offset = (index * bits_per_entry as usize) % 64;
    let mask = ((1u64 << bits_per_entry) - 1) << bit_offset;
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
        _ => 15, // Max palette size is 32768
    }
}

pub(crate) fn calculate_unique_values<T: Eq + std::hash::Hash>(values: &[T]) -> usize {
    // Should probably use a more efficient method for large datasets
    // Requiring crypto hashes for all T is a bit much
    use std::collections::HashSet;
    let unique_values: HashSet<&T> = values.iter().collect();
    unique_values.len()
}
