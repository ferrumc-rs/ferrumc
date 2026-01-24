use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn fill_simple_block_mappings(simple_blocks: Vec<(u32, String)>, mappings: &mut [TokenStream]) -> TokenStream {
    for (id, _) in simple_blocks {
        mappings[id as usize] = quote! { crate::StateBehaviorTable::spin_off(&VTABLE_SIMPLE_BLOCK, #id) };
    }

    quote! {
        const VTABLE_SIMPLE_BLOCK: crate::BlockBehaviorTable = crate::BlockBehaviorTable::from::<SimpleBlock>();
    }
}

pub fn generate_simple_block_enum(mut simple_blocks: Vec<(u32, String)>) -> (TokenStream, TokenStream) {
    simple_blocks.sort_by_key(|(id, _)| *id);

    let mut map_entries = Vec::new();
    let mut from_arms = Vec::new();
    let mut enum_variants = Vec::new();

    for (id, name) in simple_blocks {
        let variant = name.strip_prefix("minecraft:").unwrap_or(&name);
        let variant = format_ident!("{}", variant.to_pascal_case());

        enum_variants.push(quote! { #variant });
        from_arms.push(quote! { #id => Ok(SimpleBlock::#variant) });
        map_entries.push(quote! { #id });
    }

    (
        quote! {
            #[repr(usize)]
            #[derive(Clone, Debug)]
            pub enum SimpleBlock {
                #(#enum_variants),*
            }
        },
        quote! {
            const SIMPLE_BLOCK_STATE_MAP: &[u32] = &[
                #(#map_entries,)*
            ];

            impl TryFrom<u32> for SimpleBlock {
                type Error = ();

                fn try_from(data: u32) -> Result<Self, Self::Error> {
                    match data {
                        #(#from_arms),*,
                        _ => Err(())
                    }
                }
            }

            impl TryInto<u32> for SimpleBlock {
                type Error = ();

                fn try_into(self) -> Result<u32, Self::Error> {
                    Ok(SIMPLE_BLOCK_STATE_MAP[self as usize])
                }
            }
        }
    )
}