use crate::chunk::section::paletted::PalettedSection;
use crate::chunk::section::uniform::UniformSection;
use crate::chunk::section::{AIR, CHUNK_SECTION_LENGTH};
use crate::chunk::BlockStateId;
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;

// Currently there are less block state ids than u16::MAX, so we can store ids as u16s to cut down on memory usage
type CompactBlockStateId = u16;

const AIR_COMPACT: CompactBlockStateId = AIR.raw() as CompactBlockStateId;

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub struct DirectSection(pub(crate) Box<[CompactBlockStateId]>, u16);

impl Default for DirectSection {
    fn default() -> Self {
        Self(
            vec![AIR_COMPACT; CHUNK_SECTION_LENGTH].into_boxed_slice(),
            0,
        )
    }
}

impl DirectSection {
    #[inline]
    pub fn set_block(&mut self, idx: usize, block: BlockStateId) {
        if self.0[idx] == AIR_COMPACT && block != AIR {
            self.1 += 1
        } else if self.0[idx] != AIR_COMPACT && block == AIR {
            self.1 -= 1
        }

        self.0[idx] = block.raw() as CompactBlockStateId;
    }

    #[inline]
    pub fn get_block(&self, idx: usize) -> BlockStateId {
        BlockStateId::new(self.0[idx] as _)
    }

    pub fn block_count(&self) -> u16 {
        self.1
    }

    /// Packs every block id into the chunk-packet "data array" layout: a stream of u64s with
    /// 16-bit entries, lowest index in the low bits, no spillover across longs.
    ///
    /// `bits_per_entry` is fixed at 16 for direct sections (we ship the global palette id), so
    /// exactly four entries fit in each long. We could in theory `bytemuck::cast_slice::<u16,
    /// u64>` the inner buffer, but that would assume a specific host endianness; the explicit
    /// shift below is portable and not on a hot path (chunk send, not per-tick).
    pub fn to_network_longs(&self) -> Vec<u64> {
        const ENTRIES_PER_LONG: usize = 4;
        const BITS_PER_ENTRY: usize = 16;
        let mut out = vec![0u64; CHUNK_SECTION_LENGTH / ENTRIES_PER_LONG];
        for (i, &id) in self.0.iter().enumerate() {
            let long_idx = i / ENTRIES_PER_LONG;
            let bit_idx = (i % ENTRIES_PER_LONG) * BITS_PER_ENTRY;
            out[long_idx] |= (id as u64) << bit_idx;
        }
        out
    }
}

impl From<&mut UniformSection> for DirectSection {
    fn from(s: &mut UniformSection) -> Self {
        Self(
            vec![s.get_block().raw() as CompactBlockStateId; CHUNK_SECTION_LENGTH]
                .into_boxed_slice(),
            if s.get_block() == AIR { 0 } else { 4096 },
        )
    }
}

impl From<&mut PalettedSection> for DirectSection {
    fn from(s: &mut PalettedSection) -> Self {
        let mut vec = vec![AIR_COMPACT; CHUNK_SECTION_LENGTH];
        let mut count = 0;

        for (block_idx, val) in vec.iter_mut().enumerate() {
            let block = s.get_block(block_idx);
            *val = s.get_block(block_idx).raw() as CompactBlockStateId;

            if block != AIR {
                count += 1
            }
        }

        Self(vec.into_boxed_slice(), count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Round-trip: every block id we put into a DirectSection must be recoverable from the packed
    /// network long array. This guards against the previous bug where the data_array was sent
    /// empty, causing every block to decode as 0 client-side (lava rendering as water etc.).
    #[test]
    fn to_network_longs_round_trips() {
        let mut section = DirectSection::default();
        // A handful of ids spread across the 4096 cells, including the boundaries between longs.
        let samples: &[(usize, u32)] = &[
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),      // first long
            (4, 100),    // second long, low entry
            (7, 0xFFFF), // second long, high entry (max representable)
            (4095, 7),   // last cell
        ];
        for &(idx, id) in samples {
            section.set_block(idx, BlockStateId::new(id));
        }

        let longs = section.to_network_longs();
        assert_eq!(longs.len(), CHUNK_SECTION_LENGTH / 4);

        // Manually decode each long (4 entries of 16 bits, lowest index in the low bits) and
        // compare against the section's stored ids.
        for long_idx in 0..longs.len() {
            for entry in 0..4 {
                let block_idx = long_idx * 4 + entry;
                let decoded = ((longs[long_idx] >> (entry * 16)) & 0xFFFF) as u32;
                assert_eq!(
                    decoded,
                    section.get_block(block_idx).raw(),
                    "mismatch at block_idx {}",
                    block_idx
                );
            }
        }
    }
}
