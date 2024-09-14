use proc_macro::TokenStream;

mod deserialize;
mod helper;
mod serialize;
/// Derive macro for the `NBTSerialize` trait to auto-generate serialization into NBT format.
///
/// **POSSIBLE ATTRIBUTES**:
/// - ***rename***: Renames the field to the provided string.
/// - ***is_root***: Marks the struct as the root struct.
/// - ***net_encode***: Makes the NBT be encoded properly to work with other NBTEncodable types.
///                     (Must NOT implement `NBTEncodable`)
///
/// To serialize the entire root, please use the `root` attribute.
/// Otherwise the serialization **WON'T** be generated properly!!!!
///
/// <h5> For root usage: </h5>
/// Example:
///
/// ```ignore
/// use nbt_derive::NBTSerialize;
///
/// #[derive(NBTSerialize)]
/// #[nbt(is_root)] // Can also be written as #[nbt(is_root = true)]
/// pub struct Root {
///     pub player_name: String,
///     #[nbt(rename = "player_age")]
///     pub x: X,
/// }
/// #[derive(NBTSerialize)] // NOT ROOT!
/// struct X {
///     pub y: i32,
/// }
///
/// fn main() {
///     let player: Root = unimplemented!();
///
///     let mut buffer = Vec::new();
///     player.nbt_serialize(&mut buffer).unwrap();
/// }
///
/// ```
///
#[proc_macro_derive(NBTSerialize, attributes(nbt))]
pub fn nbt_serialize_derive(input: TokenStream) -> TokenStream {
    serialize::nbt_serialize_derive(input)
}

/// Derive macro for the `NBTDeserialize` trait to auto-generate deserialization into the provided Struct format.
///
/// Must define root attribute for the root struct.
///
/// <h5> Example usage: </h5>
///
/// ```ignore
///
/// use std::io::Cursor;
/// use nbt_derive::NBTDeserialize;
/// use nbt_lib::read_tag;
///
///
/// #[derive(NBTDeserialize)]
/// #[nbt(is_root)] // Can also be written as #[nbt(is_root = true)]
/// pub struct Root {
///     pub player_name: String,
///     #[nbt(rename = "player_age")]
///     pub x: X,
/// }
///
/// #[derive(NBTDeserialize)]
/// struct X {
///     pub y: i32,
/// }
///
/// fn main() {
///     let buffer: Vec<u8> = unimplemented!();
///     let nbt = read_tag(&mut Cursor::new(buffer)).unwrap();
///     let player: Root = Root::read_from(nbt).unwrap();
/// }
///
/// ```
///
#[proc_macro_derive(NBTDeserialize, attributes(nbt))]
pub fn nbt_deserialize_derive(input: TokenStream) -> TokenStream {
    deserialize::nbt_deserialize_derive(input)
}
