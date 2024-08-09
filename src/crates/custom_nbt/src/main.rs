#![feature(portable_simd)]

use std::collections::HashMap;
use std::env::current_exe;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::nbt_spec::tags::{NamedTag, Tag};
use crate::nbt_spec::serializer::serialize_to_nbt;

mod nbt_spec;

#[tokio::main]
async fn main() {
    let mut compound = HashMap::new();
    compound.insert("name".to_string(), Tag::String("Test Entity".to_string()));
    compound.insert("pos".to_string(), Tag::List(vec![
        Tag::Double(3.14),
        Tag::Double(2.718),
        Tag::Double(1.414)
    ]));

    let named_tag = NamedTag::new("test_compound", Tag::Compound(compound));

    let mut buffer = Vec::new();
    serialize_to_nbt(&named_tag, &mut buffer).await.unwrap();

    write_to_file(&buffer);
}

fn write_to_file(buffer: &Vec<u8>) {
    let exe_path = current_exe().unwrap();
    let directory = exe_path.parent().unwrap();
    let nbt_path = directory.join("test.nbt");

    let file = File::create(nbt_path.clone()).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(buffer.as_slice()).unwrap();
}