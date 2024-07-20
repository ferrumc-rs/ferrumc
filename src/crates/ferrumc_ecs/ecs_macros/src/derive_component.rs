use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let gen = quote! {
        impl crate::components::Component for #name {}
    };

    gen.into()
}