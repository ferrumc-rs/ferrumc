use proc_macro::TokenStream;
use quote::{ToTokens, quote};

pub fn profile_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: proc_macro2::TokenStream = item.into();
    let name = format!("profiler/{}", attr.to_string().replace("\"", "")).to_token_stream();
    let res = quote! {
        #[tracing::instrument(name = #name)]
        #item
    };

    res.into()
}
