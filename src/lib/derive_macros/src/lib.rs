#![feature(proc_macro_quote)]

use proc_macro::TokenStream;

mod events;
mod helpers;
mod nbt;
mod profiling;
mod net;

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

#[proc_macro_derive(NBTDeserialize, attributes(nbt))]
pub fn nbt_ser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    nbt::de::derive(input)
}

#[proc_macro_derive(NBTSerialize, attributes(nbt))]
pub fn nbt_de(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    nbt::ser::derive(input)
}

#[proc_macro_derive(NetEncode, attributes(net))]
pub fn net_encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    net::encode::derive(input)
}

#[proc_macro_derive(NetDecode, attributes(net))]
pub fn net_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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