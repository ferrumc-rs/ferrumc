#![feature(proc_macro_quote)]

use block::matches;
use proc_macro::TokenStream;

mod block;
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
/// ```ignore
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

/// A macro to lookup block state IDs at compile time.
///
/// Feed in the block name as a string literal, and an optional set of properties as a map.
/// It will output a [`ferrumc_world::block_state_id::BlockStateId`] struct with the correct ID for that block and properties.
/// Usage:
/// ```ignore
/// # use ferrumc_world::block_state_id::BlockStateId;
/// # use ferrumc_macros::block;
/// let block_state_id = block!("stone");
/// let another_block_state_id = block!("minecraft:grass_block", {snowy: true});
/// assert_eq!(block_state_id, BlockStateId(1));
/// assert_eq!(another_block_state_id, BlockStateId(8));
/// ```
/// Unfortunately, due to current limitations in Rust's proc macros, you will need to import the
/// `BlockStateId` struct manually.
///
/// The `minecraft:` namespace is optional and will be added automatically if not present.
///
/// If the block or properties are invalid, a compile-time error will be thrown that should hopefully
/// explain the issue.
#[proc_macro]
pub fn block(input: TokenStream) -> TokenStream {
    block::block(input)
}

/// A macro to check if a block state ID matches a given block name at compile time.
/// Usage:
/// ```ignore
/// # use ferrumc_macros::{match_block};
/// # use ferrumc_world::block_state_id::BlockStateId;
/// let block_state_id = BlockStateId(1);
/// if match_block!("stone", block_state_id) {
///     // do something
/// }
/// ```
/// Unfortunately, due to current limitations in Rust's proc macros, you will need to import the
/// `BlockStateId` struct manually.
///
/// The `minecraft:` namespace is optional and will be added automatically if not present.
#[proc_macro]
pub fn match_block(input: TokenStream) -> TokenStream {
    matches::matches_block(input)
}
