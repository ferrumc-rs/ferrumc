#![feature(portable_simd)]

#![allow(unsafe_code)]
#![allow(dead_code)]
use hashbrown as _;

pub mod de;

pub mod errors;
#[cfg(test)]
mod tests;
// mod chatgpt;
// mod chatgpt_v3_balanced;
// mod chatgpt_v3_balanced_2_no_tapes;
// mod chatgpt_v4_faster;
// mod chatgpt_v5_like_simdjson;
// mod chatgpt_v5_like_simdjson_2;
// mod chatgpt_v6_simple;
mod make_shift_parser;
// mod chatgpt_v7_upgraded;

pub(crate) type Result<T> = std::result::Result<T, errors::NBTError>;