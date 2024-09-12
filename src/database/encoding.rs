use bincode::config::standard;
use bincode::{Decode, Encode};
use heed::{BytesDecode, BytesEncode};
use std::borrow::Cow;
use std::marker::PhantomData;

pub struct Zstd<T>(PhantomData<T>);

impl<'a, T: Encode + 'a> BytesEncode<'a> for Zstd<T> {
    type EItem = T;

    fn bytes_encode(item: &'a Self::EItem) -> Result<Cow<'a, [u8]>, heed::BoxedError> {
        // Compress
        let bytes = bincode::encode_to_vec(item, standard())?;

        Ok(Cow::Owned(bytes))
    }
}

impl<'a, T: Decode + 'a> BytesDecode<'a> for Zstd<T> {
    type DItem = T;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, heed::BoxedError> {
        let decoded = bincode::decode_from_slice(bytes, standard())?;
        Ok(decoded.0)
    }
}

pub struct ZstdCodec;

impl ZstdCodec {
    pub async fn compress_data<T: Encode + Send + 'static>(data: T) -> crate::Result<Vec<u8>> {
        tokio::task::spawn_blocking(move || {
            let mut bytes = Vec::new();
            let mut compressor = zstd::Encoder::new(&mut bytes, 3)?;
            bincode::encode_into_std_write(&data, &mut compressor, standard())?;
            compressor.finish()?;
            Ok(bytes)
        })
        .await?
    }
    pub async fn decompress_data<T: Decode + Send + 'static>(data: Vec<u8>) -> crate::Result<T> {
        tokio::task::spawn_blocking(move || {
            let decoded = bincode::decode_from_slice(data.as_slice(), standard())?;
            Ok(decoded.0)
        })
        .await?
    }
}
