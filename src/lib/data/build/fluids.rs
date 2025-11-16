use std::fs;
use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;

#[derive(Deserialize)]
struct Fluid {
    name: String,
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/fluids.json");

    let fluids: Vec<Fluid> =
        serde_json::from_str(&fs::read_to_string("../../../assets/extracted/fluids.json").unwrap())
            .expect("Failed to parse fluids.json");
    
    let fluid_names: Vec<String> = fluids.iter().map(|f| f.name.clone()).collect();
    
    let variants = crate::array_to_tokenstream(&fluid_names);
    
    let type_from_name = &fluid_names
        .iter()
        .map(|fluid| {
            let id = &fluid;
            let name = format_ident!("{}", fluid.to_pascal_case());

            quote! {
                #id => Some(Self::#name),
            }
        })
        .collect::<TokenStream>();
        
    let type_to_name = &fluid_names
        .iter()
        .map(|fluid| {
            let id = &fluid;
            let name = format_ident!("{}", fluid.to_pascal_case());

            quote! {
                Self::#name => #id,
            }
        })
        .collect::<TokenStream>();
        
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Fluid {
            #variants
        }

        impl Fluid {
            #[doc = r" Try to parse a `Fluid` from a resource location string."]
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