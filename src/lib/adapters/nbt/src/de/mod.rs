use crate::errors::NBTError;
use std::arch::x86_64::{
    __m128i, _mm_loadu_si128, _mm_setr_epi8, _mm_shuffle_epi8, _mm_storeu_si128,
};
use std::io::Read;
use std::str;

/// Represents a token in the NBT tape.
#[derive(Debug)]
pub enum NbtToken<'a> {
    TagStart { tag_type: u8, name: Option<&'a str> },
    TagEnd,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(&'a [u8]),
    String(&'a str),
    ListStart { element_type: u8, length: usize },
    ListEnd,
    IntArray(&'a [i32]),
    LongArray(&'a [i64]),
}

/// NBT parser using a tape-based approach.
pub struct NbtParser<'a> {
    data: &'a [u8],
    pos: usize,
    tape: Vec<NbtToken<'a>>,
}

impl<'a> NbtParser<'a> {
    /// Creates a new `NbtParser` from the given data slice.
    pub fn new(data: &'a [u8]) -> NbtParser<'a> {
        NbtParser {
            data,
            pos: 0,
            tape: Vec::new(),
        }
    }

    /// Parses the NBT data and returns the tape of tokens.
    pub fn parse(&'a mut self) -> Result<&[NbtToken<'a>], NBTError> {
        if Self::is_compressed(self.data) {
            return Err(NBTError::CompressedData);
        }

        let tag_type = self.read_u8()?;
        if tag_type != 10 {
            return Err(NBTError::InvalidRootCompound(tag_type));
        }
        let name = self.parse_string()?;
        self.tape.push(NbtToken::TagStart {
            tag_type,
            name: Some(name),
        });
        self.parse_payload(tag_type)?;
        self.tape.push(NbtToken::TagEnd);
        Ok(&self.tape)
    }

    /// Decompresses the given NBT data.
    pub fn decompress(data: &[u8]) -> Result<Vec<u8>, NBTError> {
        if !Self::is_compressed(data) {
            return Ok(data.to_vec());
        }

        let mut decoder = libflate::gzip::Decoder::new(data)?;
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        Ok(decompressed)
    }

    pub(crate) fn is_compressed(data: &[u8]) -> bool {
        data.starts_with(&[0x1F, 0x8B])
    }

    fn parse_string(&mut self) -> Result<&'a str, NBTError> {
        let len = self.read_u16()? as usize;
        if self.pos + len > self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }

        // SAFETY: We just checked that the data is long enough.
        let s = unsafe { str::from_utf8_unchecked(&self.data[self.pos..self.pos + len]) };
        self.pos += len;
        Ok(s)
    }

    fn parse_payload(&mut self, tag_type: u8) -> Result<(), NBTError> {
        match tag_type {
            0 => Ok(()),
            1 => {
                // TAG_Byte
                let v = self.read_i8()?;
                self.tape.push(NbtToken::Byte(v));
                Ok(())
            }
            2 => {
                // TAG_Short
                let v = self.read_i16()?;
                self.tape.push(NbtToken::Short(v));
                Ok(())
            }
            3 => {
                // TAG_Int
                let v = self.read_i32()?;
                self.tape.push(NbtToken::Int(v));
                Ok(())
            }
            4 => {
                // TAG_Long
                let v = self.read_i64()?;
                self.tape.push(NbtToken::Long(v));
                Ok(())
            }
            5 => {
                // TAG_Float
                let v = self.read_f32()?;
                self.tape.push(NbtToken::Float(v));
                Ok(())
            }
            6 => {
                // TAG_Double
                let v = self.read_f64()?;
                self.tape.push(NbtToken::Double(v));
                Ok(())
            }
            7 => {
                // TAG_Byte_Array
                let len = self.read_i32()? as usize;
                if self.pos + len > self.data.len() {
                    return Err(NBTError::UnexpectedEndOfData);
                }
                let v = &self.data[self.pos..self.pos + len];
                self.pos += len;
                self.tape.push(NbtToken::ByteArray(v));
                Ok(())
            }
            8 => {
                // TAG_String
                let s = self.parse_string()?;
                self.tape.push(NbtToken::String(s));
                Ok(())
            }
            9 => {
                // TAG_List
                let item_type = self.read_u8()?;
                let len = self.read_i32()? as usize;
                self.tape.push(NbtToken::ListStart {
                    element_type: item_type,
                    length: len,
                });
                for _ in 0..len {
                    self.parse_payload(item_type)?;
                }
                self.tape.push(NbtToken::ListEnd);
                Ok(())
            }
            10 => {
                // TAG_Compound
                loop {
                    let tag_type = self.read_u8()?;
                    if tag_type == 0 {
                        break;
                    }
                    let name = self.parse_string()?;
                    self.tape.push(NbtToken::TagStart {
                        tag_type,
                        name: Some(name),
                    });
                    self.parse_payload(tag_type)?;
                    self.tape.push(NbtToken::TagEnd);
                }
                Ok(())
            }
            11 => {
                // TAG_Int_Array
                let len = self.read_i32()? as usize;
                let array = self.read_i32_array(len)?;
                self.tape.push(NbtToken::IntArray(array));
                Ok(())
            }
            12 => {
                // TAG_Long_Array
                let len = self.read_i32()? as usize;
                let array = self.read_i64_array(len)?;
                self.tape.push(NbtToken::LongArray(array));
                Ok(())
            }
            _ => unreachable!("Invalid tag type: {}", tag_type),
        }
    }

    /// Reads an u8 from the data.
    #[inline(always)]
    fn read_u8(&mut self) -> Result<u8, NBTError> {
        if self.pos >= self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }
        let v = unsafe { *self.data.get_unchecked(self.pos) };
        self.pos += 1;
        Ok(v)
    }

    /// Reads an i8 from the data.
    #[inline(always)]
    fn read_i8(&mut self) -> Result<i8, NBTError> {
        Ok(self.read_u8()? as i8)
    }

    /// Reads a big-endian u16 from the data.
    #[inline(always)]
    fn read_u16(&mut self) -> Result<u16, NBTError> {
        if self.pos + 2 > self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }
        let v = unsafe {
            u16::from_be_bytes([
                *self.data.get_unchecked(self.pos),
                *self.data.get_unchecked(self.pos + 1),
            ])
        };
        self.pos += 2;
        Ok(v)
    }

    /// Reads a big-endian i16 from the data.
    #[inline(always)]
    fn read_i16(&mut self) -> Result<i16, NBTError> {
        Ok(self.read_u16()? as i16)
    }

    /// Reads a big-endian u32 from the data.
    #[inline(always)]
    fn read_u32(&mut self) -> Result<u32, NBTError> {
        if self.pos + 4 > self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }
        let v = unsafe {
            u32::from_be_bytes([
                *self.data.get_unchecked(self.pos),
                *self.data.get_unchecked(self.pos + 1),
                *self.data.get_unchecked(self.pos + 2),
                *self.data.get_unchecked(self.pos + 3),
            ])
        };
        self.pos += 4;
        Ok(v)
    }

    /// Reads a big-endian i32 from the data.
    #[inline(always)]
    fn read_i32(&mut self) -> Result<i32, NBTError> {
        Ok(self.read_u32()? as i32)
    }

    /// Reads a big-endian u64 from the data.
    #[inline(always)]
    fn read_u64(&mut self) -> Result<u64, NBTError> {
        if self.pos + 8 > self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }
        let v = unsafe {
            u64::from_be_bytes([
                *self.data.get_unchecked(self.pos),
                *self.data.get_unchecked(self.pos + 1),
                *self.data.get_unchecked(self.pos + 2),
                *self.data.get_unchecked(self.pos + 3),
                *self.data.get_unchecked(self.pos + 4),
                *self.data.get_unchecked(self.pos + 5),
                *self.data.get_unchecked(self.pos + 6),
                *self.data.get_unchecked(self.pos + 7),
            ])
        };
        self.pos += 8;
        Ok(v)
    }

    /// Reads a big-endian i64 from the data.
    #[inline(always)]
    fn read_i64(&mut self) -> Result<i64, NBTError> {
        Ok(self.read_u64()? as i64)
    }

    /// Reads a big-endian f32 from the data.
    #[inline(always)]
    fn read_f32(&mut self) -> Result<f32, NBTError> {
        let bits = self.read_u32()?;
        Ok(f32::from_bits(bits))
    }

    /// Reads a big-endian f64 from the data.
    #[inline(always)]
    fn read_f64(&mut self) -> Result<f64, NBTError> {
        let bits = self.read_u64()?;
        Ok(f64::from_bits(bits))
    }

    /*/// Reads an array of i32 from the data.
    fn read_i32_array(&mut self, len: usize) -> Result<&'a [i32], NBTError> {
        let byte_len = len * 4;
        if self.pos + byte_len > self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }
        let bytes = &self.data[self.pos..self.pos + byte_len];
        if bytes.as_ptr().align_offset(std::mem::align_of::<i32>()) != 0 {
            // return Err("Data is not properly aligned for i32 array".to_string());
            return Err(NBTError::InvalidNBTData);
        }
        #[allow(clippy::cast_ptr_alignment)]
        let array = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const i32, len) };
        self.pos += byte_len;
        Ok(array)
    }

    /// Reads an array of i64 from the data.
    fn read_i64_array(&mut self, len: usize) -> Result<&'a [i64], NBTError> {
        let byte_len = len * 8;
        if self.pos + byte_len > self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }
        let bytes = &self.data[self.pos..self.pos + byte_len];
        if bytes.as_ptr().align_offset(align_of::<i64>()) != 0 {
            return Err(NBTError::InvalidNBTData);
        }
        #[allow(clippy::cast_ptr_alignment)]
        let array = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const i64, len) };
        self.pos += byte_len;
        Ok(array)
    }*/
    /// Reads an array of i32 from the data using SIMD when possible, supporting unaligned data.
    fn read_i32_array(&mut self, len: usize) -> Result<&'a [i32], NBTError> {
        let byte_len = len * size_of::<i32>();
        if self.pos + byte_len > self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }
        let bytes = &self.data[self.pos..self.pos + byte_len];

        // Create a new aligned buffer
        let mut aligned_buffer = vec![0i32; len];

        unsafe {
            let mut src_pos = 0;
            let mut dst_pos = 0;
            let mut remaining = len;

            // Use SIMD for chunks of 4 i32s
            while remaining >= 4 && src_pos + 16 <= byte_len {
                #[allow(clippy::cast_ptr_alignment)]
                let simd_bytes = _mm_loadu_si128(bytes[src_pos..].as_ptr() as *const __m128i);
                let simd_ints = _mm_shuffle_epi8(
                    simd_bytes,
                    _mm_setr_epi8(3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12),
                );
                #[allow(clippy::cast_ptr_alignment)]
                _mm_storeu_si128(
                    aligned_buffer[dst_pos..].as_mut_ptr() as *mut __m128i,
                    simd_ints,
                );
                src_pos += 16;
                dst_pos += 4;
                remaining -= 4;
            }

            // Handle remaining elements
            while remaining > 0 {
                aligned_buffer[dst_pos] = i32::from_be_bytes([
                    bytes[src_pos],
                    bytes[src_pos + 1],
                    bytes[src_pos + 2],
                    bytes[src_pos + 3],
                ]);
                src_pos += 4;
                dst_pos += 1;
                remaining -= 1;
            }
        }

        self.pos += byte_len;

        // Convert the Vec<i32> to a &'a [i32]
        // This is safe because we're leaking the memory, which will live for 'a
        let leaked_slice = aligned_buffer.leak();
        Ok(leaked_slice)
    }

    /// Reads an array of i64 from the data using SIMD when possible, supporting unaligned data.
    fn read_i64_array(&mut self, len: usize) -> Result<&'a [i64], NBTError> {
        let byte_len = len * size_of::<i64>();
        if self.pos + byte_len > self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }
        let bytes = &self.data[self.pos..self.pos + byte_len];

        // Create a new aligned buffer
        let mut aligned_buffer = vec![0i64; len];

        unsafe {
            let mut src_pos = 0;
            let mut dst_pos = 0;
            let mut remaining = len;

            // Use SIMD for chunks of 2 i64s
            while remaining >= 2 && src_pos + 16 <= byte_len {
                #[allow(clippy::cast_ptr_alignment)]
                let simd_bytes = _mm_loadu_si128(bytes[src_pos..].as_ptr() as *const __m128i);
                let simd_longs = _mm_shuffle_epi8(
                    simd_bytes,
                    _mm_setr_epi8(7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8),
                );
                #[allow(clippy::cast_ptr_alignment)]
                _mm_storeu_si128(
                    aligned_buffer[dst_pos..].as_mut_ptr() as *mut __m128i,
                    simd_longs,
                );
                src_pos += 16;
                dst_pos += 2;
                remaining -= 2;
            }

            // Handle remaining elements
            while remaining > 0 {
                aligned_buffer[dst_pos] = i64::from_be_bytes([
                    bytes[src_pos],
                    bytes[src_pos + 1],
                    bytes[src_pos + 2],
                    bytes[src_pos + 3],
                    bytes[src_pos + 4],
                    bytes[src_pos + 5],
                    bytes[src_pos + 6],
                    bytes[src_pos + 7],
                ]);
                src_pos += 8;
                dst_pos += 1;
                remaining -= 1;
            }
        }

        self.pos += byte_len;

        // Convert the Vec<i64> to a &'a [i64]
        // This is safe because we're leaking the memory, which will live for 'a
        let leaked_slice = aligned_buffer.leak();
        Ok(leaked_slice)
    }
}

