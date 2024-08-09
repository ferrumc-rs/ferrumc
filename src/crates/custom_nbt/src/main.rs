use std::env::current_exe;
use std::io::Write;
use crate::nbt_spec::serialize_to_nbt;

mod nbt_spec;

fn main() {
    let mut buffer = Vec::new();
    serialize_to_nbt(&true, &mut buffer).unwrap();

    // write it to a file
    let exe_path = current_exe().unwrap();
    let directory = exe_path.parent().unwrap();
    let nbt_path = directory.join("test.nbt");

    let file = std::fs::File::create(nbt_path.clone()).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(&buffer).unwrap();

    println!("Wrote to {:?}", nbt_path);
}
