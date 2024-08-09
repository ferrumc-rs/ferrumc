#![feature(portable_simd)]

use std::collections::HashMap;
use std::env::current_exe;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::nbt_spec::serializer::NBTSerialize;

pub mod nbt_spec;
pub mod test;

#[test]
fn main() -> std::io::Result<()> {
    let mut buffer = Vec::new();

    let test_struct = test::NBTTestStruct::new();
    let named_tag = test_struct.to_nbt();
    named_tag.serialize(&mut buffer)?;

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
}