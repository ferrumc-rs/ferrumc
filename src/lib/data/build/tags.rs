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

    for (tag_name, tag_data) in tags.iter() {
        let tag_name = tag_name.replace('/', "_");
        let const_tag_name_ident = format_ident!("{}_TAGS", tag_name.to_shouty_snake_case());
        let get_function_name_ident = format_ident!("get_{tag_name}_tag");

        lookup_fns.extend(quote! {
            pub const fn #get_function_name_ident(tag_name: &str) -> Option<&TagData> {
                let mut i = 0;
                let v = Self::#const_tag_name_ident;
                while i < v.len() {
                    if v[i].name == tag_name {
                        return Some(v[i]);
                    }
                    i += 1;
                }
                None
            }
        });

        let mut tag_names = TokenStream::new();

        for (tag, values) in tag_data.iter() {
            let tag_ident =
                format_ident!("{}_{}", const_tag_name_ident, tag.to_shouty_snake_case());

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
            pub const #const_tag_name_ident: &'static [&'static TagData] = &[#tag_names];
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
