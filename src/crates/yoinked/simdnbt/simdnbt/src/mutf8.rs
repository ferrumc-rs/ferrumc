//! The string representation used in NBT.

use std::{
    borrow::{Borrow, Cow},
    fmt, mem,
    ops::Deref,
    simd::prelude::*,
};

use simd_cesu8::mutf8;

/// A MUTF-8 string slice. This is how strings are represented internally in NBT.
#[derive(Eq, PartialEq)]
pub struct Mutf8Str {
    pub(crate) slice: [u8],
}
/// An owned MUTF-8 string.
#[derive(Eq, PartialEq, Clone, Default)]
pub struct Mutf8String {
    pub(crate) vec: Vec<u8>,
}

#[inline]
fn is_plain_ascii(slice: &[u8]) -> bool {
    let mut is_plain_ascii = true;
    let chunks_32_exact = slice.array_chunks::<32>();
    let mut remainder = chunks_32_exact.remainder();
    if remainder.len() > 16 {
        let chunk;
        (chunk, remainder) = remainder.split_first_chunk::<16>().unwrap();
        let mask = u8x16::splat(0b10000000);
        let zero = u8x16::splat(0);
        let simd = u8x16::from_array(*chunk);
        let masked = simd & mask;
        if masked != zero {
            is_plain_ascii = false;
        }
    }
    if remainder.len() > 8 {
        let chunk;
        (chunk, remainder) = remainder.split_first_chunk::<8>().unwrap();
        let mask = u8x8::splat(0b10000000);
        let zero = u8x8::splat(0);
        let simd = u8x8::from_array(*chunk);
        let masked = simd & mask;
        if masked != zero {
            is_plain_ascii = false;
        }
    }
    if remainder.len() > 4 {
        let chunk;
        (chunk, remainder) = remainder.split_first_chunk::<4>().unwrap();
        let mask = u8x4::splat(0b10000000);
        let zero = u8x4::splat(0);
        let simd = u8x4::from_array(*chunk);
        let masked = simd & mask;
        if masked != zero {
            is_plain_ascii = false;
        }
    }
    for &byte in remainder {
        if byte & 0b10000000 != 0 {
            is_plain_ascii = false;
        }
    }

    for &chunk in chunks_32_exact {
        let mask = u8x32::splat(0b10000000);
        let zero = u8x32::splat(0);
        let simd = u8x32::from_array(chunk);
        let masked = simd & mask;
        if masked != zero {
            is_plain_ascii = false;
        }
    }

    is_plain_ascii
}

impl Mutf8Str {
    // we can't implement FromStr on Cow<Mutf8Str>
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn from_str(s: &str) -> Cow<Mutf8Str> {
        match mutf8::encode(s) {
            Cow::Borrowed(slice) => Cow::Borrowed(Mutf8Str::from_slice(slice)),
            Cow::Owned(vec) => Cow::Owned(Mutf8String { vec }),
        }
    }

    /// Try to convert this MUTF-8 string into a UTF-8 string. If the data isn't
    /// valid MUTF-8, it'll return an empty string without erroring.
    #[inline]
    pub fn to_str(&self) -> Cow<str> {
        // fast check to skip if none of the bytes have the top bit set.
        // note that this allows some valid utf8 but invalid mutf8 through as
        // null bytes aren't allowed in mutf8.
        if is_plain_ascii(&self.slice) {
            // SAFETY: Plain ASCII is always valid UTF-8.
            unsafe { Cow::Borrowed(std::str::from_utf8_unchecked(&self.slice)) }
        } else {
            // we use the non-strict variant as it's apparently significantly
            // faster and our is_plain_ascii check makes it non-strict already
            // anyways.
            mutf8::decode(&self.slice).unwrap_or_default()
        }
    }

    #[inline]
    pub fn to_string_lossy(&self) -> Cow<str> {
        mutf8::decode_lossy(&self.slice)
    }

