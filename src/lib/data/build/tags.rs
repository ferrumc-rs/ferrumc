use heck::ToShoutySnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;
use std::fs;

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/tags.json");

    let tags: BTreeMap<String, serde_json::Value> =
        serde_json::from_str(&fs::read_to_string("../../../assets/extracted/tags.json").unwrap())
            .expect("Failed to parse tags.json");

    let mut constants = TokenStream::new();
    let mut type_from_name = TokenStream::new();

    for (name, tag_data) in tags.iter() {
        let const_ident = format_ident!("{}", name.replace('/', "_").to_shouty_snake_case());

        // Convert the tag data to a string representation for now
        let tag_str = serde_json::to_string(tag_data).unwrap();

        constants.extend(quote! {
            pub const #const_ident: TagData = TagData {
                name: #name,
                data: #tag_str,
            };
        });

        type_from_name.extend(quote! {
            #name => Some(&Self::#const_ident),
        });
    }

    quote! {
        #[derive(Debug, Clone)]
        pub struct TagData {
            pub name: &'static str,
            pub data: &'static str,
        }

        impl TagData {
            #constants

            #[doc = r" Try to parse a `TagData` from a resource location string."]
            pub fn from_name(name: &str) -> Option<&'static Self> {
                let name = name.strip_prefix("minecraft:").unwrap_or(name);
                match name {
                    #type_from_name
                    _ => None
                }
            }
        }
    }
}
