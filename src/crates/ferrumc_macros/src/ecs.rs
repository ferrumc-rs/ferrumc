use proc_macro::TokenStream;

use quote::{format_ident, quote};
use regex::Regex;
use syn::{parse_macro_input, DeriveInput, Fields};

pub fn derive_component(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;


    let snake_case_name = split_camel_case(&name.to_string());
    let ext_trait_name = format_ident!("{}StateExt", name);
    let get_method_name = format_ident!("get_{}_component", snake_case_name);
    let get_method_name_mut = format_ident!("get_{}_component_mut", snake_case_name);


    let gen = quote! {
        impl crate::ecs::component::Component for #name {}

        pub trait #ext_trait_name {
            async fn #get_method_name<'a>(&'a self, entity_id: impl TryInto<usize>) -> crate::Result<crate::ecs::component::ComponentRef<'a, #name>>;
            async fn #get_method_name_mut<'a>(&'a self, entity_id: impl TryInto<usize>) -> crate::Result<crate::ecs::component::ComponentRefMut<'a, #name>>;
        }

        impl #ext_trait_name for crate::ecs::world::World {
             async fn #get_method_name<'a>(&'a self, entity_id: impl TryInto<usize>) -> crate::Result<crate::ecs::component::ComponentRef<'a, #name>>{
                let entity_id = entity_id.try_into().map_err(|_| crate::Error::ConversionError)?;
                let state = self.clone();
                state.get_component::<#name>(entity_id).await
            }

            async fn #get_method_name_mut<'a>(&'a self, entity_id: impl TryInto<usize>) -> crate::Result<crate::ecs::component::ComponentRefMut<'a, #name>>{
                let entity_id = entity_id.try_into().map_err(|_| crate::Error::ConversionError)?;
                let state = self.clone();
                state.get_component_mut::<#name>(entity_id).await
            }
        }
    };
    gen.into()
}

pub fn derive_constructor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("New can only be derived for structs"),
    };

    let (field_names, field_types): (Vec<_>, Vec<_>) = match fields {
        Fields::Named(fields) => fields.named.iter().map(|f| (&f.ident, &f.ty)).unzip(),
        _ => panic!("New can only be derived for structs with named fields"),
    };

    let expanded = quote! {
        impl #name {
            pub fn new(#(#field_names: #field_types),*) -> Self {
                Self {
                    #(#field_names),*
                }
            }
        }
    };

    expanded.into()
}

fn split_camel_case(s: &str) -> String {
    let re = Regex::new(r"([a-z0-9])([A-Z])").unwrap();
    re.replace_all(s, "${1}_${2}").to_lowercase()
}