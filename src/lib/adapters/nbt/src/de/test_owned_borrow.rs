use crate::errors::NBTError;
use std::collections::HashMap;
use std::io::Read;
use std::str;

/// Represents a token in the NBT tape.
#[derive(Debug, PartialEq, Clone)]
pub enum NbtToken {
    TagStart { tag_type: u8, name: String },
    TagEnd,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    ListStart { element_type: u8, length: usize },
    ListEnd,
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

/// NBT parser using a tape-based approach.
/// Please use Clone carefully.
#[derive(Debug, Clone)]
pub struct NbtParser {
    data: Vec<u8>,
    pos: usize,
    tape: Vec<NbtToken>,
}

impl NbtParser {
    /// Creates a new `NbtParser` from the given data vector.
    pub fn new(data: Vec<u8>) -> NbtParser {
        let len = data.len();
        NbtParser {
            data,
            pos: 0,
            tape: Vec::with_capacity(len + 1024), // Preallocate with an estimate
        }
    }

    /// Parses the NBT data and returns the tape of tokens.
    pub fn parse(&mut self) -> Result<&[NbtToken], NBTError> {
        if Self::is_compressed(&self.data) {
            return Err(NBTError::CompressedData);
        }

        let tag_type = self.read_u8()?;
        if tag_type != 10 {
            return Err(NBTError::InvalidRootCompound(tag_type));
        }
        let name = self.parse_string()?;
        self.tape.push(NbtToken::TagStart { tag_type, name: name.clone() });
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

    /// Checks if the data is compressed (gzip).
    pub(crate) fn is_compressed(data: &[u8]) -> bool {
        data.starts_with(&[0x1F, 0x8B])
    }

    /// Parses a string from the data.
    fn parse_string(&mut self) -> Result<String, NBTError> {
        let len = self.read_u16()? as usize;
        if self.pos + len > self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }

        // SAFETY: We just checked that the data is long enough and is valid UTF-8.
        let s = unsafe { str::from_utf8_unchecked(&self.data[self.pos..self.pos + len]) }.to_string();
        self.pos += len;
        Ok(s)
    }

    /// Parses the payload based on the tag type.
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
                let array = self.read_byte_array(len)?;
                self.tape.push(NbtToken::ByteArray(array));
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
                        name: name.clone(),
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
            _ => Err(NBTError::InvalidTagType(tag_type)),
        }
    }

    /// Reads a single byte from the data.
    #[inline(always)]
    fn read_u8(&mut self) -> Result<u8, NBTError> {
        if self.pos >= self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }
        let v = self.data[self.pos];
        self.pos += 1;
        Ok(v)
    }

    /// Reads a signed byte from the data.
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
        let v = u16::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
        ]);
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
        let v = u32::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
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
        let v = u64::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
            self.data[self.pos + 4],
            self.data[self.pos + 5],
            self.data[self.pos + 6],
            self.data[self.pos + 7],
        ]);
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

    /// Reads a byte array from the data.
    fn read_byte_array(&mut self, len: usize) -> Result<Vec<u8>, NBTError> {
        if self.pos + len > self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }
        let array = self.data[self.pos..self.pos + len].to_vec();
        self.pos += len;
        Ok(array)
    }

    /// Reads an array of i32 from the data.
    fn read_i32_array(&mut self, len: usize) -> Result<Vec<i32>, NBTError> {
        let byte_len = len * std::mem::size_of::<i32>();
        if self.pos + byte_len > self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }
        let mut array = Vec::with_capacity(len);

        // Efficiently parse i32 array
        for chunk in self.data[self.pos..self.pos + byte_len].chunks_exact(4) {
            let value = i32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            array.push(value);
        }

        self.pos += byte_len;
        Ok(array)
    }

    /// Reads an array of i64 from the data.
    fn read_i64_array(&mut self, len: usize) -> Result<Vec<i64>, NBTError> {
        let byte_len = len * std::mem::size_of::<i64>();
        if self.pos + byte_len > self.data.len() {
            return Err(NBTError::UnexpectedEndOfData);
        }
        let mut array = Vec::with_capacity(len);

        // Efficiently parse i64 array
        for chunk in self.data[self.pos..self.pos + byte_len].chunks_exact(8) {
            let value = i64::from_be_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3],
                chunk[4], chunk[5], chunk[6], chunk[7],
            ]);
            array.push(value);
        }

        self.pos += byte_len;
        Ok(array)
    }
}

