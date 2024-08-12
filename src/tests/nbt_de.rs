use nbt_lib::{NBTDeserializeBytes};
use crate::tests::nbt_ser::SimpleRoot;

#[test]
fn try_read() {
    // base => ../../../../ (root of the repository)
    let file_bytes = std::fs::read(".etc/nbt_lib_validation.nbt").unwrap();

    let root = SimpleRoot::read_from_bytes(&mut std::io::Cursor::new(file_bytes)).unwrap();

    println!("{:#?}", root);
}