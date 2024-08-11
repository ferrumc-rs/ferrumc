use std::io::Write;
use crate::NBTResult;

pub trait NBTSerialize {
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()>;
}