use std::collections::HashMap;

#[derive(Debug)]
pub struct NbtCompoundView<'a, 'b> {
    tape: &'b [NbtToken<'a>],
    start: usize,
    end: usize,
    children: HashMap<&'a str, usize>,
}

impl<'a, 'b> NbtCompoundView<'a, 'b> {
    pub fn new(tape: &'b [NbtToken<'a>], start: usize) -> Self {
        let mut view = NbtCompoundView {
            tape,
            start,
            end: tape.len() - 1, // Assume the last TagEnd belongs to this compound
            children: HashMap::new(),
        };
        view.parse();
        view
    }

    fn parse(&mut self) {
        let mut i = self.start + 1; // Start after the TagStart of this compound
        while i < self.tape.len() {
            match &self.tape[i] {
                NbtToken::TagStart {
                    name: Some(tag_name),
                    ..
                } => {
                    self.children.insert(tag_name, i);
                    // Skip to the end of this tag
                    i = self.find_tag_end(i) + 1;
                }
                NbtToken::TagEnd => {
                    // This TagEnd might belong to our compound
                    self.end = i;
                    break;
                }
                _ => i += 1,
            }
        }
    }

    fn find_tag_end(&self, start: usize) -> usize {
        let mut i = start + 1;
        while i < self.tape.len() {
            match &self.tape[i] {
                NbtToken::TagEnd => return i,
                NbtToken::TagStart { .. } => {
                    // Skip nested structures
                    i = self.find_tag_end(i) + 1;
                }
                _ => i += 1,
            }
        }
        self.tape.len() - 1 // If no TagEnd found, return the last index
    }

    pub fn get(&self, name: &str) -> Option<NbtTokenView<'a, 'b>> {
        self.children
            .get(name)
            .map(|&pos| NbtTokenView::new(self.tape, pos))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, NbtTokenView<'a, 'b>)> + '_ {
        self.children
            .iter()
            .map(|(&name, &pos)| (name, NbtTokenView::new(self.tape, pos)))
    }
}

