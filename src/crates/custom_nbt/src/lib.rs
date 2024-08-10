#![feature(portable_simd)]

use std::env::current_exe;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::nbt_spec::serializer::NBTSerialize;
use crate::test::NBTTestStruct;

pub mod nbt_spec;
pub mod test;

#[test]
fn main() -> std::io::Result<()> {
    let start = std::time::Instant::now();
    let root = NBTTestStruct::new();


    let mut buffer = Vec::new();
    let named_tag = root.to_nbt();
    named_tag.serialize(&mut buffer)?;
    println!("Took {:?}", start.elapsed());
    /*nbt::to_writer(&mut buffer, &test_struct, None).unwrap();*/
    /*   let buffer = fastnbt::to_bytes(&root).unwrap();
   */
    write_to_file(&buffer);

    Ok(())
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