use proc_macro::TokenStream;
use quote::quote;

use syn::DeriveInput;

pub fn derive_name(input: TokenStream) -> TokenStream {
    // auto generate a #name() method for the struct
    let input = syn::parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let name_str = name.to_string();

    let expanded = quote! {
        impl #name {
            pub fn type_name() -> &'static str {
                #name_str
            }
        }
    };

    TokenStream::from(expanded)
}