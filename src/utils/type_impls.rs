use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncWrite, AsyncWriteExt};

use crate::utils::encoding::position::Position;
use crate::utils::encoding::varint::{read_varint, VarInt, write_varint};
use crate::utils::encoding::varlong::{read_varlong, Varlong, write_varlong};
use crate::utils::error::Error;

/// This trait is used to decode a type from a byte stream. It is implemented for all types that
/// can be decoded from a byte stream.
///
/// This trait is async, as it is expected that decoding will involve reading from a stream, which
/// is an async operation.
///
/// It has a single method, `decode`, which takes a mutable reference to a type that implements
/// `AsyncRead` and `AsyncSeek`, and returns a `Result` containing a boxed version of the type being
/// decoded.
///
/// The main use for this is type-agnostic decoding in macros, mainly in the `Decode` derive macro
/// ([ferrumc_macros::derive_decode()]) to convert a byte stream into struct fields without knowledge
/// of their types.
pub trait Decode {
    #[allow(unused)]
    #[allow(async_fn_in_trait)]
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin;
}

impl Decode for bool {
    /// Decodes a bool from a byte stream. This is a simple operation, as a bool is just a single
    /// byte, with 0 being false and 1 being true.
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
    /// Decodes a u8 from a byte stream. Takes out a single byte.
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
    /// Decodes an i8 from a byte stream. Takes out a single byte, and sign extends it.
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
    /// Decodes a u16 from a byte stream. Takes out 2 bytes.
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
    /// Decodes an i16 from a byte stream. Takes out 2 bytes, and sign extends it.
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
    /// Decodes a u32 from a byte stream. Takes out 4 bytes.
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
    /// Decodes an i32 from a byte stream. Takes out 4 bytes, and sign extends it.
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

impl Decode for u64 {
    /// Decodes a u64 from a byte stream. Takes out 8 bytes.
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut buf = [0u8; 8];
        bytes
            .read_exact(&mut buf)
            .await
            .map_err(|_| Error::Generic("Failed to read u64".parse().unwrap()))?;
        Ok(Box::from(u64::from_be_bytes(buf)))
    }
}

impl Decode for i64 {
    /// Decodes an i64 from a byte stream. Takes out 8 bytes, and sign extends it.
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
    /// Decodes a f32 (float) from a byte stream. Takes out 4 bytes.
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
    /// Decodes a f64 (double) from a byte stream. Takes out 8 bytes.
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
    /// Decodes a String from a byte stream. The first byte(s) is a VarInt representing the length of
    /// the string, followed by the string itself. The string is expected to be UTF-8 encoded.
    /// Takes out a variable number of bytes.
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
    /// Decodes a VarInt from a byte stream. VarInts are a variable length encoding of integers,
    /// where the lower 7 bits of each byte are used to encode the number, and the 8th bit is used
    /// to indicate if there are more bytes to read. This method reads bytes until it finds a byte
    /// where the 8th bit is 0, and then decodes the number from the bytes read. Uses
    /// [ferrumc_utils::encoding::varint::read_varint] to read the VarInt.
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        Ok(Box::from(read_varint(bytes).await?))
    }
}

impl Decode for Varlong {
    /// Decodes a Varlong from a byte stream. Varlongs are a variable length encoding of longs,
    /// where the lower 7 bits of each byte are used to encode the number, and the 8th bit is used
    /// to indicate if there are more bytes to read. This method reads bytes until it finds a byte
    /// where the 8th bit is 0, and then decodes the number from the bytes read. Uses
    /// [ferrumc_utils::encoding::varlong::read_varlong] to read the Varlong.
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        Ok(Box::from(read_varlong(bytes).await?))
    }
}

impl Decode for u128 {
    /// Decodes a u128 from a byte stream. Takes out 16 bytes.
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
    /// Decodes a Vec from a byte stream. The first byte(s) is a VarInt representing the length of the
    /// Vec, followed by the elements of the Vec. The elements are decoded in order, and the Vec is
    /// constructed from the decoded elements. Uses [ferrumc_utils::encoding::varint::read_varint] to
    /// read the length of the Vec, and [Decode::decode] to decode each element.
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
    /// Decodes a Position from a byte stream. A Position is a 64-bit integer, where the 26 MSB
    /// are the x coordinate, the next 26 bits are the z coordinate, and the 12 LSB are
    /// the y coordinate. This method reads the 64-bit integer, and then extracts the x, y, and z
    /// coordinates from it. The x and z coordinates are sign extended, and the y coordinate is zero
    /// extended. The coordinates are then stored in a Position struct. The Position struct is then
    /// boxed and returned. The Position struct is used to represent block positions in Minecraft.
    async fn decode<T>(bytes: &mut T) -> Result<Box<Self>, Error>
    where
        T: AsyncRead + AsyncSeek + Unpin,
    {
        let mut pos = Position { x: 0, y: 0, z: 0 };

        let full_data = bytes.read_i64().await?;

        pos.x = (full_data >> 38) as i32;
        pos.y = (full_data << 52 >> 52) as i16;
        pos.z = (full_data << 26 >> 38) as i32;

        Ok(Box::from(pos))
    }
}

