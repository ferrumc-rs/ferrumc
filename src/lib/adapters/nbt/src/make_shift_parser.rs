use std::str;
use std::convert::TryInto;
use hashbrown::HashMap;

/// Represents an NBT tag.
#[derive(Debug)]
pub enum NbtTag<'a> {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(&'a [u8]),
    String(&'a str),
    List(NbtList<'a>),
    Compound(NbtCompound<'a>),
    IntArray(&'a [i32]),
    LongArray(&'a [i64]),
}

/// Represents an NBT list tag.
#[derive(Debug)]
pub struct NbtList<'a> {
    element_type: u8,
    elements: Vec<NbtTag<'a>>,
}

impl<'a> NbtList<'a> {
    /// Returns the element type of the list.
    pub fn element_type(&self) -> u8 {
        self.element_type
    }

    /// Returns the elements of the list.
    pub fn elements(&self) -> &Vec<NbtTag<'a>> {
        &self.elements
    }
}

/// Represents an NBT compound tag.
#[derive(Debug)]
pub struct NbtCompound<'a> {
    tags: HashMap<&'a str, NbtTag<'a>>,
}

impl<'a> NbtCompound<'a> {
    /// Gets a tag by name.
    pub fn get(&self, name: &str) -> Option<&NbtTag<'a>> {
        self.tags.get(name)
    }
}

