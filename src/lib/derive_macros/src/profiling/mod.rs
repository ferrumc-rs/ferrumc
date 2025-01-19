use proc_macro::TokenStream;
use quote::{ToTokens, quote};

#[allow(unused_variables)]
pub fn profile_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = format!("profiler/{}", attr.to_string().replace("\"", "")).to_token_stream();
    let res = quote! {
        #[tracing::instrument(name = $name)]
        $item
    };

    res.into()
}
