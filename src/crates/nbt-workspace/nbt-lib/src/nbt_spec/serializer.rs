use std::io;
use std::io::Write;


pub trait NBTSerialize {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()>;
}