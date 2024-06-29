use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncWrite, AsyncWriteExt};
use crate::encoding::position::Position;

use crate::encoding::varint::{read_varint, VarInt, write_varint};
use crate::encoding::varlong::{read_varlong, Varlong, write_varlong};
use crate::error::Error;

pub trait Decode {
    #[allow(unused)]
    #[allow(async_fn_in_trait)]
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin;
}

impl Decode for bool {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 1];
        bytes
            .read_exact(&mut buf)
            .await
            .map_err(|_| Error::Generic("Failed to read bool".parse().unwrap()))?;
        Ok(Box::from(buf[0] != 0))
    }
}

impl Decode for u8 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 1];
        bytes
            .read_exact(&mut buf)
            .await
            .map_err(|_| Error::Generic("Failed to read u8".parse().unwrap()))?;
        Ok(Box::from(buf[0]))
    }
}

impl Decode for i8 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 1];
        bytes
            .read_exact(&mut buf)
            .await
            .map_err(|_| Error::Generic("Failed to read i8".parse().unwrap()))?;
        Ok(Box::from(buf[0] as i8))
    }
}

impl Decode for u16 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 2];
        bytes
            .read_exact(&mut buf)
            .await
            .map_err(|_| Error::Generic("Failed to read u16".parse().unwrap()))?;
        Ok(Box::from(u16::from_be_bytes(buf)))
    }
}

impl Decode for i16 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 2];
        bytes
            .read_exact(&mut buf)
            .await
            .map_err(|_| Error::Generic("Failed to read i16".parse().unwrap()))?;
        Ok(Box::from(i16::from_be_bytes(buf)))
    }
}

impl Decode for u32 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 4];
        bytes
            .read_exact(&mut buf)
            .await
            .map_err(|_| Error::Generic("Failed to read u32".parse().unwrap()))?;
        Ok(Box::from(u32::from_be_bytes(buf)))
    }
}

impl Decode for i32 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 4];
        bytes
            .read_exact(&mut buf)
            .await
            .map_err(|_| Error::Generic("Failed to read i32".parse().unwrap()))?;
        Ok(Box::from(i32::from_be_bytes(buf)))
    }
}

impl Decode for i64 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 8];
        bytes
            .read_exact(&mut buf)
            .await
            .map_err(|_| Error::Generic("Failed to read i64".parse().unwrap()))?;
        Ok(Box::from(i64::from_be_bytes(buf)))
    }
}

impl Decode for f32 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 4];
        bytes
            .read_exact(&mut buf)
            .await
            .map_err(|_| Error::Generic("Failed to read f32".parse().unwrap()))?;
        Ok(Box::from(f32::from_be_bytes(buf)))
    }
}

impl Decode for f64 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 8];
        bytes
            .read_exact(&mut buf)
            .await
            .map_err(|_| Error::Generic("Failed to read f64".parse().unwrap()))?;
        Ok(Box::from(f64::from_be_bytes(buf)))
    }
}

impl Decode for String {
    // Now this one is a bit more complicated. The first few bytes are the len as a varint, but we
    // can't be sure how many bytes it will take up. We also can't be sure the entire varint has
    // arrived yet. So we need to read bytes until we have the entire varint, then read the string.
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let remaining_bytes = read_varint(bytes).await?;
        let mut string_buf = vec![0u8; remaining_bytes.into()];
        bytes.read_exact(&mut string_buf).await?;
        Ok(Box::from(String::from_utf8(string_buf)?))
    }
}

impl Decode for VarInt {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        Ok(Box::from(read_varint(bytes).await?))
    }
}

impl Decode for Varlong {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        Ok(Box::from(read_varlong(bytes).await?))
    }
}


impl Decode for u128 {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 16];
        bytes
            .read_exact(&mut buf)
            .await
            .map_err(|_| Error::Generic("Failed to read u128".parse().unwrap()))?;
        Ok(Box::from(u128::from_be_bytes(buf)))
    }
}

