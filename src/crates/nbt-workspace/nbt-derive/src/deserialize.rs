use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, parse_macro_input, Type};

use crate::helper::{parse_field_attributes, parse_struct_attributes, AttributeValues};

pub(crate) fn nbt_deserialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let AttributeValues {is_root, rename, .. } = parse_struct_attributes(&input.attrs);
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
            let field_type =  format_generic_type(field_type);
            quote! {
                #field_name: match compound.get(#field_name_str) {
                    Some(tag) => #field_type::read_from(tag)?,
                    None => None,
                },
            }
        } else {
            let field_type =  format_generic_type(field_type);
            quote! {
                #field_name: #field_type::read_from(compound.get(#field_name_str)
                    .ok_or_else(|| ::nbt_lib::NBTError::DeserializeError(format!("Field {} not found", #field_name_str)))?)?,
            }
        };

        deserialize_field
    });

    let root_alignment = if is_root {
        quote! {
            let mut compound = nbt.get(#name)
                .ok_or_else(|| ::nbt_lib::NBTError::DeserializeError(format!("Root `{}` not found", #name)))?;
        }
    } else {
        quote! {
            let mut compound = nbt;
        }
    };

    let deserialize_impl = quote! {
        impl ::nbt_lib::NBTDeserialize for #struct_name {
            fn read_from(mut nbt: nbt_lib::NBTTag) -> ::nbt_lib::NBTResult<Self> {
                #root_alignment

                Ok(Self {
                    #(#field_deserializations)*
                })
            }
        }
    };

    let from_bytes_for_root = if is_root {
        quote! {
            impl ::nbt_lib::NBTDeserializeBytes for #struct_name {
                fn read_from_bytes(cursor: &mut std::io::Cursor<Vec<u8>>) -> ::nbt_lib::NBTResult<Self> {
                    let nbt = ::nbt_lib::read_tag(cursor)?;
                    <Self as ::nbt_lib::NBTDeserialize>::read_from(nbt)
                }
            }
        }
    } else {
        quote! {}
    };

    let deserialize_impl = quote! {
        #deserialize_impl
        #from_bytes_for_root
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

fn format_generic_type(ty: &Type) -> proc_macro2::TokenStream {
    let new_ty = ty.into_token_stream().to_string()
        .replace("<", "::<"); // Vec<i32> -> Vec::<i32>
    syn::parse_str(&new_ty).unwrap()
}