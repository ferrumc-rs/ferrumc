use proc_macro::TokenStream;

use crate::helper::{parse_field_attributes, parse_struct_attributes};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

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

        let is_optional = if let syn::Type::Path(path) = field_type {
            path.path.segments.iter().any(|segment| segment.ident == "Option")
        } else {
            false
        };

        if is_optional {
            quote! {
                if let Some(value) = &self.#field_name {
                    // <#field_type as nbt_lib::nbt_spec::serializer::impls::NBTFieldType>::tag_type().serialize(writer)?;
                    // value.tag_type().serialize(writer)?;
                    // <value as nbt_lib::nbt_spec::serializer::impls::NBTFieldType>.tag_type().serialize(writer)?;
                    <#field_type as nbt_lib::nbt_spec::serializer::impls::NBTFieldType>::tag_type(&self.#field_name).serialize(writer)?;
                    #field_name_as_string.serialize(writer)?;
                    value.serialize(writer)?;
                }
            }
        } else {
            quote! {
            // <#field_type as nbt_lib::nbt_spec::serializer::impls::NBTFieldType>::tag_type().serialize(writer)?;
            // self.#field_name.tag_type().serialize(writer)?;
            <#field_type as nbt_lib::nbt_spec::serializer::impls::NBTFieldType>::tag_type(&self.#field_name).serialize(writer)?;

            #field_name_as_string.serialize(writer)?;
            // <#field_type as nbt_lib::nbt_spec::serializer::impls::NBTFieldType>.tag_type().serialize(writer)?;

            self.#field_name.serialize(writer)?;
        }
        }
    });

    let root_header = if is_root {
        quote! {
            nbt_lib::nbt_spec::serializer::tag_types::TAG_COMPOUND.serialize(writer)?;
            #name.serialize(writer)?;
        }
    } else {
        quote! {}
    };

    let serialize_impl = quote! {
        impl ::nbt_lib::NBTSerialize for #struct_name {
            fn serialize<W: std::io::Write>(&self, writer: &mut W) -> ::nbt_lib::NBTResult<()> {
                #root_header
                #(#fields)*
                nbt_lib::nbt_spec::serializer::tag_types::TAG_END.serialize(writer)?;
                Ok(())
            }
        }

        impl nbt_lib::nbt_spec::serializer::impls::NBTFieldType for #struct_name {
            fn tag_type(&self) -> u8 {
                nbt_lib::nbt_spec::serializer::tag_types::TAG_COMPOUND
            }
        }

        impl nbt_lib::nbt_spec::serializer::impls::NBTAnonymousType for #struct_name {
            fn tag_type() -> u8 {
                nbt_lib::nbt_spec::serializer::tag_types::TAG_COMPOUND
            }
        }

        impl nbt_lib::nbt_spec::serializer::NBTCompoundMarker for #struct_name {}
    };


    TokenStream::from(serialize_impl)
}

