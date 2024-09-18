use std::collections::HashMap;

pub struct NBTTape<'a> {
    cursor: NBTCursor<'a>,
    tokens: Vec<NBTToken<'a>>,
    index_map: HashMap<&'a str, usize>,
}

pub struct NBTCursor<'a> {
    data: &'a [u8],
    position: usize,
}

pub enum NBTToken<'a> {
    Byte(&'a str, i8),
    StartCompound(&'a str, usize), // usize is the index of the starting position of the data in the tape.
    EndCompound,
}

impl<'a> NBTTape<'a> {
    pub fn read_tag(data: &'a [u8]) -> NBTTape<'a> {
        let mut tape = NBTTape {
            tokens: Vec::new(),
            index_map: HashMap::new(),
            cursor: NBTCursor::new(data),
        };

        let root_tag = tape.cursor.read_u8();
        println!("Root tag: {}", root_tag);
        let (_size, name) = tape.cursor.read_string();
        
        println!("root tag name: {}", name);

        unimplemented!()
    }
    
    
    pub fn record(&mut self, name: &'a str, token: NBTToken<'a>) {
        let index = self.tokens.len();
        self.index_map.insert(name, index);
        self.tokens.push(token);
    }
    
}
impl<'a> NBTCursor<'a> {
    pub fn new(data: &'a [u8]) -> NBTCursor<'a> {
        NBTCursor {
            data,
            position: 0,
        }
    }
    pub fn wind_forward(&mut self, amount: usize) {
        self.position += amount;
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn read_bytes(&mut self, amount: usize) -> &'a [u8] {
        let start = self.position;
        self.wind_forward(amount);
        &self.data[start..self.position]
    }
    pub fn read_u8(&mut self) -> u8 {
        let bytes = self.read_bytes(1);
        bytes[0]
    }
    pub fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }
    pub fn read_u16(&mut self) -> u16 {
        let bytes = self.read_bytes(2);
        u16::from_be_bytes([bytes[0], bytes[1]])
    }
    pub fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }
    pub fn read_u32(&mut self) -> u32 {
        let bytes = self.read_bytes(4);
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
    pub fn read_i32(&mut self) -> i32 {
        self.read_u32() as i32
    }
    pub fn read_u64(&mut self) -> u64 {
        let b = self.read_bytes(8);
        u64::from_be_bytes([
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
        ])
    }
    pub fn read_i64(&mut self) -> i64 {
        self.read_u64() as i64
    }
    
    pub fn read_string(&mut self) -> (u16, &'a str) {
        let length = self.read_u16();
        let bytes = self.read_bytes(length as usize);
        let string = std::str::from_utf8(bytes).unwrap();
        (length, string)
    }
}