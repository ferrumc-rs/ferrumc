use proc_macro::{quote, TokenStream};
use quote::ToTokens;

#[allow(unused_variables)]
pub fn profile_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = format!("profiler/{}", attr.to_string().replace("\"", "")).to_token_stream();
    quote! {
        #[tracing::instrument(name = $name)]
        $item
    }
}
