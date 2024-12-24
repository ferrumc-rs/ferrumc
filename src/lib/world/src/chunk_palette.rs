use ferrumc_core::transform::position::{self, Position};
use ferrumc_net_codec::net_types::{length_prefixed_vec::LengthPrefixedVec, var_int::VarInt};

// This is only made for blocks currently, biomes and blocks should use the same palettes doe :)
pub trait ChunkPalette: Send + Sync {
    type BlockType;

    fn get_block_index(position: Position) -> i32 {
        ((position.y as i32) * 16 * 16) + ((position.z as i32) * 16) + position.x as i32 
    }

    fn get_block(&self, position: Position) -> Option<Self::BlockType>;

    fn set_block(&mut self, position: Position, block: Self::BlockType);

    fn bits_per_block(&self) -> u8;

}

pub struct SingleValuedPalette {
    data: LengthPrefixedVec<u8>,
    value: VarInt
}

impl ChunkPalette for SingleValuedPalette {
    type BlockType = i32;

    fn get_block(&self, _position: Position) -> Option<Self::BlockType> {
        Some(*self.value)
    }

    fn set_block(&mut self, _position: Position, block: Self::BlockType) {
        self.value = VarInt::from(block);
    }

    fn bits_per_block(&self) -> u8 {
        0
    }
}

impl SingleValuedPalette {

    pub fn empty() -> Self {
        Self {
            data: LengthPrefixedVec::new(vec![0; 24 * 10]),
            value: VarInt::from(0)
        }
    }

}

pub struct IndirectPalette {
    data: LengthPrefixedVec<u8>,
    bits_per_block: u8,
    blocks_per_long: i32,
    palette: LengthPrefixedVec<VarInt>
}

impl ChunkPalette for IndirectPalette {
    type BlockType = i32;

    fn get_block(&self, position: Position) -> Option<Self::BlockType> {
        let index = Self::get_block_index(position);
        let bit_offset = index % (self.bits_per_block as i32);
        let compressed_data_index = (index / (self.bits_per_block as i32)) as usize;
        let block_data = self.data.data[compressed_data_index];

        let palette_index = ((block_data >> bit_offset) & ((1 << self.bits_per_block) - 1)) as usize;
        let block_type = self.palette.data[palette_index];
        
        Some(*block_type)
    }

    fn set_block(&mut self, position: Position, block: Self::BlockType) {
        let block_varint = VarInt::from(block);
        if !self.palette.data.contains(&block_varint) {
            self.palette.data.push(block_varint);
        }

        let index = Self::get_block_index(position);
        let mut l = self.data.data[(index / (self.bits_per_block as i32)) as usize];
        let mask = (1 << (self.bits_per_block as i32)) - 1;
        let offset = (self.bits_per_block as i32) * (index % self.blocks_per_long);
        l &= !(mask << offset);
        l |= (block << offset) as u8;

        self.data.data[(index / self.blocks_per_long) as usize] = l;
    }

    fn bits_per_block(&self) -> u8 {
        self.bits_per_block
    }
}

impl IndirectPalette {

    pub fn empty() -> Self {
        Self::new(4)
    }

    pub fn new(bits_per_block: u8) -> Self {
        Self {
            data: LengthPrefixedVec::new(vec![0; 24 * 10]),
            bits_per_block,
            blocks_per_long: 64 / (bits_per_block as i32),
            palette: LengthPrefixedVec::new(vec![VarInt::from(0); 4096])
        }
    }

}

pub struct DirectPalette;