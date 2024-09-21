use proc_macro::TokenStream;

pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let _fields = crate::helpers::get_fields(&input);

    unimplemented!()
}
