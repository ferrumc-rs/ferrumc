use crate::encode::AsyncWrite;
use crate::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use crate::net_types::var_int::VarInt;
use std::collections::HashMap;
use std::io::Write;
use tokio::io::AsyncWriteExt;

macro_rules! impl_for_primitives {
    ($($primitive_type:ty $(| $alt:ty)?),*) => {
        $(
            impl NetEncode for $primitive_type {
                fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> NetEncodeResult<()> {
                    writer.write_all(&self.to_be_bytes())?;
                    Ok(())
                }
                async fn encode_async<W: tokio::io::AsyncWrite + Unpin>(&self, writer: &mut W, _: &NetEncodeOpts) -> NetEncodeResult<()> {
                    writer.write_all(&self.to_be_bytes()).await?;
                    Ok(())
                }
            }

            $(
                impl NetEncode for $alt {
                    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
                        (*self as $primitive_type).encode(writer, opts)
                    }
                    async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
                        (*self as $primitive_type).encode_async(writer, opts).await
                    }
                }
            )?
        )*
    };
}

impl_for_primitives!(
    u8 | i8,
    u16 | i16,
    u32 | i32,
    u64 | i64,
    u128 | i128,
    usize | isize,
    f32,
    f64
);

impl NetEncode for bool {
    fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> NetEncodeResult<()> {
        (*self as u8).encode(writer, &NetEncodeOpts::None)
    }
    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        (*self as u8)
            .encode_async(writer, &NetEncodeOpts::None)
            .await
    }
}

impl NetEncode for String {
    fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> NetEncodeResult<()> {
        self.as_str().encode(writer, &NetEncodeOpts::None)
    }
    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        self.as_str()
            .encode_async(writer, &NetEncodeOpts::None)
            .await
    }
}

impl NetEncode for &str {
    fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> NetEncodeResult<()> {
        let len: VarInt = VarInt::new(self.len() as i32);
        len.encode(writer, &NetEncodeOpts::None)?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        let len: VarInt = VarInt::new(self.len() as i32);
        len.encode_async(writer, &NetEncodeOpts::None).await?;
        writer.write_all(self.as_bytes()).await?;
        Ok(())
    }
}

impl<T> NetEncode for Vec<T>
where
    T: NetEncode,
{
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        if matches!(opts, NetEncodeOpts::SizePrefixed) {
            let len: VarInt = VarInt::new(self.len() as i32);
            len.encode(writer, opts)?;
        }

        for item in self {
            item.encode(writer, opts)?;
        }
        Ok(())
    }
    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        if matches!(opts, NetEncodeOpts::SizePrefixed) {
            let len: VarInt = VarInt::new(self.len() as i32);
            len.encode_async(writer, opts).await?;
        }

        for item in self {
            item.encode_async(writer, opts).await?;
        }
        Ok(())
    }
}

impl NetEncode for &[u8] {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        if matches!(opts, NetEncodeOpts::SizePrefixed) {
            let len: VarInt = VarInt::new(self.len() as i32);
            len.encode(writer, opts)?;
        }

        writer.write_all(self)?;
        Ok(())
    }
    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        if matches!(opts, NetEncodeOpts::SizePrefixed) {
            let len: VarInt = VarInt::new(self.len() as i32);
            len.encode_async(writer, opts).await?;
        }

        writer.write_all(self).await?;
        Ok(())
    }
}

impl NetEncode for &[&str] {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        if matches!(opts, NetEncodeOpts::SizePrefixed) {
            let len: VarInt = VarInt::new(self.len() as i32);
            len.encode(writer, opts)?;
        }

        for item in *self {
            item.encode(writer, opts)?;
        }
        Ok(())
    }
    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        if matches!(opts, NetEncodeOpts::SizePrefixed) {
            let len: VarInt = VarInt::new(self.len() as i32);
            len.encode_async(writer, opts).await?;
        }

        for item in *self {
            item.encode_async(writer, opts).await?;
        }
        Ok(())
    }
}

impl<T: NetEncode> NetEncode for Option<T> {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        match self {
            Some(value) => value.encode(writer, opts),
            None => Ok(()),
        }
    }
    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        match self {
            Some(value) => value.encode_async(writer, opts).await,
            None => Ok(()),
        }
    }
}

impl<K, V> NetEncode for HashMap<K, V>
where
    K: NetEncode,
    V: NetEncode,
{
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        let len: VarInt = VarInt::new(self.len() as i32);
        len.encode(writer, opts)?;

        for (key, value) in self {
            key.encode(writer, opts)?;
            value.encode(writer, opts)?;
        }
        Ok(())
    }
    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        let len: VarInt = VarInt::new(self.len() as i32);
        len.encode_async(writer, opts).await?;

        for (key, value) in self {
            key.encode_async(writer, opts).await?;
            value.encode_async(writer, opts).await?;
        }
        Ok(())
    }
}
