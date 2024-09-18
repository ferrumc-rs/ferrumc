mod r#impl;

trait NBTSerializable {
    fn serialize(&self, but: &mut Vec<u8>, options: &NBTSerializeOptions);
    fn id() -> u8;
}

/// Options for serializing NBT data.
/// To simplify root serialization.
enum NBTSerializeOptions {
    None,
}
