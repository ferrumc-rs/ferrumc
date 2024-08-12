use syn::Attribute;

const BASE_NAME: &str = "nbt";
const IS_ROOT_ATTRIBUTE: &str = "is_root";
const RENAME_ATTRIBUTE: &str = "rename";


/// Parses the attributes of the struct and returns the values of the `is_root` and `rename` attributes.
/// @return: (is_root, rename)
/// Example of usage:
/// ```rust
/// use nbt_derive::Serialize;
///
/// #[derive(Serialize)]
/// #[nbt(is_root = true)]
/// #[nbt(rename = "player_data")]
/// pub struct Root {
///    pub player_name: String,
/// }
pub fn parse_struct_attributes(attrs: &[Attribute]) -> (bool, Option<String>) {
    let mut is_root = false;
    let mut rename = None;

    for attr in attrs {
        if !attr.path().is_ident(BASE_NAME) {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(IS_ROOT_ATTRIBUTE) {
                // is_root = meta.value()?.parse::<syn::LitBool>()?.value;
                // So it also works like this (both are valid):
                // #[nbt(is_root)]
                // #[nbt(is_root = true)]
                is_root = true;
                if let Ok(value) = meta.value() {
                    is_root = value.parse::<syn::LitBool>()?.value;
                }
            } else if meta.path.is_ident(RENAME_ATTRIBUTE) {
                rename = Some(meta.value()?.parse::<syn::LitStr>()?.value());
            }
            Ok(())
        }).unwrap();
    }

    (is_root, rename)
}

/// Parses the attributes of the field and returns the value of the `rename` attribute.
/// @return: rename option
/// Example of usage:
/// ```rust
/// use nbt_derive::Serialize;
///
/// #[derive(Serialize)]
/// pub struct Root {
///   #[nbt(rename = "player_name")]
///   pub x: String,
/// }
pub fn parse_field_attributes(attrs: &[Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        if !attr.path().is_ident(BASE_NAME) { return None; }

        let mut rename = None;
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(RENAME_ATTRIBUTE) {
                rename = Some(meta.value()?.parse::<syn::LitStr>()?.value());
            }

            Ok(())
        }).unwrap();

        rename
    })
}