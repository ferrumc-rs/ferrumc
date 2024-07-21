extern crate proc_macro;

use proc_macro::TokenStream;

mod encode;
mod decode;
mod packet;
mod ecs;
mod utils;

#[proc_macro_derive(Decode)]
pub fn decode_derive(input: TokenStream) -> TokenStream {
    decode::derive(input)
}

#[proc_macro_derive(Encode, attributes(encode))]
pub fn encode_derive(input: TokenStream) -> TokenStream {
    encode::derive(input)
}

#[proc_macro_attribute]
pub fn packet(args: TokenStream, input: TokenStream) -> TokenStream { packet::attribute(args, input) }

#[proc_macro]
pub fn bake_packet_registry(input: TokenStream) -> TokenStream {
    packet::bake(input)
}


#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    ecs::derive_component(input)
}
#[proc_macro_derive(Constructor)]
pub fn derive_constructor(input: TokenStream) -> TokenStream {
    ecs::derive_constructor(input)
}

#[proc_macro_derive(AutoGenName)]
pub fn derive_name(input: TokenStream) -> TokenStream {
    utils::derive_name(input)
}