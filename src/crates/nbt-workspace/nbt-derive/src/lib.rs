use proc_macro::TokenStream;

mod serialize;
#[proc_macro_derive(Serialize)]
pub fn nbt_serialize_derive(input: TokenStream) -> TokenStream {
    serialize::nbt_serialize_derive(input)
}