#[derive(Debug)]
pub struct NbtTokenView<'a, 'b> {
    tape: &'b [NbtToken<'a>],
    pos: usize,
}

impl<'a, 'b> NbtTokenView<'a, 'b> {
    pub fn new(tape: &'b [NbtToken<'a>], pos: usize) -> Self {
        NbtTokenView { tape, pos }
    }

    pub fn token(&self) -> &NbtToken<'a> {
        &self.tape[self.pos]
    }

    pub fn as_compound(&self) -> Option<NbtCompoundView<'a, 'b>> {
        match self.tape[self.pos] {
            NbtToken::TagStart { tag_type: 10, .. } => {
                Some(NbtCompoundView::new(self.tape, self.pos))
            }
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<NbtListView<'a, 'b>> {
        match self.tape[self.pos] {
            NbtToken::TagStart { tag_type: 9, .. } => Some(NbtListView::new(self.tape, self.pos)),
            _ => None,
        }
    }

    pub fn name(&self) -> Option<&'a str> {
        match &self.tape[self.pos] {
            NbtToken::TagStart { name, .. } => name.as_ref().map(|s| *s),
            _ => None,
        }
    }

    pub fn value(&self) -> Option<&NbtToken<'a>> {
        match &self.tape[self.pos] {
            NbtToken::TagStart { .. } => self.tape.get(self.pos + 1),
            _ => Some(&self.tape[self.pos]),
        }
    }
}

