// Include generated modules
#![feature(const_option_ops)]
#![feature(const_trait_impl)]
#![feature(const_cmp)]

pub mod generated;

// Include build-generated blocks module
include!(concat!(env!("OUT_DIR"), "/blocks.rs"));

// Re-export all generated types for convenience
pub use generated::*;

#[cfg(test)]
mod tests;
