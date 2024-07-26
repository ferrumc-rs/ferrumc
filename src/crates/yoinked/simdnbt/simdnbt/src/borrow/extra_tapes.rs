use std::fmt::{self, Debug};

use crate::{raw_list::RawList, Mutf8Str};

#[derive(Default)]
pub struct ExtraTapes<'a> {
    pub elements: Vec<ExtraTapeElement<'a>>,
}

impl Debug for ExtraTapes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExtraTapes")
    }
}

pub union ExtraTapeElement<'a> {
    /// An indicator for how long the following list is. This is what we point to from
    /// `TapeTagValue`.
    pub length: u32,
    pub byte_array: &'a [u8],
    pub string: &'a Mutf8Str,
    pub int_array: RawList<'a, i32>,
    pub long_array: RawList<'a, i64>,
}
