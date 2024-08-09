use crate::nbt_spec::serializer::NBTSerialize;
use crate::nbt_spec::buffer::NBTBuffer;
use crate::nbt_spec::tag_types;
use tokio::io::{AsyncWrite};
use std::io;

macro_rules! impl_nbt_serialize {
($type:ty, $tag:expr, $write_method:ident) => {
    impl NBTSerialize for $type {
        async fn nbt_serialize<W: AsyncWrite + Unpin + Send>(&self, name: &str, writer: &mut W) -> io::Result<()> {
            let mut buffer = NBTBuffer::new();
            buffer.write_u8($tag);
            buffer.write_string(name);
            buffer.$write_method(*self);
            buffer.flush(writer).await
        }
    }
};
}

impl_nbt_serialize!(i8, tag_types::TAG_BYTE, write_i8);
impl_nbt_serialize!(u8, tag_types::TAG_BYTE, write_u8);
impl_nbt_serialize!(bool, tag_types::TAG_BYTE, write_bool);
impl_nbt_serialize!(i16, tag_types::TAG_SHORT, write_i16);
impl_nbt_serialize!(u16, tag_types::TAG_SHORT, write_u16);
impl_nbt_serialize!(i32, tag_types::TAG_INT, write_i32);
impl_nbt_serialize!(i64, tag_types::TAG_LONG, write_i64);
impl_nbt_serialize!(f32, tag_types::TAG_FLOAT, write_f32);
impl_nbt_serialize!(f64, tag_types::TAG_DOUBLE, write_f64);
