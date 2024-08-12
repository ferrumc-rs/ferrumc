#![allow(dead_code)]
use std::io::{Cursor};
use crate::NBTResult;
use cursor_ext::CursorExt;
use crate::nbt_spec::deserializer::nbt_tag_reader::NBTTag;

// pub(self) means that the module is accessible from the children.
pub(self) mod cursor_ext;
pub(self) mod bytes_impls;
pub(self) mod impls;
pub mod nbt_tag_reader;


pub trait NBTDeserializeBytes {
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self>
    where Self: Sized;
}
pub trait NBTDeserialize {
    fn read_from(nbt: NBTTag) -> NBTResult<Self>
    where Self: Sized;
}
