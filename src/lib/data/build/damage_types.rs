use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{collections::BTreeMap, fs};

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/damage_type.json");

    let damage_types: BTreeMap<String, serde_json::Value> = serde_json::from_str(
        &fs::read_to_string("../../../assets/extracted/damage_type.json").unwrap(),
    )
    .expect("Failed to parse damage_type.json");

    let damage_type_names: Vec<String> = damage_types.keys().cloned().collect();

    let variants = crate::array_to_tokenstream(&damage_type_names);

    let type_from_name = &damage_type_names
        .iter()
        .map(|damage_type| {
            let id = &damage_type;
            let name = format_ident!("{}", damage_type.to_pascal_case());

            quote! {
                #id => Some(Self::#name),
            }
        })
        .collect::<TokenStream>();

    let type_to_name = &damage_type_names
        .iter()
        .map(|damage_type| {
            let id = &damage_type;
            let name = format_ident!("{}", damage_type.to_pascal_case());

            quote! {
                Self::#name => #id,
            }
        })
        .collect::<TokenStream>();

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum DamageType {
            #variants
        }

        impl DamageType {
            #[doc = r" Try to parse a `DamageType` from a resource location string."]
            pub const fn try_from_name(name: &str) -> Option<Self> {
                let name = crate::helpers::strip_prefix_or_self(name, "minecraft:");
                match name {
                    #type_from_name
                    _ => None
                }
            }
            #[doc = " Get the name of the `DamageType`"]
            pub const fn to_name(&self) -> &'static str {
                match self {
                    #type_to_name
                }
            }
        }
    }
}
