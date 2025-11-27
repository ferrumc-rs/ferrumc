use heck::ToShoutySnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;
use std::fs;

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/tags.json");

    let tags: BTreeMap<String, BTreeMap<String, Vec<String>>> =
        serde_json::from_str(&fs::read_to_string("../../../assets/extracted/tags.json").unwrap())
            .expect("Failed to parse tags.json");

    let mut tag_consts = TokenStream::new();
    let mut lists = TokenStream::new();
    let mut lookup_fns = TokenStream::new();

    for (name, tag_data) in tags.iter() {
        let name = name.replace('/', "_");
        let const_ident = format_ident!("{}_TAGS", name.to_shouty_snake_case());
        let name_ident = format_ident!("get_{name}_tag");

        lookup_fns.extend(quote! {
            pub fn #name_ident(tag_name: &str) -> Option<&TagData> {
                Self::#const_ident
                    .iter()
                    .filter_map(|tag| if tag.name == tag_name { Some(*tag) } else { None })
                    .next()
            }
        });

        let mut tag_names = TokenStream::new();

        for (tag, values) in tag_data.iter() {
            let tag_ident = format_ident!("{}_{}", const_ident, tag.to_shouty_snake_case());

            let values = values.iter().map(|value| format!("minecraft:{value}"));

            tag_names.extend(quote! {
                &Self::#tag_ident,
            });

            tag_consts.extend(quote! {
                pub const #tag_ident: TagData = TagData {
                    name: #tag,
                    values: &[#(#values),*],
                };
            })
        }

        lists.extend(quote! {
            pub const #const_ident: &'static [&'static TagData] = &[#tag_names];
        });
    }

    quote! {
        #[derive(Debug, Clone)]
        pub struct TagData {
            pub name: &'static str,
            pub values: &'static [&'static str],
        }

        impl TagData {
            #tag_consts

            #lists

            #lookup_fns
        }
    }
}
