use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{collections::BTreeMap, fs};

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/effect.json");

    let effects: BTreeMap<String, serde_json::Value> =
        serde_json::from_str(&fs::read_to_string("../../../assets/extracted/effect.json").unwrap())
            .expect("Failed to parse effect.json");

    let effect_names: Vec<String> = effects.keys().cloned().collect();

    let variants = crate::array_to_tokenstream(&effect_names);

    let type_from_name = &effect_names
        .iter()
        .map(|effect| {
            let id = &effect;
            let name = format_ident!("{}", effect.to_pascal_case());

            quote! {
                #id => Some(Self::#name),
            }
        })
        .collect::<TokenStream>();

    let type_to_name = &effect_names
        .iter()
        .map(|effect| {
            let id = &effect;
            let name = format_ident!("{}", effect.to_pascal_case());

            quote! {
                Self::#name => #id,
            }
        })
        .collect::<TokenStream>();

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Effect {
            #variants
        }

        impl Effect {
            #[doc = r" Try to parse an `Effect` from a resource location string."]
            pub fn from_name(name: &str) -> Option<Self> {
                let name = name.strip_prefix("minecraft:").unwrap_or(name);
                match name {
                    #type_from_name
                    _ => None
                }
            }

            pub const fn to_name(&self) -> &'static str {
                match self {
                    #type_to_name
                }
            }
        }
    }
}
