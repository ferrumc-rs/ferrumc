use tokio::io::{AsyncWrite, AsyncWriteExt};
use crate::enc::Encode;

pub unsafe trait SafeTransmute: Sized {}

macro_rules! impl_safe_transmute_for_types {
    ($($ty:ty),*) => {
        $(
            unsafe impl SafeTransmute for $ty {}
        )*
    };
}
impl_safe_transmute_for_types!(bool, u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);
impl<T: SafeTransmute> Encode for T {
    async fn encode<W>(&self, writer: &mut W) -> crate::error::Result<()>
    where
        W: AsyncWrite + Unpin
    {
        let bytes = unsafe { as_u8_slice(self) };
        writer.write_all(bytes).await?;
        Ok(())
    }
}

/// Transmutes a reference to a value to a slice of bytes.
/// do NOT call this function for something that is not a `#[repr(C)]` type.
/// doing so is undefined behavior.
unsafe fn as_u8_slice<T>(p: &T) -> &[u8] {
    std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        size_of::<T>(),
    )
}