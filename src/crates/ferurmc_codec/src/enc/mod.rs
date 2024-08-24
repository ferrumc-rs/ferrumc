use tokio::io::AsyncWrite;
use crate::prelude::*;

mod primitives;
mod non_primitives;

pub trait Encode {
    async fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin;
}

