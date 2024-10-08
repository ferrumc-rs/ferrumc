use std::io::Write;
use crate::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};


macro_rules! impl_for_primitives {
    ($($primitive_type:ty | $alt:ty),*) => {
        $(
            impl NetEncode for $primitive_type {
                fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> NetEncodeResult<()> {
                    writer.write_all(&self.to_be_bytes())?;
                    Ok(())
                }
            }
        
            impl NetEncode for $alt {
                fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
                    // Basically use the encode method of the primitive type,
                    // by converting alt -> primitive and then encoding.
                    (*self as $primitive_type).encode(writer, opts)
                }
            }
        
        )*
    };
}

impl_for_primitives!(
    u8 | i8,
    u16 | i16,
    u32 | i32,
    u64 | i64,
    u128 | i128,
    f32 | f64
);

impl NetEncode for bool {
    fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> NetEncodeResult<()> {
        (*self as u8).encode(writer, &NetEncodeOpts::None)
    }
}

impl NetEncode for String {
    fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> NetEncodeResult<()> {
        let len = self.len() as u32;
        len.encode(writer, &NetEncodeOpts::None)?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}