/// This trait is used to encode a type into a byte stream. It is implemented for all types that
/// can be encoded into a byte stream. This trait is async, as it is expected that encoding will
/// involve writing to a stream, which is an async operation.
///
/// It has a single method, `encode`, which takes a mutable reference to a type that implements
/// `AsyncWrite` and `AsyncSeek`, and returns a `Result` containing `()`. The method writes the
/// encoded bytes to the byte stream.
///
/// The main use for this is type-agnostic encoding in macros, mainly in the `Encode` derive macro
/// ([ferrumc_macros::derive_encode()]) to convert struct fields into a byte stream without knowledge
/// of their types.
pub trait Encode {
    #[allow(unused)]
    #[allow(async_fn_in_trait)]
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin;
}

impl Encode for bool {
    /// Encodes a bool into a byte stream. A bool is encoded as a single byte, with 0 being false
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
    /// Encodes a u8 into a byte stream. A u8 is encoded as a single byte.
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
    /// Encodes an i8 into a byte stream. An i8 is encoded as a single byte.
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
    /// Encodes a u16 into a byte stream. A u16 is encoded as 2 bytes.
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
    /// Encodes an i16 into a byte stream. An i16 is encoded as 2 bytes.
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
    /// Encodes a u32 into a byte stream. A u32 is encoded as 4 bytes.
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
    /// Encodes an i32 into a byte stream. An i32 is encoded as 4 bytes.
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

impl Encode for u64 {
    /// Encodes a u64 into a byte stream. A u64 is encoded as 8 bytes.
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let buf = self.to_be_bytes();
        bytes
            .write_all(&buf)
            .await
            .map_err(|_| Error::Generic("Failed to write u64".parse().unwrap()))
    }
}

impl Encode for i64 {
    /// Encodes an i64 into a byte stream. An i64 is encoded as 8 bytes.
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
    /// Encodes a f32 (float) into a byte stream. A f32 is encoded as 4 bytes.
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
    /// Encodes a f64 (double) into a byte stream. A f64 is encoded as 8 bytes.
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
    /// Encodes a String into a byte stream. A String is encoded as a VarInt representing the length
    /// of the string, followed by the string itself. The string is expected to be UTF-8 encoded.
    /// The length of the string is encoded as a VarInt, and the string is then written to the byte
    /// stream. Uses [ferrumc_utils::encoding::varint::VarInt] to encode the length of the string.
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
    /// Encodes a VarInt into a byte stream. VarInts are a variable length encoding of integers,
    /// where the lower 7 bits of each byte are used to encode the number, and the 8th bit is used
    /// to indicate if there are more bytes to read. This method encodes the number into bytes, and
    /// then writes the bytes to the byte stream. Uses [ferrumc_utils::encoding::varint::write_varint]
    /// to encode the VarInt.
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        write_varint(*self, bytes).await
    }
}

impl Encode for Varlong {
    /// Encodes a Varlong into a byte stream. Varlongs are a variable length encoding of longs,
    /// where the lower 7 bits of each byte are used to encode the number, and the 8th bit is used
    /// to indicate if there are more bytes to read. This method encodes the number into bytes, and
    /// then writes the bytes to the byte stream. Uses [ferrumc_utils::encoding::varlong::write_varlong]
    /// to encode the Varlong.
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        write_varlong(*self, bytes).await
    }
}

impl Encode for u128 {
    /// Encodes a u128 into a byte stream. A u128 is encoded as 16 bytes.
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
    /// Encodes a Vec into a byte stream. The Vec is encoded as a VarInt representing the length of
    /// the Vec, followed by the elements of the Vec. The elements are encoded in order, and then
    /// written to the byte stream. Uses [ferrumc_utils::encoding::varint::VarInt] to encode the length
    /// of the Vec, and [Encode::encode] to encode each element.
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
    /// Encodes a Position into a byte stream. A Position is a 64-bit integer, where the 26 MSB
    /// are the x coordinate, the next 26 bits are the z coordinate, and the 12 LSB are
    /// the y coordinate. This method encodes the x, y, and z coordinates into a 64-bit integer,
    /// and then writes the integer to the byte stream. The x and z coordinates are masked to 26 bits,
    /// and the y coordinate is masked to 12 bits. Uses [ferrumc_utils::encoding::position::Position]
    /// to represent the Position.
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + AsyncSeek + Unpin,
    {
        let u64val: u64 = ((self.x as u64 & 0x3FFFFFF) << 38)
            | ((self.z as u64 & 0x3FFFFFF) << 12)
            | (self.y as u64 & 0xFFF);
        let u64bytes = u64val.to_be_bytes();
        bytes
            .write_all(&u64bytes)
            .await
            .map_err(|_| Error::Generic("Failed to write Position".parse().unwrap()))
    }
}