pub struct NbtListView<'a, 'b> {
    tape: &'b [NbtToken<'a>],
    start: usize,
    end: usize,
}

impl<'a, 'b> NbtListView<'a, 'b> {
    pub fn new(tape: &'b [NbtToken<'a>], start: usize) -> Self {
        let mut view = NbtListView {
            tape,
            start,
            end: start,
        };
        view.parse();
        view
    }

    fn parse(&mut self) {
        let mut i = self.start + 2; // Start after the ListStart
        while i < self.tape.len() {
            match &self.tape[i] {
                NbtToken::ListEnd => {
                    self.end = i;
                    break;
                }
                NbtToken::TagStart { .. } => {
                    i = self.find_tag_end(i) + 1;
                }
                _ => i += 1,
            }
        }
    }

    fn find_tag_end(&self, start: usize) -> usize {
        let mut i = start + 1;
        while i < self.tape.len() {
            match &self.tape[i] {
                NbtToken::TagEnd => return i,
                NbtToken::TagStart { .. } => {
                    i = self.find_tag_end(i) + 1;
                }
                _ => i += 1,
            }
        }
        self.tape.len() - 1
    }

    pub fn iter(&self) -> impl Iterator<Item = NbtTokenView<'a, 'b>> + '_ {
        (self.start + 2..self.end).filter_map(move |i| match &self.tape[i] {
            NbtToken::TagStart { .. }
            | NbtToken::Byte(..)
            | NbtToken::Short(..)
            | NbtToken::Int(..)
            | NbtToken::Long(..)
            | NbtToken::Float(..)
            | NbtToken::Double(..)
            | NbtToken::String(..)
            | NbtToken::ByteArray(..)
            | NbtToken::IntArray(..)
            | NbtToken::LongArray(..) => Some(NbtTokenView::new(self.tape, i)),
            _ => None,
        })
    }
}

#[cfg(test)]
#[test]
#[ignore]
fn basic_usage() {
    let bytes = include_bytes!("../../../../../../.etc/hello_world.nbt");

    let mut parser = NbtParser::new(bytes);
    let tapes = parser.parse().unwrap();

    let root = NbtCompoundView::new(tapes, 0);

    for (name, tag) in root.iter() {
        println!("{}: {:?}", name, tag.token());
    }
}
