use proc_macro::TokenStream;

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
/// // Just an example of the serialize method. It's already implemented for
/// impl Root { pub fn serialize(&self, writer: &mut Vec<u8>) -> std::io::Result<()> { Ok(unimplemented!()) } }
/// ```
///
#[proc_macro_derive(Serialize, attributes(nbt))]
pub fn nbt_serialize_derive(input: TokenStream) -> TokenStream {
    serialize::nbt_serialize_derive(input)
}