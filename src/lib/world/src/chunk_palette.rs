use std::io::{Cursor, Write};

use bitcode::{Decode, Encode};
use deepsize::DeepSizeOf;
use ferrumc_core::transform::position::{self, Position};
use ferrumc_net_codec::net_types::{length_prefixed_vec::LengthPrefixedVec, var_int::VarInt};

use crate::chunk_format::Section;

// This is only made for blocks currently, biomes and blocks should use the same palettes doe :)
pub trait ChunkPalette: Send + Sync {

    fn get_block(&self, position: Position) -> Option<i32>;

    fn set_block(&mut self, position: Position, block: i32);

    fn bits_per_block(&self) -> u8;

}

#[derive(Encode, Decode, Clone, DeepSizeOf)]
pub struct SingleValuedPalette {
    pub data: LengthPrefixedVec<u8>,
    pub value: VarInt,
    pub non_air_blocks: u16,
}

impl ChunkPalette for SingleValuedPalette {

    fn get_block(&self, _position: Position) -> Option<i32> {
        Some(*self.value)
    }

    fn set_block(&mut self, _position: Position, block: i32) {
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
            value: VarInt::from(0),
            non_air_blocks: 0
        }
    }

}

#[derive(Encode, Decode, Clone, DeepSizeOf)]
pub struct IndirectPalette {
    pub data: LengthPrefixedVec<u8>,
    pub bits_per_block: u8,
    blocks_per_long: i32,
    pub palette: LengthPrefixedVec<VarInt>,
    pub non_air_blocks: u16,
}

impl ChunkPalette for IndirectPalette {

    fn get_block(&self, position: Position) -> Option<i32> {
        let index = self.get_block_index(position);
        let bit_offset = index % (self.bits_per_block as i32);
        let compressed_data_index = (index / (self.bits_per_block as i32)) as usize;
        let block_data = self.data[compressed_data_index];

        let palette_index = ((block_data >> bit_offset) & ((1 << self.bits_per_block) - 1)) as usize;
        let block_type = self.palette[palette_index];
        
        Some(*block_type)
    }

    fn set_block(&mut self, position: Position, block: i32) {
        let block_varint = VarInt::from(block);
        if !self.palette.data.contains(&block_varint) {
            self.palette.data.push(block_varint);
        }

        let index = self.get_block_index(position);
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
            palette: LengthPrefixedVec::new(vec![VarInt::from(0); 4096]),
            non_air_blocks: 0
        }
    }

    fn get_block_index(&self, position: Position) -> i32 {
        ((position.y as i32) * 16 * 16) + ((position.z as i32) * 16) + position.x as i32 
    }

}

#[derive(Encode, Decode, Clone, DeepSizeOf)]
pub struct DirectPalette {
    pub data: LengthPrefixedVec<u8>,
    pub non_air_blocks: u16,
}

impl DirectPalette {

    pub fn empty() -> Self {
        Self {
            data: LengthPrefixedVec::new(vec![0; 24 * 10]),
            non_air_blocks: 0
        }
    }

}

impl ChunkPalette for DirectPalette {

    fn get_block(&self, position: Position) -> Option<i32> {
        None
    }

    fn set_block(&mut self, position: Position, block: i32) {
        
    }

    fn bits_per_block(&self) -> u8 {
        15
    }
}

#[derive(Encode, Decode, Clone, DeepSizeOf)]
pub enum PaletteType {
    SingleValued(SingleValuedPalette),
    Indirect(IndirectPalette),
    Direct(DirectPalette)
}

impl PaletteType {

    pub fn get_chunk_palette_bits(bits_per_block: u8) -> Option<PaletteType> {
        match bits_per_block {
            0 => Some(PaletteType::SingleValued(SingleValuedPalette::empty())),
            4..8 => Some(PaletteType::Indirect(IndirectPalette::empty())),
            15 => Some(PaletteType::Direct(DirectPalette::empty())),
            _ => None,
        }
    }

    pub fn get_bits_per_block(&self) -> u8 {
        match self {
            Self::SingleValued(palette) => palette.bits_per_block(),
            Self::Indirect(palette) => palette.bits_per_block(),
            Self::Direct(palette) => palette.bits_per_block(),
        }
    }

    pub fn get_data(&self) -> Option<&LengthPrefixedVec<u8>> {
        match self {
            Self::Indirect(palette) => Some(&palette.data),
            Self::Direct(palette) => Some(&palette.data),
            _ => None
        }
    }

}