use crate::prelude::*;
use tokio::io::AsyncWrite;

mod primitives;
mod non_repr_c;

pub trait Encode {
    async fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin;
}

