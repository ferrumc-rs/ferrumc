use crate::block::JSON_FILE;
use proc_macro::TokenStream;
use quote::quote;
use simd_json::base::{ValueAsObject, ValueAsScalar};
use simd_json::derived::ValueObjectAccess;

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
    let block_name = &input.name;
    let block_name = if block_name.starts_with("minecraft:") {
        block_name.to_string()
    } else {
        format!("minecraft:{}", block_name)
    };
    let block_state_id_var = &input.id_var;
    let mut buf = JSON_FILE.to_vec();
    let v = simd_json::to_owned_value(&mut buf).unwrap();
    let filtered_names = v
        .as_object()
        .unwrap()
        .iter()
        .filter(|(_, v)| v.get("name").as_str() == Some(&block_name))
        .map(|(k, v)| (k.parse::<u32>().unwrap(), v))
        .collect::<Vec<_>>();
    if filtered_names.is_empty() {
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
            #block_state_id_var == BlockStateId(#id)
        });
    }
    let joined = quote! {
        #(#arms)||*
    };
    joined.into()
}
