use proc_macro2::TokenStream;
use quote::quote;
use crate::{BuildConfig, ComplexBlock};

pub fn generate_complex_blocks(config: &BuildConfig, block_states: Vec<(u32, ComplexBlock)>) -> TokenStream {
    quote! {
        // TODO
    }
}