use std::io::Write;

mod r#impl;

pub trait NBTSerializable {
    fn serialize<W: Write>(&self, buf: &mut W, options: &NBTSerializeOptions<'_>);
    fn id() -> u8;
}

/// Options for serializing NBT data.
/// To simplify root serialization.
#[derive(PartialEq, Debug)]
pub enum NBTSerializeOptions<'a> {
    None,
    WithHeader(&'a str),
    Network,
    Flatten,
}
