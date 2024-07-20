use proc_macro::TokenStream;

mod derive_component;
mod derive_constructor;

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    derive_component::derive(input)
}
#[proc_macro_derive(Constructor)]
pub fn derive_constructor(input: TokenStream) -> TokenStream {
    derive_constructor::derive(input)
}