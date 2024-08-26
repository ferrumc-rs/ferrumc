use tokio::io::AsyncWrite;

use crate::prelude::*;

mod non_primitives;
mod primitives;

pub trait NetEncode {
    #[allow(async_fn_in_trait)]
    async fn net_encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin;
}
