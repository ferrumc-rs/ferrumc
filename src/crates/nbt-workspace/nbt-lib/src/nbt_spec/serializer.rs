use std::io::Write;
use impls::NBTTagIdentity;
use crate::NBTResult;

pub mod impls;
pub mod tag_types;

pub trait NBTSerialize: NBTTagIdentity {
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()>;
}