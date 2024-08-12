#![allow(dead_code)]
use std::collections::HashMap;
use std::io::{Cursor, Read};

#[derive(Debug)]
enum NBTTag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NBTTag>),
    Compound(HashMap<String, NBTTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[test]
fn try_read() {
    // base => ../../../../
    let file_bytes = std::fs::read("../../../../.etc/nbt_lib_validation.nbt").unwrap();

    let nbt_tag = deserialize_simple(file_bytes);

    println!("{:#?}", nbt_tag);
}

fn deserialize_simple(bytes: Vec<u8>) -> NBTTag {
    read_tag(&mut Cursor::new(bytes))
}

fn read_tag(cursor:&mut Cursor<Vec<u8>>) -> NBTTag {
    let mut compound_data: HashMap<String, NBTTag> = HashMap::new();

    while cursor.position() < cursor.get_ref().len() as u64 {
        let tag_type: u8 = cursor.read_i8() as u8;
        if tag_type == 0  {
            break;
        }
        let name: String = cursor.read_nbt_string();

        println!("Reading tag: {} ({})", name, tag_type);

        let mut tag: NBTTag = NBTTag::End;

        match tag_type {
            0 => tag = NBTTag::End,
            1 => tag = NBTTag::Byte(cursor.read_i8()),
            2 => tag = NBTTag::Short(cursor.read_i16()),
            3 => tag = NBTTag::Int(cursor.read_i32()),
            4 => tag = NBTTag::Long(cursor.read_i64()),
            5 => tag = NBTTag::Float(cursor.read_f32()),
            6 => tag = NBTTag::Double(cursor.read_f64()),
            8 => tag = NBTTag::String(cursor.read_nbt_string()),
            10 => tag = read_tag(cursor),
            _ => {
                println!("Unknown tag type: {}", tag_type);
            }
        }

        if let NBTTag::End = tag {
            println!("Ending {}", name);
            break;
        }

        println!("{} = {:?}", name, tag);
        compound_data.insert(name, tag);
    }

    println!("End of tag");

    NBTTag::Compound(compound_data)
}

trait CursorExt {
    fn read_i8(&mut self) -> i8;
    fn read_i16(&mut self) -> i16;
    fn read_i32(&mut self) -> i32;
    fn read_i64(&mut self) -> i64;
    fn read_f32(&mut self) -> f32;
    fn read_f64(&mut self) -> f64;
    fn read_nbt_string(&mut self) -> String;
    fn read_string_with_len(&mut self, len: u16) -> String;
}

impl CursorExt for Cursor<Vec<u8>> {
    fn read_i8(&mut self) -> i8 {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).unwrap();
        i8::from_be_bytes(buf)
    }

    fn read_i16(&mut self) -> i16 {
        let mut buf = [0; 2];
        self.read_exact(&mut buf).unwrap();
        i16::from_be_bytes(buf)
    }

    fn read_i32(&mut self) -> i32 {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).unwrap();
        i32::from_be_bytes(buf)
    }

    fn read_i64(&mut self) -> i64 {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).unwrap();
        i64::from_be_bytes(buf)
    }

    fn read_f32(&mut self) -> f32 {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).unwrap();
        f32::from_be_bytes(buf)
    }

    fn read_f64(&mut self) -> f64 {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).unwrap();
        f64::from_be_bytes(buf)
    }

    fn read_nbt_string(&mut self) -> String {
        let len = self.read_i16() as u16;
        self.read_string_with_len(len)
    }

    fn read_string_with_len(&mut self, len: u16) -> String {
        let mut buf = vec![0; len as usize];
        self.read(&mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }
}