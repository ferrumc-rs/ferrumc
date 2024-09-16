#![feature(proc_macro_quote)]

mod profiling;

#[proc_macro_attribute]
pub fn profile_fn(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    profiling::profile_fn(attr, item).into()
}
