use std::fs;
use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::array_to_tokenstream;

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/sounds.json");

    let sounds: Vec<String> =
        serde_json::from_str(&fs::read_to_string("../../../assets/extracted/sounds.json").unwrap())
            .expect("Failed to parse sounds.json");
    
    let variants = array_to_tokenstream(&sounds);
    
    let type_from_name = &sounds
        .iter()
        .map(|sound| {
            let id = &sound;
            let name = format_ident!("{}", sound.to_pascal_case());

            quote! {
                #id => Some(Self::#name),
            }
        })
        .collect::<TokenStream>();
        
    let type_to_name = &sounds
        .iter()
        .map(|sound| {
            let id = &sound;
            let name = format_ident!("{}", sound.to_pascal_case());

            quote! {
                Self::#name => #id,
            }
        })
        .collect::<TokenStream>();
        
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Sound {
            #variants
        }

        impl Sound {
            #[doc = r" Try to parse a `Sound` from a resource location string."]
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