#![feature(proc_macro_quote)]

use proc_macro::TokenStream;

mod events;
mod helpers;
mod nbt;
mod net;
mod profiling;
mod registry;

#[proc_macro_attribute]
pub fn profile(attr: TokenStream, item: TokenStream) -> TokenStream {
    profiling::profile_fn(attr, item)
}

#[proc_macro_attribute]
pub fn event_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    events::event_handler_fn(attr, item)
}

#[proc_macro_derive(Event, attributes(event))]
pub fn event(input: TokenStream) -> TokenStream {
    events::derive(input)
}

#[proc_macro_derive(NBTDeserialize, attributes(nbt))]
pub fn nbt_ser(input: TokenStream) -> TokenStream {
    nbt::de::derive(input)
}

#[proc_macro_derive(NBTSerialize, attributes(nbt))]
pub fn nbt_de(input: TokenStream) -> TokenStream {
    nbt::ser::derive(input)
}

#[proc_macro_derive(NetEncode, attributes(net))]
pub fn net_encode(input: TokenStream) -> TokenStream {
    net::encode::derive(input)
}

#[proc_macro_derive(NetDecode, attributes(net))]
pub fn net_decode(input: TokenStream) -> TokenStream {
    net::decode::derive(input)
}

// #=================== PACKETS ===================#
#[proc_macro_attribute]
pub fn packet(args: TokenStream, input: TokenStream) -> TokenStream {
    net::packets::attribute(args, input)
}

#[proc_macro]
pub fn bake_packet_registry(input: TokenStream) -> TokenStream {
    net::packets::bake_registry(input)
}
// #=================== PACKETS ===================#

/// Get a registry entry from the registries.json file.
/// returns protocol_id (as u64) of the specified entry.
#[proc_macro]
pub fn get_registry_entry(input: TokenStream) -> TokenStream {
    registry::get_entry::get(input)
}