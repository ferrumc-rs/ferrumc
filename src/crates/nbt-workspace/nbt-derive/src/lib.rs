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
/// #[derive(Serialize(root = "Player"))] // ROOT!
/// pub struct Root {
///     pub player_name: String,
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
#[proc_macro_derive(Serialize, attributes(root))]
pub fn nbt_serialize_derive(input: TokenStream) -> TokenStream {
    serialize::nbt_serialize_derive(input)
}