impl<V: Decode + Unpin> Decode for Vec<V> {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let len = read_varint(bytes).await?.get_val();
        // Yes the cast is necessary, and yes it's annoying
        let mut vec = Vec::new();
        for _ in 0..len {
            vec.push(Box::into_inner(V::decode(bytes).await?));
        }
        Ok(Box::from(vec))
    }
}

impl Decode for Position {
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut pos = Position {
            x: 0,
            y: 0,
            z: 0,
        };
        
        let full_data = bytes.read_i64().await?;

        pos.x = (full_data >> 38) as i32;
        pos.y = (full_data << 52 >> 52) as i16;
        pos.z = (full_data << 26 >> 38) as i32;

        Ok(Box::from(pos))
    }
}


pub trait Encode {
    #[allow(unused)]
    #[allow(async_fn_in_trait)]
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin;
}



impl Encode for bool {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let buf = if *self { [1u8] } else { [0u8] };
        bytes
            .write_all(&buf)
            .await
            .map_err(|_| Error::Generic("Failed to write bool".parse().unwrap()))
    }
}

impl Encode for u8 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let buf = [*self];
        bytes
            .write_all(&buf)
            .await
            .map_err(|_| Error::Generic("Failed to write u8".parse().unwrap()))
    }
}

impl Encode for i8 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let buf = [*self as u8];
        bytes
            .write_all(&buf)
            .await
            .map_err(|_| Error::Generic("Failed to write i8".parse().unwrap()))
    }
}

impl Encode for u16 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let buf = self.to_be_bytes();
        bytes
            .write_all(&buf)
            .await
            .map_err(|_| Error::Generic("Failed to write u16".parse().unwrap()))
    }
}

impl Encode for i16 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let buf = self.to_be_bytes();
        bytes
            .write_all(&buf)
            .await
            .map_err(|_| Error::Generic("Failed to write i16".parse().unwrap()))
    }
}

impl Encode for u32 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let buf = self.to_be_bytes();
        bytes
            .write_all(&buf)
            .await
            .map_err(|_| Error::Generic("Failed to write u32".parse().unwrap()))
    }
}

impl Encode for i32 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let buf = self.to_be_bytes();
        bytes
            .write_all(&buf)
            .await
            .map_err(|_| Error::Generic("Failed to write i32".parse().unwrap()))
    }
}

impl Encode for i64 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let buf = self.to_be_bytes();
        bytes
            .write_all(&buf)
            .await
            .map_err(|_| Error::Generic("Failed to write i64".parse().unwrap()))
    }
}

impl Encode for f32 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let buf = self.to_be_bytes();
        bytes
            .write_all(&buf)
            .await
            .map_err(|_| Error::Generic("Failed to write f32".parse().unwrap()))
    }
}

impl Encode for f64 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let buf = self.to_be_bytes();
        bytes
            .write_all(&buf)
            .await
            .map_err(|_| Error::Generic("Failed to write f64".parse().unwrap()))
    }
}

impl Encode for String {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let len = VarInt::new(self.len() as i32);
        len.encode(bytes).await?;
        bytes
            .write_all(self.as_bytes())
            .await
            .map_err(|_| Error::Generic("Failed to write String".parse().unwrap()))
    }
}

impl Encode for VarInt {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        write_varint(*self, bytes).await
    }
}

impl Encode for Varlong {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        write_varlong(*self, bytes).await
    }
}

impl Encode for u128 {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let buf = self.to_be_bytes();
        bytes
            .write_all(&buf)
            .await
            .map_err(|_| Error::Generic("Failed to write u128".parse().unwrap()))
    }
}

impl<V: Encode> Encode for Vec<V> {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        // Length of the vec in general can be handled with the
        // Encode derive macro
        // Therefore, we just need to encode each element
        for v in self {
            v.encode(bytes).await?;
        }
        Ok(())
    }
}


impl Encode for Position {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let u64val: u64 = ((self.x as u64 & 0x3FFFFFF) << 38) | ((self.z as u64 & 0x3FFFFFF) << 12) | (self.y as u64 & 0xFFF);
        let u64bytes = u64val.to_be_bytes();
        bytes.write_all(&u64bytes).await.map_err(|_| Error::Generic("Failed to write Position".parse().unwrap()))
    }
}