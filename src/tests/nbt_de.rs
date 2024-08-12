use std::io::Read;
use nbt_lib::{NBTDeserializeBytes, read_tag};
use crate::tests::nbt_ser::SimpleRoot;

#[test]
fn try_read() {
    // base => ../../../../ (root of the repository)
    let file_bytes = std::fs::read(".etc/bigtest.nbt").unwrap();
    // gzip decompression
    let file_bytes = match file_bytes[0..2] {
        [0x1F, 0x8B] => {
            let mut decoder = flate2::read::GzDecoder::new(&file_bytes[..]);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed).unwrap();
            decompressed
        },
        _ => file_bytes,
    };

    let root = SimpleRoot::read_from_bytes(&mut std::io::Cursor::new(file_bytes)).unwrap();

    println!("{:#?}", root);
}