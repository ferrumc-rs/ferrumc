use ferrumc_nbt::de::borrow::NbtTag;
use ferrumc_nbt::de::converter::readers::async_reader::read_nbt_content_async;
use ferrumc_nbt::de::converter::readers::sync_reader::read_nbt_content_recursive;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use std::io::Read;
use tokio::io::AsyncRead;

pub fn create_compound_buffer() -> Vec<u8> {
    let mut buffer = Vec::with_capacity(256);
    buffer.push(10);
    buffer.push(0); //fake name byte 1
    buffer.push(0); //fake name byte 2

    buffer
}

pub async fn decode_compound_to_buffer_async<R: AsyncRead + Unpin>(
    reader: &mut R,
) -> Result<Vec<u8>, NetDecodeError> {
    let tag_id = u8::decode_async(reader, &NetDecodeOpts::None).await?;
    if tag_id != 10 {
        return Err(NetDecodeError::ExternalError(
            "Expected Compound NBT (10)".into(),
        ));
    }

    decode_compound_to_buffer_raw_async(reader).await
}

pub fn decode_compound_to_buffer<R: Read>(reader: &mut R) -> Result<Vec<u8>, NetDecodeError> {
    let tag_id = u8::decode(reader, &NetDecodeOpts::None)?;

    if tag_id != 10 {
        return Err(NetDecodeError::ExternalError(
            "Expected Compound NBT (10)".into(),
        ));
    }

    decode_compound_to_buffer_raw(reader)
}

pub fn read_mutf8_string<R: Read>(reader: &mut R) -> Result<String, NetDecodeError> {
    let length = u16::decode(reader, &NetDecodeOpts::None)?;
    let mut buf = vec![0u8; length as usize];
    reader.read_exact(&mut buf)?;

    String::from_utf8(buf) //todo: from mutf
        .map_err(|e| NetDecodeError::ExternalError(Box::new(e)))
}

pub async fn read_mutf8_string_async<R: AsyncRead + Unpin>(
    reader: &mut R,
) -> Result<String, NetDecodeError> {
    let length = u16::decode_async(reader, &NetDecodeOpts::None).await?;
    let mut buf = vec![0u8; length as usize];
    tokio::io::AsyncReadExt::read_exact(reader, &mut buf).await?;

    String::from_utf8(buf) //todo: from mutf
        .map_err(|e| NetDecodeError::ExternalError(Box::new(e)))
}

pub fn decode_compound_to_buffer_raw<R: Read>(reader: &mut R) -> Result<Vec<u8>, NetDecodeError> {
    let mut buffer = create_compound_buffer();

    read_nbt_content_recursive(&NbtTag::Compound, reader, &mut buffer)
        .map_err(|e| NetDecodeError::ExternalError(Box::new(e)))?;

    Ok(buffer)
}

pub async fn decode_compound_to_buffer_raw_async<R: AsyncRead + Unpin>(
    reader: &mut R,
) -> Result<Vec<u8>, NetDecodeError> {
    let mut buffer = create_compound_buffer();

    read_nbt_content_async(&NbtTag::Compound, reader, &mut buffer).await
        .map_err(|e| NetDecodeError::ExternalError(Box::new(e)))?;

    Ok(buffer)
}
