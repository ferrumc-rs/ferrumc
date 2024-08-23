use crate::utils::impls::type_impls::Encode;
use nbt_lib::nbt_spec::serializer::NBTCompoundMarker;
use nbt_lib::NBTSerialize;
use tokio::io::AsyncWrite;
use crate::utils::error::Error;

pub mod bitset;
pub mod position;
pub mod varint;
pub mod varlong;
pub mod velocity;


pub struct Enc<S>(pub S);

impl<S> Encode for Enc<S> {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + Unpin
    {
        self.0.encode(bytes).await
    }
}

impl<S: NBTSerialize> Enc<S> {
    fn into_encodable(self) -> Enc<S> {
        Enc(self.0)
    }
}
pub trait Fallback {
    type Output;
    fn into_encodable(self) -> Self::Output;
}

impl<S> Fallback for Enc<S> {
    type Output = S;

    fn into_encodable(self) -> S {
        self.0
    }
}
