use proc_macro::TokenStream;

mod deserialize;
mod helper;
mod serialize;
/// Derive macro for the `NBTSerialize` trait to auto-generate serialization into NBT format.
///
/// To serialize the entire root, please use the `root` attribute.
/// Otherwise the serialization ***won't*** be generated properly!!!!
///
/// <h5> For root usage: </h5>
/// Example:
///
/// ```rust
/// use nbt_derive::Serialize;
///
/// #[derive(Serialize)]
/// #[nbt(is_root)] // Can also be written as #[nbt(is_root = true)]
/// pub struct Root {
///     pub player_name: String,
///     #[nbt(rename = "player_age")]
///     pub x: X,
/// }
/// #[derive(Serialize)] // NOT ROOT!
/// struct X {
///     pub y: i32,
/// }
///
/// fn main() {
///     let player: Root = unimplemented!();
///
///     let mut buffer = Vec::new();
///     player.serialize(&mut buffer).unwrap();
/// }
///
/// ```
///
#[proc_macro_derive(Serialize, attributes(nbt))]
pub fn nbt_serialize_derive(input: TokenStream) -> TokenStream {
    serialize::nbt_serialize_derive(input)
}

/// Derive macro for the `NBTDeserialize` trait to auto-generate deserialization into the provided Struct format.
///
/// Must define root attribute for the root struct.
///
/// <h5> Example usage: </h5>

/// ```rust
/// use nbt_derive::Deserialize;
///
///
/// #[derive(Deserialize)]
/// #[nbt(is_root)] // Can also be written as #[nbt(is_root = true)]
/// pub struct Root {
///     pub player_name: String,
///     #[nbt(rename = "player_age")]
///     pub x: X,
/// }
///
/// #[derive(Deserialize)]
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
#[proc_macro_derive(Deserialize, attributes(nbt))]
pub fn nbt_deserialize_derive(input: TokenStream) -> TokenStream {
    deserialize::nbt_deserialize_derive(input)
}
