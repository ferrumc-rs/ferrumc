use heck::ToShoutySnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use syn::LitFloat;

#[derive(Deserialize, Clone, Debug)]
pub struct Attribute {
    pub id: u16,
    pub default_value: f64,
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/attributes.json");

    let attributes: BTreeMap<String, Attribute> = serde_json::from_str(
        &fs::read_to_string("../../../assets/extracted/attributes.json").unwrap(),
    )
    .expect("Failed to parse attributes.json");

    let mut constants = TokenStream::new();
    let mut type_from_id_arms = TokenStream::new();
    let mut type_from_name = TokenStream::new();
    let mut const_idents = Vec::new();

    for (name, attribute) in attributes.iter() {
        let const_ident = format_ident!("{}", name.to_shouty_snake_case());
        const_idents.push(const_ident.clone());
        let id_lit = syn::LitInt::new(&attribute.id.to_string(), Span::call_site());
        let default_value_lit = LitFloat::new(
            &format!("{:.1}", attribute.default_value),
            Span::call_site(),
        );

        constants.extend(quote! {
            pub const #const_ident: Attribute = Attribute {
                id: #id_lit,
                name: #name,
                default_value: #default_value_lit,
            };
        });

        type_from_id_arms.extend(quote! {
            #id_lit => Some(&Self::#const_ident),
        });

        type_from_name.extend(quote! {
            #name => Some(&Self::#const_ident),
        });
    }

    quote! {
        use std::collections::HashMap;

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct Attribute {
            pub id: u16,
            pub name: &'static str,
            pub default_value: f64,
        }

        impl Attribute {
            #constants

            #[doc = r" Try to parse an `Attribute` from a resource location string."]
            pub fn from_name(name: &str) -> Option<&'static Self> {
                let name = name.strip_prefix("minecraft:").unwrap_or(name);
                match name {
                    #type_from_name
                    _ => None
                }
            }

            #[doc = r" Try to get an `Attribute` from its ID."]
            pub const fn from_id(id: u16) -> Option<&'static Self> {
                match id {
                    #type_from_id_arms
                    _ => None
                }
            }

            #[doc = r" Get all attributes as a slice."]
            pub fn all() -> &'static [&'static Self] {
                &[#(&Self::#const_idents),*]
            }
        }
    }
}