#[derive(Debug)]
pub struct NbtCompoundView<'a> {
    tape: &'a [NbtToken],
    start: usize,
    end: usize,
    children: HashMap<String, usize>,
}

impl<'a> NbtCompoundView<'a> {
    pub fn new(tape: &'a [NbtToken], start: usize) -> Self {
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
                NbtToken::TagStart { name, .. } => {
                    self.children.insert(name.clone(), i);
                    // Skip to the end of this tag
                    i = self.find_tag_end(i) + 1;
                }
                NbtToken::TagEnd => {
                    // This TagEnd belongs to our compound
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
                    i = self.find_tag_end(i) + 1;
                }
                _ => i += 1,
            }
        }
        self.tape.len() - 1 // If no TagEnd found, return the last index
    }

    pub fn get(&self, name: &str) -> Option<NbtTokenView<'a>> {
        self.children
            .get(name)
            .map(|&pos| NbtTokenView::new(self.tape, pos))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, NbtTokenView<'a>)> + '_ {
        self.children.iter().map(move |(name, &pos)| (name.as_str(), NbtTokenView::new(self.tape, pos)))
    }
}

#[derive(Debug)]
pub struct NbtTokenView<'a> {
    tape: &'a [NbtToken],
    pos: usize,
}

impl<'a> NbtTokenView<'a> {
    pub fn new(tape: &'a [NbtToken], pos: usize) -> Self {
        NbtTokenView { tape, pos }
    }

    pub fn token(&self) -> &NbtToken {
        &self.tape[self.pos]
    }

    pub fn as_compound(&self) -> Option<NbtCompoundView<'a>> {
        match &self.tape[self.pos] {
            NbtToken::TagStart { tag_type: 10, .. } => Some(NbtCompoundView::new(self.tape, self.pos)),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<NbtListView<'a>> {
        match &self.tape[self.pos] {
            NbtToken::TagStart { tag_type: 9, .. } => Some(NbtListView::new(self.tape, self.pos)),
            _ => None,
        }
    }

    pub fn name(&self) -> Option<&str> {
        match &self.tape[self.pos] {
            NbtToken::TagStart { name, .. } => Some(name.as_str()),
            _ => None,
        }
    }

    pub fn value(&self) -> Option<&NbtToken> {
        match &self.tape[self.pos] {
            NbtToken::TagStart { .. } => self.tape.get(self.pos + 1),
            _ => Some(&self.tape[self.pos]),
        }
    }
}

pub struct NbtListView<'a> {
    tape: &'a [NbtToken],
    start: usize,
    end: usize,
}

impl<'a> NbtListView<'a> {
    pub fn new(tape: &'a [NbtToken], start: usize) -> Self {
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

    pub fn iter(&self) -> impl Iterator<Item = NbtTokenView<'a>> + '_ {
        (self.start + 2..self.end).filter_map(move |i| match &self.tape[i] {
            NbtToken::TagStart { .. }
            | NbtToken::Byte(_)
            | NbtToken::Short(_)
            | NbtToken::Int(_)
            | NbtToken::Long(_)
            | NbtToken::Float(_)
            | NbtToken::Double(_)
            | NbtToken::String(_)
            | NbtToken::ByteArray(_)
            | NbtToken::IntArray(_)
            | NbtToken::LongArray(_) => Some(NbtTokenView::new(self.tape, i)),
            _ => None,
        })
    }

    pub fn len(&self) -> usize {
        self.end - self.start - 2
    }
}
