#![feature(portable_simd)]
#![allow(dead_code)]

use std::env::current_exe;
use std::fs::File;
use std::hint::black_box;
use std::io::{BufWriter, Write};

use crate::nbt_spec::serializer::NBTSerialize;
use crate::test::NBTTestStruct;

pub mod nbt_spec;
pub mod test;

#[test]
fn main() -> std::io::Result<()> {
    let start = std::time::Instant::now();
    let buffer = black_box(with_fastnbt());
    let fasnbt_time= start.elapsed();

    let start = std::time::Instant::now();
    let buffer_v2 = black_box(with_custom_nbt());
    let custom_time = start.elapsed();

    println!("fastnbt: {:?}", fasnbt_time);
    println!("custom_nbt: {:?}", custom_time);

    Ok(())
}

fn with_fastnbt() -> Vec<u8> {
    let nbttest_struct = NBTTestStruct::new();
    let buffer = fastnbt::to_bytes(&nbttest_struct).unwrap();
    buffer
}
fn with_custom_nbt() -> Vec<u8> {
    let nbttest_struct = NBTTestStruct::new();
    let mut buffer = Vec::new();
    let named_tag = nbttest_struct.to_nbt();
    named_tag.serialize(&mut buffer).unwrap();

    buffer
}
fn write_to_file(buffer: &Vec<u8>) {
    let exe_path = current_exe().unwrap();
    let directory = exe_path.parent().unwrap();
    let nbt_path = directory.join("test.nbt");

    let file = File::create(nbt_path.clone()).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(buffer.as_slice()).unwrap();

    println!("Wrote NBT data to {:?}", nbt_path);
}