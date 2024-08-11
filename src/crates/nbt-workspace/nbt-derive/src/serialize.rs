use proc_macro::TokenStream;

use quote::quote;
use syn::{Attribute, Data, DeriveInput, parse_macro_input};

const RENAME_NON_ROOT_ERROR: &str = "Rename attribute can only be used with root attribute, please rename the field name of the parent.";

pub(crate) fn nbt_serialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let (is_root, rename) = parse_struct_attributes(&input.attrs);

    if !is_root && rename.is_some() { panic!("{RENAME_NON_ROOT_ERROR}") }

    let name = rename.clone().unwrap_or_else(|| struct_name.to_string());
    // let name = format_ident!("{}", name);


    let Data::Struct(data) = input.data else {
        panic!("NBTSerialize can only be derived for structs");
    };

    let fields = data.fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_name_as_string = field_name.as_ref().unwrap().to_string();
        let field_type = &f.ty;

        let field_rename = parse_field_attributes(&f.attrs);
        let field_name_as_string = field_rename.unwrap_or_else(|| field_name_as_string);

        let is_optional = if let  syn::Type::Path(path) = field_type {
            path.path.segments.iter().any(|segment| segment.ident == "Option")
        } else {
            false
        };

        if is_optional {
            quote! {
                if let Some(value) = &self.#field_name {
                    <#field_type as nbt_lib::nbt_spec::impls::NBTTag>::tag_type().serialize(writer)?;
                    #field_name_as_string.serialize(writer)?;
                    value.serialize(writer)?;
                }
            }
        } else {
            quote! {
            <#field_type as nbt_lib::nbt_spec::impls::NBTTag>::tag_type().serialize(writer)?;
            #field_name_as_string.serialize(writer)?;
            self.#field_name.serialize(writer)?;
        }
        }
    });

    let root_header = if is_root {
        quote! {
            nbt_lib::nbt_spec::tag_types::TAG_COMPOUND.serialize(writer)?;
            #name.serialize(writer)?;
        }
    } else {
        quote! {}
    };

    let serialize_impl = quote! {
        impl ::nbt_lib::nbt_spec::serializer::NBTSerialize for #struct_name {
            fn serialize<W: std::io::Write>(&self, writer: &mut W) -> ::nbt_lib::NBTResult<()> {
                #root_header
                #(#fields)*
                nbt_lib::nbt_spec::tag_types::TAG_END.serialize(writer)?;
                Ok(())
            }
        }

        impl nbt_lib::nbt_spec::impls::NBTTag for #struct_name {
            fn tag_type() -> u8 {
                nbt_lib::nbt_spec::tag_types::TAG_COMPOUND
            }
        }
    };


    TokenStream::from(serialize_impl)
}

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
fn parse_struct_attributes(attrs: &[Attribute]) -> (bool, Option<String>) {
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
fn parse_field_attributes(attrs: &[Attribute]) -> Option<String> {
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

/*
fn has_root_attribute(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident(IS_ROOT_ATTRIBUTE))
}

const RENAME_ATTRIBUTE: &str = "rename";
fn get_rename_attribute(attrs: &[Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        if !attr.path().is_ident(RENAME_ATTRIBUTE) {
            return None;
        }

        let Ok(Meta::List(meta_list)) = attr.parse_meta() else {
            panic!("Invalid rename attribute");
        };

        let meta_list = meta_list.parse_nested_meta(|attr| {
            let Meta::NameValue(meta_name_value) = attr else {
                None
            };

            Some(meta_name_value.value)
        }).unwrap();


    })
}*/