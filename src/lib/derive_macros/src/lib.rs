#![feature(proc_macro_quote)]

mod events;
mod profiling;
mod nbt;
mod helpers;

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

#[proc_macro_derive(NBTDeserialize)]
pub fn nbt_ser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    nbt::de::derive(input)
}

#[proc_macro_derive(NBTSerialize)]
pub fn nbt_de(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    nbt::ser::derive(input)
}