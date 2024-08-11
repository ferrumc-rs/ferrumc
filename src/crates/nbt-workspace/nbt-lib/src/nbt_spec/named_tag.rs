
use std::io::Write;
use crate::nbt_spec::serializer::NBTSerialize;
use crate::nbt_spec::tag::Tag;
use crate::NBTResult;

#[derive(Debug)]
pub struct NamedTag {
    pub name: String,
    pub tag: Tag,
}

impl NamedTag {
    pub fn new(name: String, tag: Tag) -> NamedTag {
        NamedTag {
            name,
            tag,
        }
    }
}

impl NBTSerialize for NamedTag {
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        writer.write_all(&[self.tag.get_type_id()])?;
        writer.write_all(&(self.name.len() as u16).to_be_bytes())?;
        writer.write_all(self.name.as_bytes())?;
        self.tag.write(writer)?;
        Ok(())
    }
}