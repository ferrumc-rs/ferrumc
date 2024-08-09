use std::io;
use std::io::Write;

pub enum Tag {
    Byte(i8)
}

pub trait NBTSerialize {
    fn nbt_serialize<W: Write + Unpin>(&self , writer: &mut W) -> io::Result<()>;
}

impl NBTSerialize for bool {
    fn nbt_serialize<W: Write + Unpin>(&self, writer: &mut W) -> io::Result<()> {
        // TAG_Byte
        writer.write_all(&[1])?;

        // Name length (0 for now, we're not using names in this simple version)
        writer.write_all(&[0, 0])?;

        // Value
        writer.write_all(&[*self as u8])?;

        Ok(())
    }
}

pub fn serialize_to_nbt<T: NBTSerialize, W: Write + Unpin>(value: &T, writer: &mut W) -> io::Result<()> {
    value.nbt_serialize(writer)
}

