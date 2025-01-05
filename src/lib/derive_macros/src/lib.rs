#![feature(proc_macro_quote)]

use proc_macro::TokenStream;

mod events;
mod helpers;
mod inventory;
mod nbt;
mod net;
mod profiling;
mod static_loading;

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
/// You can get the packet_id from:
/// https://protocol.ferrumc.com,
/// In incoming packets (serverbound),
/// You should use the 'resource' value referenced in the packet,
/// e.g. "finish_configuration", which would result in the packet_id being automatically fetched.
#[proc_macro_attribute]
pub fn packet(args: TokenStream, input: TokenStream) -> TokenStream {
    net::packets::attribute(args, input)
}

#[proc_macro]
pub fn bake_packet_registry(input: TokenStream) -> TokenStream {
    net::packets::bake_registry(input)
}

/// Get a packet entry from the packets.json file.
/// returns protocol_id (as 0x??) of the specified packet.
/// e.g. get_packet_entry!("play", "clientbound", "add_entity") -> 0x01
#[proc_macro]
pub fn get_packet_entry(input: TokenStream) -> TokenStream {
    static_loading::packets::get(input)
}
// #=================== PACKETS ===================#

/// Get a registry entry from the registries.json file.
/// returns protocol_id (as u64) of the specified entry.
#[proc_macro]
pub fn get_registry_entry(input: TokenStream) -> TokenStream {
    static_loading::registry::get(input)
}

// #=================== INVENTORY ===================#
#[proc_macro_derive(Inventory, attributes(slot))]
pub fn create_inventory(input: TokenStream) -> TokenStream {
    inventory::create(input)
}

#[proc_macro_attribute]
pub fn slot(args: TokenStream, input: TokenStream) -> TokenStream {
    inventory::slot(args, input)
}

#[proc_macro_attribute]
pub fn inventory_type(args: TokenStream, input: TokenStream) -> TokenStream {
    inventory::inventory_type(args, input)
}
// #=================== INVENTORY ===================#
