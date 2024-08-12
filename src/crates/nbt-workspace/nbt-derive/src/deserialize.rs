use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input, Type};

use crate::helper::{parse_field_attributes, parse_struct_attributes};

pub(crate) fn nbt_deserialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let (is_root, rename) = parse_struct_attributes(&input.attrs);
    let name = rename.clone().unwrap_or_else(|| struct_name.to_string());

    let Data::Struct(data_struct) = &input.data else {
        panic!("NBTDeserialize can only be derived for structs");
    };

    let Fields::Named(fields_named) = &data_struct.fields else {
        panic!("NBTDeserialize can only be derived for structs with named fields");
    };

    let field_deserializations = fields_named.named.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let field_name_str = field_name.as_ref().unwrap().to_string();

        let field_rename = parse_field_attributes(&field.attrs);
        let field_name_str = field_rename.unwrap_or(field_name_str);

        let deserialize_field = if is_optional_type(field_type) {
            quote! {
                #field_name: match compound.get(#field_name_str) {
                    Some(tag) => Some(#field_type::deserialize(tag)?),
                    None => None,
                },
            }
        } else {
            quote! {
                #field_name: #field_type::deserialize(compound.get(#field_name_str)
                    .ok_or_else(|| NBTError::DeserializeError(format!("Field {} not found", #field_name_str)))?)?,
            }
        };

        deserialize_field
    });

    let deserialize_impl = quote! {
            impl NBTDeserialize for #struct_name {
                fn deserialize(tag:NBTTag) -> NBTResult<Self> {
                    let NBTTag::Compound(compound) = tag else {
                        return Err(NBTError::DeserializeError("Expected compound".to_string()));
                    };
                    Ok(Self {
                        #(#field_deserializations)*
                    })
                }
            }
        };

    TokenStream::from(deserialize_impl)
}

fn is_optional_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}