/// NBT parser for parsing NBT data from a byte slice.
pub struct NbtParser<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> NbtParser<'a> {
    /// Creates a new `NbtParser` from the given data slice.
    pub fn new(data: &'a [u8]) -> NbtParser<'a> {
        NbtParser { data, pos: 0 }
    }

    /// Parses the NBT data and returns the root tag.
    pub fn parse(&mut self) -> Result<(&'a str, NbtTag<'a>), String> {
        let tag_type = self.read_u8()?;
        if tag_type != 10 {
            return Err("Root tag must be a TAG_Compound".to_string());
        }
        let name = self.parse_string()?;
        let payload = self.parse_payload(tag_type)?;
        Ok((name, payload))
    }

    fn parse_string(&mut self) -> Result<&'a str, String> {
        let len = self.read_u16()? as usize;
        if self.pos + len > self.data.len() {
            return Err("Unexpected end of data while reading string".to_string());
        }
        
        let s = unsafe {
            str::from_utf8_unchecked(&self.data[self.pos..self.pos + len])
        };
        self.pos += len;
        Ok(s)
    }

    fn parse_payload(&mut self, tag_type: u8) -> Result<NbtTag<'a>, String> {
        match tag_type {
            0 => Ok(NbtTag::End),
            1 => {
                // TAG_Byte
                let v = self.read_i8()?;
                Ok(NbtTag::Byte(v))
            }
            2 => {
                // TAG_Short
                let v = self.read_i16()?;
                Ok(NbtTag::Short(v))
            }
            3 => {
                // TAG_Int
                let v = self.read_i32()?;
                Ok(NbtTag::Int(v))
            }
            4 => {
                // TAG_Long
                let v = self.read_i64()?;
                Ok(NbtTag::Long(v))
            }
            5 => {
                // TAG_Float
                let v = self.read_f32()?;
                Ok(NbtTag::Float(v))
            }
            6 => {
                // TAG_Double
                let v = self.read_f64()?;
                Ok(NbtTag::Double(v))
            }
            7 => {
                // TAG_Byte_Array
                let len = self.read_i32()? as usize;
                if self.pos + len > self.data.len() {
                    return Err("Unexpected end of data while reading TAG_Byte_Array".to_string());
                }
                let v = &self.data[self.pos..self.pos + len];
                self.pos += len;
                Ok(NbtTag::ByteArray(v))
            }
            8 => {
                // TAG_String
                let s = self.parse_string()?;
                Ok(NbtTag::String(s))
            }
            9 => {
                // TAG_List
                let item_type = self.read_u8()?;
                let len = self.read_i32()? as usize;
                let mut elements = Vec::with_capacity(len);
                for _ in 0..len {
                    let item = self.parse_payload(item_type)?;
                    elements.push(item);
                }
                Ok(NbtTag::List(NbtList {
                    element_type: item_type,
                    elements,
                }))
            }
            10 => {
                // TAG_Compound
                let mut tags = HashMap::new();
                loop {
                    let tag_type = self.read_u8()?;
                    if tag_type == 0 {
                        break;
                    }
                    let name = self.parse_string()?;
                    let payload = self.parse_payload(tag_type)?;
                    // tags.push((name, payload));
                    tags.insert(name, payload);
                }
                Ok(NbtTag::Compound(NbtCompound { tags }))
            }
            11 => {
                // TAG_Int_Array
                let len = self.read_i32()? as usize;
                let array = self.read_i32_array(len)?;
                Ok(NbtTag::IntArray(array))
            }
            12 => {
                // TAG_Long_Array
                let len = self.read_i32()? as usize;
                let array = self.read_i64_array(len)?;
                Ok(NbtTag::LongArray(array))
            }
            _ => Err(format!("Unknown tag type: {}", tag_type)),
        }
    }

    /// Reads a u8 from the data.
    fn read_u8(&mut self) -> Result<u8, String> {
        if self.pos >= self.data.len() {
            return Err("Unexpected end of data while reading u8".to_string());
        }
        let v = self.data[self.pos];
        self.pos += 1;
        Ok(v)
    }

    /// Reads an i8 from the data.
    fn read_i8(&mut self) -> Result<i8, String> {
        Ok(self.read_u8()? as i8)
    }

    /// Reads a big-endian u16 from the data.
    fn read_u16(&mut self) -> Result<u16, String> {
        if self.pos + 2 > self.data.len() {
            return Err("Unexpected end of data while reading u16".to_string());
        }
        let v = u16::from_be_bytes(
            self.data[self.pos..self.pos + 2]
                .try_into()
                .map_err(|_| "Failed to read u16".to_string())?,
        );
        self.pos += 2;
        Ok(v)
    }

    /// Reads a big-endian i16 from the data.
    fn read_i16(&mut self) -> Result<i16, String> {
        Ok(self.read_u16()? as i16)
    }

    /// Reads a big-endian u32 from the data.
    fn read_u32(&mut self) -> Result<u32, String> {
        if self.pos + 4 > self.data.len() {
            return Err("Unexpected end of data while reading u32".to_string());
        }
        let v = u32::from_be_bytes(
            self.data[self.pos..self.pos + 4]
                .try_into()
                .map_err(|_| "Failed to read u32".to_string())?,
        );
        self.pos += 4;
        Ok(v)
    }

    /// Reads a big-endian i32 from the data.
    fn read_i32(&mut self) -> Result<i32, String> {
        Ok(self.read_u32()? as i32)
    }

    /// Reads a big-endian u64 from the data.
    fn read_u64(&mut self) -> Result<u64, String> {
        if self.pos + 8 > self.data.len() {
            return Err("Unexpected end of data while reading u64".to_string());
        }
        let v = u64::from_be_bytes(
            self.data[self.pos..self.pos + 8]
                .try_into()
                .map_err(|_| "Failed to read u64".to_string())?,
        );
        self.pos += 8;
        Ok(v)
    }

    /// Reads a big-endian i64 from the data.
    fn read_i64(&mut self) -> Result<i64, String> {
        Ok(self.read_u64()? as i64)
    }

    /// Reads a big-endian f32 from the data.
    fn read_f32(&mut self) -> Result<f32, String> {
        let bits = self.read_u32()?;
        Ok(f32::from_bits(bits))
    }

    /// Reads a big-endian f64 from the data.
    fn read_f64(&mut self) -> Result<f64, String> {
        let bits = self.read_u64()?;
        Ok(f64::from_bits(bits))
    }

    /// Reads an array of i32 from the data.
    fn read_i32_array(&mut self, len: usize) -> Result<&'a [i32], String> {
        let byte_len = len * 4;
        if self.pos + byte_len > self.data.len() {
            return Err("Unexpected end of data while reading i32 array".to_string());
        }
        let bytes = &self.data[self.pos..self.pos + byte_len];
        if bytes.as_ptr().align_offset(std::mem::align_of::<i32>()) != 0 {
            return Err("Data is not properly aligned for i32 array".to_string());
        }
        #[allow(clippy::cast_ptr_alignment)]
        let array = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const i32, len) };
        self.pos += byte_len;
        Ok(array)
    }

    /// Reads an array of i64 from the data.
    fn read_i64_array(&mut self, len: usize) -> Result<&'a [i64], String> {
        let byte_len = len * 8;
        if self.pos + byte_len > self.data.len() {
            return Err("Unexpected end of data while reading i64 array".to_string());
        }
        let bytes = &self.data[self.pos..self.pos + byte_len];
        if bytes.as_ptr().align_offset(std::mem::align_of::<i64>()) != 0 {
            return Err("Data is not properly aligned for i64 array".to_string());
        }
        #[allow(clippy::cast_ptr_alignment)]
        let array = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const i64, len) };
        self.pos += byte_len;
        Ok(array)
    }
}

#[test]
fn basic_usage() {
    let bytes = include_bytes!("../../../../../.etc/hello_world.nbt");

    let mut parser = NbtParser::new(bytes);
    let (name, tag) = parser.parse().unwrap();
    println!("Root Name: {}", name);
    println!("Tag: {:?}", tag);
}

