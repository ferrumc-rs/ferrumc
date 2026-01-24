use ferrumc_blocks_build::complex::fill_complex_block_mappings;
use ferrumc_blocks_build::config::{get_block_states, get_build_config};
use ferrumc_blocks_build::separate_blocks;
use ferrumc_blocks_build::simple::fill_simple_block_mappings;
use quote::__private::TokenStream;
use quote::quote;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let build_config = get_build_config();
    let block_states = get_block_states();

    let mut mappings = Vec::with_capacity(block_states.len());
    mappings.resize(block_states.len(), TokenStream::new());
    let (simple_blocks, complex_blocks) = separate_blocks(block_states);

    let enum_const = fill_simple_block_mappings(simple_blocks, &mut mappings);
    let complex_consts = fill_complex_block_mappings(&build_config, complex_blocks, &mut mappings);

    let mapping_const = quote! {
        {
            use ferrumc_blocks_generated::*;

            #enum_const
            #(#complex_consts)*

            &[
                #(#mappings),*
            ]
        }
    };

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dir = Path::new(&out_dir).join("mappings.rs");
    fs::write(dir, mapping_const.to_string()).unwrap();
}