    /// Convert a slice of bytes into a Mutf8Str. This is safe because it's only
    /// checked to be valid MUTF-8 while being converted to UTF-8.
    #[inline]
    pub fn from_slice(slice: &[u8]) -> &Mutf8Str {
        // SAFETY: &[u8] and &Mutf8Str are the same layout.
        unsafe { mem::transmute::<&[u8], &Mutf8Str>(slice) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.slice.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slice.is_empty()
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.slice
    }
}

impl fmt::Display for Mutf8Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}

impl fmt::Debug for Mutf8Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("m")?;
        fmt::Debug::fmt(&self.to_str(), f)
    }
}

impl fmt::Debug for Mutf8String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("m")?;
        fmt::Debug::fmt(&self.to_str(), f)
    }
}

impl ToOwned for Mutf8Str {
    type Owned = Mutf8String;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        Mutf8String {
            vec: self.slice.to_vec(),
        }
    }
}
impl Borrow<Mutf8Str> for Mutf8String {
    #[inline]
    fn borrow(&self) -> &Mutf8Str {
        self.as_str()
    }
}

impl Mutf8String {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    #[inline]
    pub fn as_str(&self) -> &Mutf8Str {
        Mutf8Str::from_slice(self.vec.as_slice())
    }

    /// Try to convert this MUTF-8 string into a UTF-8 string. If the data isn't
    /// valid MUTF-8, it'll return an empty string without erroring.
    #[inline]
    pub fn into_string(self) -> String {
        if is_plain_ascii(&self.vec) {
            // SAFETY: &[u8] and &str are the same layout.
            unsafe { String::from_utf8_unchecked(self.vec) }
        } else {
            mutf8::decode(&self.vec).unwrap_or_default().to_string()
        }
    }

    #[inline]
    pub fn from_string(s: String) -> Mutf8String {
        Self::from_vec(mutf8::encode(&s).into_owned())
    }

    #[inline]
    pub fn from_vec(vec: Vec<u8>) -> Mutf8String {
        Self { vec }
    }
}
impl Deref for Mutf8String {
    type Target = Mutf8Str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl From<String> for Mutf8String {
    #[inline]
    fn from(s: String) -> Self {
        Self::from_string(s)
    }
}
impl From<&str> for Mutf8String {
    #[inline]
    fn from(s: &str) -> Self {
        Self::from_string(s.to_owned())
    }
}

impl Default for &Mutf8Str {
    #[inline]
    fn default() -> Self {
        Mutf8Str::from_slice(&[])
    }
}

impl From<&Mutf8Str> for Mutf8String {
    #[inline]
    fn from(s: &Mutf8Str) -> Self {
        s.to_owned()
    }
}

impl From<&Mutf8Str> for String {
    #[inline]
    fn from(s: &Mutf8Str) -> Self {
        s.to_str().into_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::mutf8::Mutf8Str;

    #[test]
    fn same_as_utf8() {
        let str = "Hello, world!";
        // 16-bit Unicode characters are the same in UTF-8 and MUTF-8:
        assert_eq!(
            Mutf8Str::from_str(str),
            Cow::Borrowed(Mutf8Str::from_slice(str.as_bytes()))
        );
        assert_eq!(Mutf8Str::from_str(str).to_str(), Cow::Borrowed(str));
    }

    #[test]
    fn surrogate_pairs() {
        let str = "\u{10401}";
        let mutf8_data = &[0xED, 0xA0, 0x81, 0xED, 0xB0, 0x81];
        // 'mutf8_data' is a byte slice containing a 6-byte surrogate pair which
        // becomes a 4-byte UTF-8 character.
        assert_eq!(
            Mutf8Str::from_slice(mutf8_data).to_str(),
            Cow::Borrowed(str)
        );
    }

    #[test]
    fn null_bytes() {
        let str = "\0";
        let mutf8_data = vec![0xC0, 0x80];
        // 'str' is a null character which becomes a two-byte MUTF-8 representation.
        assert_eq!(
            Mutf8Str::from_slice(&mutf8_data).to_str(),
            Cow::Borrowed(str)
        );
    }
}
