use crate::de::NBTTape;

#[test]
fn test_basic() {
    let data = include_bytes!("../../../../../../.etc/hello_world.nbt");

    let _tape = NBTTape::read_tag(data);
}