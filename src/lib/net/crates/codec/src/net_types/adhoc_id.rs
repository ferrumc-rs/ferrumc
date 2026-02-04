use crate::encode::errors::NetEncodeError;
use crate::encode::{NetEncode, NetEncodeOpts};
use std::io::Write;
use tokio::io::AsyncWrite;

pub struct AdHocID<T: NetEncode> {
    pub inner: T,
}

impl<T: NetEncode> NetEncode for AdHocID<T> {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        writer.write_all(&[0])?;
        self.inner.encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        _writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        unreachable!()
    }
}

impl<T> From<T> for AdHocID<T>
where
    T: NetEncode,
{
    fn from(inner: T) -> Self {
        Self { inner }
    }
}
