use crate::block::BLOCK_STATES;
use proc_macro::TokenStream;
use quote::quote;

struct Input {
    name: String,
    id_var: syn::Expr,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name_lit: syn::LitStr = input.parse()?;
        let _comma: syn::Token![,] = input.parse()?;
        let id_var: syn::Expr = input.parse()?;
        Ok(Input {
            name: name_lit.value(),
            id_var,
        })
    }
}

// match_block!("stone", block_state_id); -> "if block_state_id == BlockStateId(1) { ... }
// match_block!("dirt", block_state_id); -> "if block_name == BlockStateId( { ... }
pub fn matches_block(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as Input);
    let block_name = input
        .name
        .split_once("minecraft:")
        .map(|x| x.1)
        .unwrap_or(input.name.as_str());
    let block_id_var = &input.id_var;

    let states = BLOCK_STATES.get(block_name);
    if states.is_none_or(|x| x.is_empty()) {
        return syn::Error::new_spanned(
            &input.id_var,
            format!("Block name '{}' not found in registry", block_name),
        )
        .to_compile_error()
        .into();
    }
    let mut arms = Vec::new();
    for (id, _) in filtered_names {
        arms.push(quote! {
            #block_state_id_var == BlockStateId::new(#id)
        });
    }
    let joined = quote! {
        #(#arms)||*
    };
    matched.into()
}
