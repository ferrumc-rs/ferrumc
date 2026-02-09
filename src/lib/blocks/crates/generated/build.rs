use ferrumc_blocks_build::complex::generate_complex_blocks;
use ferrumc_blocks_build::config::{get_block_states, get_build_config};
use ferrumc_blocks_build::simple::generate_simple_block_enum;
use ferrumc_blocks_build::{format_code, separate_blocks};
use heck::ToSnakeCase;
use std::fs;

fn main() {
    let build_config = get_build_config();
    let block_states = get_block_states();

    let (simple_blocks, complex_blocks) = separate_blocks(block_states);

    let (simple_enum, enum_impl) = generate_simple_block_enum(simple_blocks);
    let (block_structs, block_mod) = generate_complex_blocks(&build_config, complex_blocks);

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let enum_impl = quote::quote! {
        use crate::SimpleBlock;

        #enum_impl
    };

    fs::write(
        format!("{}/blocks.rs", out_dir),
        format_code(&block_mod.to_string()),
    )
    .unwrap();
    fs::write(
        format!("{}/simple.rs", out_dir),
        format_code(&simple_enum.to_string()),
    )
    .unwrap();
    fs::write(
        format!("{}/simple_impl.rs", out_dir),
        format_code(&enum_impl.to_string()),
    )
    .unwrap();

    block_structs
        .into_iter()
        .for_each(|((structure, struct_impl), name)| {
            let mod_name = name.to_string().to_snake_case();

            fs::write(
                format!("{}/{}.rs", out_dir, mod_name),
                format_code(&structure.to_string()),
            )
            .unwrap();
            fs::write(
                format!("{}/{}_impl.rs", out_dir, mod_name),
                format_code(&struct_impl.to_string()),
            )
            .unwrap();
        });
}
