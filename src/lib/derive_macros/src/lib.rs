#![feature(proc_macro_quote)]

use proc_macro::TokenStream;

mod commands;
mod helpers;
mod nbt;
mod net;
mod profiling;
mod registries_packets;
mod static_loading;

#[proc_macro_attribute]
pub fn profile(attr: TokenStream, item: TokenStream) -> TokenStream {
    profiling::profile_fn(attr, item)
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
/// You can get the packet_id from:
/// <https://protocol.ferrumc.com>,
/// In incoming packets (serverbound),
/// You should use the 'resource' value referenced in the packet,
/// e.g. "finish_configuration", which would result in the packet_id being automatically fetched.
#[proc_macro_attribute]
pub fn packet(args: TokenStream, input: TokenStream) -> TokenStream {
    net::packets::attribute(args, input)
}

#[proc_macro]
pub fn setup_packet_handling(input: TokenStream) -> TokenStream {
    net::packets::setup_packet_handling(input)
}

/// Get a packet entry from the packets.json file.
/// returns protocol_id (as 0x??) of the specified packet.
/// e.g. get_packet_entry!("play", "clientbound", "add_entity") -> 0x01
#[proc_macro]
pub fn lookup_packet(input: TokenStream) -> TokenStream {
    static_loading::packets::lookup_packet(input)
}
// #=================== PACKETS ===================#

/// Creates a command.
///
/// A command function can take a sender argument, multiple command arguments and bevy system arguments.
///
/// The optional sender argument is marked with `#[sender]` attribute and command arguments are marked with
/// the `#[arg]` attribute. Any other argument is treated as a bevy system arg.
///
/// Usage example:
///
/// ```
/// #[command("hello")]
/// fn command(#[sender] sender: Sender) {
///     sender.send_message(TextComponent::from("Hello, world!"), false);
/// }
/// ```
#[proc_macro_attribute]
pub fn command(attr: TokenStream, input: TokenStream) -> TokenStream {
    commands::command(attr, input)
}

// #[proc_macro_attribute]
// pub fn arg(attr: TokenStream, input: TokenStream) -> TokenStream {
//     commands::arg(attr, input)
// }

/// Get a registry entry from the registries.json file.
/// returns protocol_id (as u64) of the specified entry.
#[proc_macro]
pub fn get_registry_entry(input: TokenStream) -> TokenStream {
    static_loading::registry::get(input)
}

#[proc_macro]
pub fn build_registry_packets(input: TokenStream) -> TokenStream {
    registries_packets::build_mapping(input)
}
