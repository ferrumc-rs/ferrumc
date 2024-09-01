use syn::Attribute;

const BASE_NAME: &str = "nbt";
const IS_ROOT_ATTRIBUTE: &str = "is_root";
const RENAME_ATTRIBUTE: &str = "rename";
const NO_ENCODE_ATTRIBUTE: &str = "no_encode";

pub struct AttributeValues {
    pub is_root: bool,
    pub rename: Option<String>,
    pub no_encode: bool,
}


/// Parses the attributes of the struct and returns the values of the `is_root` and `rename` attributes.
/// @return: (is_root, rename)
/// Example of usage:
/// ```rust
/// use nbt_derive::NBTSerialize;
///
/// #[derive(NBTSerialize)]
/// #[nbt(is_root = true)]
/// #[nbt(rename = "player_data")]
/// pub struct Root {
///    pub player_name: String,
/// }
pub fn parse_struct_attributes(attrs: &[Attribute]) -> AttributeValues {

    let mut attribute_values = AttributeValues {
        is_root: false,
        rename: None,
        no_encode: false,
    };

    for attr in attrs {
        if !attr.path().is_ident(BASE_NAME) {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(IS_ROOT_ATTRIBUTE) {
                // So it also works like this (both are valid):
                // #[nbt(is_root)]
                // #[nbt(is_root = true)]
                attribute_values.is_root = true;
                if let Ok(value) = meta.value() {
                    attribute_values.is_root = value.parse::<syn::LitBool>()?.value;
                }
            }
            if meta.path.is_ident(RENAME_ATTRIBUTE) {
                attribute_values.rename = Some(meta.value()?.parse::<syn::LitStr>()?.value());
            }
            if meta.path.is_ident(NO_ENCODE_ATTRIBUTE) {
                attribute_values.no_encode = true;
                if let Ok(value) = meta.value() {
                    attribute_values.no_encode = value.parse::<syn::LitBool>()?.value;
                }
            }
            Ok(())
        })
        .unwrap();
    }

    attribute_values
}

/// Parses the attributes of the field and returns the value of the `rename` attribute.
/// @return: rename option
/// Example of usage:
/// ```rust
/// use nbt_derive::NBTSerialize;
///
/// #[derive(NBTSerialize)]
/// pub struct Root {
///   #[nbt(rename = "player_name")]
///   pub x: String,
/// }
pub fn parse_field_attributes(attrs: &[Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        if !attr.path().is_ident(BASE_NAME) {
            return None;
        }

        let mut rename = None;
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(RENAME_ATTRIBUTE) {
                rename = Some(meta.value()?.parse::<syn::LitStr>()?.value());
            }

            Ok(())
        })
        .unwrap();

        rename
    })
}
