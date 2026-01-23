use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_simple_block_enum(mut simple_blocks: Vec<(u32, String)>, mappings: &mut [TokenStream]) -> TokenStream {
    simple_blocks.sort_by_key(|(id, _)| *id);

    let mut map_entries = Vec::new();
    let mut enum_variants = Vec::new();

    for (id, name) in simple_blocks {
        let variant = name.strip_prefix("minecraft:").unwrap_or(&name);
        let variant = format_ident!("{}", variant.to_pascal_case());

        mappings[id as usize] = quote! { SimpleBlock::#variant };

        enum_variants.push(quote! { #variant });
        map_entries.push(quote! { #id });
    }

    quote! {
        use ferrumc_world::block_state_id::BlockStateId;

        const SIMPLE_BLOCK_STATE_MAP: &[u32] = &[
            #(#map_entries,)*
        ];

        #[repr(usize)]
        #[derive(Clone, Debug)]
        pub enum SimpleBlock {
            #(#enum_variants),*
        }

        impl TryInto<BlockStateId> for SimpleBlock {
            type Error = ();

            fn try_into(self) -> Result<BlockStateId, Self::Error> {
                Ok(BlockStateId::new(SIMPLE_BLOCK_STATE_MAP[self as usize]))
            }
        }
    }
}