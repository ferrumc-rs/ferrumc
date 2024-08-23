use std::io::Write;
use impls::NBTFieldType;
use crate::NBTResult;

pub mod impls;
pub mod tag_types;
pub mod nbt_tag_to_writer;


pub trait NBTSerialize: NBTFieldType {
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()>;
}

pub trait NBTAnonymousType {
    fn tag_type() -> u8;
}


/// Just a marker trait to identify NBTCompound.
/// This is used to implement network Serialize for : NBTSerialize + NBTCompoundMarker
pub trait NBTCompoundMarker {}