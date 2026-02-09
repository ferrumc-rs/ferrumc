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

    if fs::exists("src/blocks").unwrap_or(false) {
        fs::remove_dir_all("src/blocks").unwrap();
    }

    fs::create_dir_all("src/blocks").unwrap();

    let enum_impl = quote::quote! {
        use crate::SimpleBlock;

        #enum_impl
    };

    fs::write("src/blocks.rs", format_code(&block_mod.to_string())).unwrap();
    fs::write("src/simple.rs", format_code(&simple_enum.to_string())).unwrap();
    fs::write("src/simple_impl.rs", format_code(&enum_impl.to_string())).unwrap();

    block_structs
        .into_iter()
        .for_each(|((structure, struct_impl), name)| {
            let mod_name = name.to_string().to_snake_case();

            fs::write(
                format!("src/blocks/{}.rs", mod_name),
                format_code(&structure.to_string()),
            )
            .unwrap();
            fs::write(
                format!("src/blocks/{}_impl.rs", mod_name),
                format_code(&struct_impl.to_string()),
            )
            .unwrap();
        });
}
