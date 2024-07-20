use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let expanded = quote::quote! {
        impl Component for #name {}
    };
    TokenStream::from(expanded)
}