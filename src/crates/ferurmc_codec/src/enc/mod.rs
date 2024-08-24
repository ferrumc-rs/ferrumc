use tokio::io::AsyncWrite;
use crate::prelude::*;

mod primitives;
mod non_primitives;

pub trait Encode {
    #[allow(async_fn_in_trait)]
    async fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin;
}

