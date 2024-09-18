#![feature(proc_macro_quote)]

mod profiling;
mod events;

#[proc_macro_attribute]
pub fn profile(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    profiling::profile_fn(attr, item)
}

#[proc_macro_attribute]
pub fn event_handler(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    events::event_handler_fn(attr, item)
}