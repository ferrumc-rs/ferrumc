use std::{io, slice};
use std::io::Write;

use crate::{NBTResult, NBTSerialize, NBTTag};
use crate::nbt_spec::serializer::impls::NBTFieldType;
use crate::nbt_spec::serializer::tag_types::TAG_END;

impl NBTTag {
    pub fn tag_type(&self) -> u8 {
        match self {
            NBTTag::End => 0,
            NBTTag::Byte(_) => 1,
            NBTTag::Short(_) => 2,
            NBTTag::Int(_) => 3,
            NBTTag::Long(_) => 4,
            NBTTag::Float(_) => 5,
            NBTTag::Double(_) => 6,
            NBTTag::ByteArray(_) => 7,
            NBTTag::String(_) => 8,
            NBTTag::List(_) => 9,
            NBTTag::Compound(_) => 10,
            NBTTag::IntArray(_) => 11,
            NBTTag::LongArray(_) => 12,
        }
    }
}

impl NBTFieldType for NBTTag {
    fn tag_type(&self) -> u8 {
        self.tag_type()
    }
}

impl NBTSerialize for NBTTag {
    fn nbt_serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        write_tag(self, writer)
    }
}

fn write_tag<W: Write>(tag: &NBTTag, writer: &mut W) -> NBTResult<()> {
    match tag {
        NBTTag::End => Ok(()),
        NBTTag::Byte(v) => writer.write_all(&[*v as u8]),
        NBTTag::Short(v) => writer.write_all(&v.to_be_bytes()),
        NBTTag::Int(v) => writer.write_all(&v.to_be_bytes()),
        NBTTag::Long(v) => writer.write_all(&v.to_be_bytes()),
        NBTTag::Float(v) => writer.write_all(&v.to_be_bytes()),
        NBTTag::Double(v) => writer.write_all(&v.to_be_bytes()),
        NBTTag::ByteArray(v) => {
            writer.write_all(&(v.len() as i32).to_be_bytes())?;
            write_i8_vec_unsafe(writer, v)
        }
        NBTTag::String(v) => {
            let bytes = v.as_bytes();
            writer.write_all(&(bytes.len() as i16).to_be_bytes())?;
            writer.write_all(bytes)
        }
        NBTTag::List(v) => {
            let tag_type = v.first().map(|t| t.tag_type()).unwrap_or(TAG_END);
            writer.write_all(&[tag_type])?;
            writer.write_all(&(v.len() as i32).to_be_bytes())?;
            for tag in v {
                write_tag(tag, writer)?;
            }
            Ok(())
        }
        NBTTag::Compound(v) => {
            for (name, tag) in v {
                write_tag_named(name, tag, writer)?;
            }
            writer.write_all(&[TAG_END])?;
            Ok(())
        }
        NBTTag::IntArray(v) => {
            writer.write_all(&(v.len() as i32).to_be_bytes())?;
            for i in v {
                writer.write_all(&i.to_be_bytes())?;
            }
            Ok(())
        }
        NBTTag::LongArray(v) => {
            writer.write_all(&(v.len() as i32).to_be_bytes())?;
            for i in v {
                writer.write_all(&i.to_be_bytes())?;
            }
            Ok(())
        }
    }
    .map_err(|e| e.into())
}
fn write_i8_vec_unsafe<W: Write>(writer: &mut W, v: &Vec<i8>) -> io::Result<()> {
    let ptr = v.as_ptr() as *const u8;
    let slice = unsafe { slice::from_raw_parts(ptr, v.len()) };
    writer.write_all(slice)
}
fn write_tag_named<W: Write>(name: &str, tag: &NBTTag, writer: &mut W) -> NBTResult<()> {
    writer.write_all(&[tag.tag_type()])?;
    writer.write_all(&(name.len() as i16).to_be_bytes())?;
    writer.write_all(name.as_bytes())?;
    write_tag(tag, writer)
}
