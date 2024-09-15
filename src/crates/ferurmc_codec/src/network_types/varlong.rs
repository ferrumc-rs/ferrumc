use std::fmt::Display;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::prelude::*;

/// A Varlong is a variable-length long that is used in the Minecraft protocol. Similar to
/// [crate::encoding::varint], it uses the least amount of bytes possible to represent the value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Varlong(pub i64);

impl Varlong {
    pub fn new(value: i64) -> Self {
        Varlong(value)
    }

    /// Read a Varlong from the given cursor. Uses simple bit shifting to read the value.
    ///
    /// I did not write this, but I genuinely have no idea where it came from. I think it was from
    /// a Minecraft protocol library, but I can't find the original source.
    pub async fn read<T>(cursor: &mut T) -> Result<Varlong>
    where
        T: AsyncRead + Unpin,
    {
        let mut val = 0;
        let mut count = 0;
        loop {
            let byte = cursor.read_u8().await.map_err(|e| CodecError::Io(e))?;
            val |= ((byte & 0x7F) as i64) << (count * 7);
            count += 1;
            if count > 10 {
                return Err(CodecError::VarLongTooBig);
            }
            if (byte & 0x80) == 0 {
                return Ok(Varlong(val));
            }
        }
    }
}

mod adapters {
    use crate::enc::{EncodeOption, NetEncode};

    use super::*;

    impl Display for Varlong {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl From<Varlong> for i64 {
        fn from(varlong: Varlong) -> i64 {
            varlong.0
        }
    }

    impl From<i64> for Varlong {
        fn from(value: i64) -> Varlong {
            Varlong(value)
        }
    }

    impl Into<usize> for Varlong {
        fn into(self) -> usize {
            self.0 as usize
        }
    }

    impl NetEncode for Varlong {
        async fn net_encode<T>(&self, cursor: &mut T, _encode_option: &EncodeOption) -> Result<()>
        where
            T: AsyncWrite + Unpin,
        {
            write_varlong(*self, cursor).await
        }
    }
}

/// Write a Varlong to the given cursor.
///
/// Yoinked from valence: https://github.com/valence-rs/valence/blob/main/crates/valence_protocol/src/var_long.rs#L52
///
/// Uses some assembly magic to write the Varlong in a more efficient way. I have no idea how it works.
/// If the target architecture is not x86 or x86_64, or the target OS is macOS, it falls back to a slower method.
///
/// Unfortunately, this method uses a fair amount of unsafe code, so it's not the most readable.
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(target_os = "macos")
))]
pub async fn write_varlong<T>(varlong: Varlong, mut w: T) -> Result<()>
where
    T: AsyncWrite + Unpin,
{
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    // Break the number into 7-bit parts and spread them out into a vector
    let mut res = [0_u64; 2];
    {
        let x = varlong.0 as u64;

        res[0] = unsafe { _pdep_u64(x, 0x7f7f7f7f7f7f7f7f) };
        res[1] = unsafe { _pdep_u64(x >> 56, 0x000000000000017f) }
    };
    let stage1: __m128i = unsafe { std::mem::transmute(res) };

    // Create a mask for where there exist values
    // This signed comparison works because all MSBs should be cleared at this point
    // Also handle the special case when num == 0
    let minimum =
        unsafe { _mm_set_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff_u8 as i8) };
    let exists = unsafe { _mm_or_si128(_mm_cmpgt_epi8(stage1, _mm_setzero_si128()), minimum) };
    let bits = unsafe { _mm_movemask_epi8(exists) };

    // Count the number of bytes used
    let bytes_needed = 32 - bits.leading_zeros() as u8; // lzcnt on supported CPUs

    // Fill that many bytes into a vector
    let ascend = unsafe { _mm_setr_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15) };
    let mask = unsafe { _mm_cmplt_epi8(ascend, _mm_set1_epi8(bytes_needed as i8)) };

    // Shift it down 1 byte so the last MSB is the only one set, and make sure only
    // the MSB is set
    let shift = unsafe { _mm_bsrli_si128::<1>(mask) };
    let msbmask = unsafe { _mm_and_si128(shift, _mm_set1_epi8(128_u8 as i8)) };

    // Merge the MSB bits into the vector
    let merged = unsafe { _mm_or_si128(stage1, msbmask) };
    let bytes = unsafe { std::mem::transmute::<__m128i, [u8; 16]>(merged) };

    w.write_all(unsafe { bytes.get_unchecked(..bytes_needed as usize) })
        .await?;

    Ok(())
}

/// Fallback method for writing a Varlong. Safer and cross-platform, but slower.
#[cfg(any(
    not(any(target_arch = "x86", target_arch = "x86_64")),
    target_os = "macos"
))]
async fn write_varlong<T>(varlong: Varlong, mut w: T) -> Result<()>
where
    T: AsyncWrite + Unpin,
{
    use byteorder::WriteBytesExt;

    let mut val = varlong.0 as u64;
    loop {
        if val & 0b1111111111111111111111111111111111111111111111111111111110000000 == 0 {
            w.write_u8(val as u8).await?;
            return Ok(());
        }
        w.write_u8(val as u8 & 0b01111111 | 0b10000000).await?;
        val >>= 7;
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[tokio::test]
    async fn read_varlong_valid_input() {
        let mut cursor = Cursor::new(vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]);
        let result = Varlong::read(&mut cursor).await;
        assert_eq!(result.unwrap(), Varlong::new(9223372036854775807));
    }

    #[tokio::test]
    async fn read_varlong_too_big() {
        let mut cursor = Cursor::new(vec![0xff; 9]);
        let result = Varlong::read(&mut cursor).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn write_varlong_valid_input() {
        let mut cursor = Cursor::new(Vec::new());
        let result = write_varlong(Varlong::from(-2147483648), &mut cursor).await;
        assert!(result.is_ok());
        assert_eq!(
            cursor.into_inner(),
            vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01]
        );
    }

    #[tokio::test]
    async fn write_varlong_zero() {
        let mut cursor = Cursor::new(Vec::new());
        let result = write_varlong(Varlong::from(0), &mut cursor).await;
        assert!(result.is_ok());
        assert_eq!(cursor.into_inner(), vec![0b00000000]);
    }

    #[tokio::test]
    async fn read_varlong_empty_input() {
        let mut cursor = Cursor::new(vec![]);
        let result = Varlong::read(&mut cursor).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn read_varlong_single_byte() {
        let mut cursor = Cursor::new(vec![0b00000001]);
        let result = Varlong::read(&mut cursor).await;
        assert_eq!(result.unwrap(), Varlong::new(1));
    }

    #[tokio::test]
    async fn write_varlong_negative_input() {
        let mut cursor = Cursor::new(Vec::new());
        let result = write_varlong(Varlong::from(-1), &mut cursor).await;
        assert!(result.is_ok());
        assert_eq!(
            cursor.into_inner(),
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]
        );
    }
}
