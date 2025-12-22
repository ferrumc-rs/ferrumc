use fastnbt::{DeOpts, SerOpts, Value};
use ferrumc_nbt::NBTSerializable;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use mutf8::mutf8_to_utf8;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

#[derive(Debug, Clone, PartialEq)]
pub enum TextComponent {
    Simple(String),
    Complex(HashMap<String, Value>),
}

impl Default for TextComponent {
    fn default() -> Self {
        TextComponent::Simple("".into())
    }
}

impl Hash for TextComponent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            TextComponent::Simple(string) => {
                todo!("implement TextComponent hash")
            }
            TextComponent::Complex(map) => {
                todo!("implement TextComponent hash")
            }
        }
    }
}

fn decode_string(data: &[u8]) -> Result<String, NetDecodeError> {
    let utf8 = mutf8_to_utf8(&data).map_err(|err| NetDecodeError::ExternalError(err.into()))?;

    Ok(String::from_utf8(utf8.into_owned())
        .map_err(|err| NetDecodeError::ExternalError(err.into()))?)
}

impl NetDecode for TextComponent {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let tag = u8::decode(reader, opts)?;

        match tag {
            8 => {
                let len = u16::decode(reader, &NetDecodeOpts::default())?;
                let mut buf = vec![0; len as usize];
                reader.read_exact(&mut buf)?;

                let string = decode_string(&buf)?;

                Ok(TextComponent::Simple(string))
            }
            10 => {
                let content = fastnbt::from_reader_with_opts(reader, DeOpts::network_nbt())
                    .map_err(|e| NetDecodeError::ExternalError(e.into()))?;
                Ok(TextComponent::Complex(content))
            }
            _ => Err(NetDecodeError::ExternalError("Unexpected tag".into())),
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let tag = u8::decode_async(reader, opts).await?;

        match tag {
            8 => {
                let len = u16::decode_async(reader, &NetDecodeOpts::default()).await?;
                let mut buf = vec![0; len as usize];
                reader.read_exact(&mut buf).await?;

                let string = decode_string(&buf)?;

                Ok(TextComponent::Simple(string))
            }
            10 => {
                //todo: fix AsyncReadSyncAdapter
                // we don't know how much bytes will be read. Also fastnbt cant async.
                // write fn to copy compound tag bytes to local buffer.
                let mut adapter = AsyncReadSyncAdapter { reader };
                let content = fastnbt::from_reader_with_opts(&mut adapter, DeOpts::network_nbt())
                    .map_err(|e| NetDecodeError::ExternalError(e.into()))?;

                Ok(TextComponent::Complex(content))
            }
            _ => Err(NetDecodeError::ExternalError("Unexpected tag".into())),
        }
    }
}

impl NetEncode for TextComponent {
    fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match self {
            TextComponent::Simple(string) => {
                let options = ferrumc_nbt::NBTSerializeOptions::Network;
                string.serialize(writer, &options);

                Ok(())
            }
            TextComponent::Complex(complex) => {
                let options = SerOpts::network_nbt();
                fastnbt::to_writer_with_opts(writer, complex, options)
            }
        }
            .map_err(|err| NetEncodeError::ExternalError(Box::new(err)))?;

        Ok(())
    }

    //todo: byte pool
    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        let options = SerOpts::network_nbt();
        let mut buf: Vec<u8>;

        match self {
            TextComponent::Simple(string) => {
                buf = Vec::with_capacity(string.len());
                fastnbt::to_writer_with_opts(&mut buf, string, options)
            }
            TextComponent::Complex(complex) => {
                buf = Vec::with_capacity(256);
                fastnbt::to_writer_with_opts(&mut buf, complex, options)
            }
        }
            .map_err(|err| NetEncodeError::ExternalError(Box::new(err)))?;

        writer.write_all(&buf).await?;

        Ok(())
    }
}

///I hate my life
struct AsyncReadSyncAdapter<'a, R: AsyncRead + Unpin> {
    reader: &'a mut R,
}

impl<'a, R: AsyncRead + Unpin> Read for AsyncReadSyncAdapter<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        futures::executor::block_on(self.reader.read(buf))
    }
}
