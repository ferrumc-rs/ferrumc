use std::io::Write;
use crate::nbt_spec::impls::NBTTag;
use crate::NBTResult;

pub trait NBTSerialize: NBTTag {
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()>;
}