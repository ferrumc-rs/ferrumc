use crate::helpers::{get_derive_attributes, StructInfo};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let net_attributes = get_derive_attributes(&input, "net");
    let repr_attr = get_derive_attributes(&input, "repr");
    // check the type of repr attribute
    let repr_attr = {
        let mut repr_type = None;
        repr_attr.iter().for_each(|attr| {
            attr.parse_nested_meta(|meta| {
                let Some(ident) = meta.path.get_ident() else {
                    return Ok(());
                };

                repr_type = Some(ident.to_string());

                Ok(())
            })
            .unwrap();
        });

        repr_type.map(|val| syn::parse_str::<syn::Ident>(&val).expect("Failed to parse repr type"))
    };

    // check if any attribute that has "#[net(u8_cast)]"
    let (type_cast, type_cast_handler) = {
        let mut type_cast = None;
        let mut type_cast_handler = None;
        net_attributes.iter().for_each(|attr| {
            attr.parse_nested_meta(|meta| {
                let Some(ident) = meta.path.get_ident() else {
                    return Ok(());
                };

                match ident.to_string().as_str() {
                    "type_cast" => {
                        let value = meta.value().expect("value failed");
                        let value = value.parse::<LitStr>().expect("parse failed");
                        let n = value.value();
                        type_cast = Some(n);
                    }
                    "type_cast_handler" => {
                        let value = meta.value().expect("value failed");
                        let value = value.parse::<LitStr>().expect("parse failed");
                        let n = value.value();
                        type_cast_handler = Some(n);
                    }
                    &_ => {
                        return Ok(());
                    }
                }

                Ok(())
            })
            .unwrap();
        });

        (type_cast, type_cast_handler)
    };

    // So for enums we can simply read the type and then cast it directly.
    if let Some(type_cast) = type_cast {
        let Some(repr_attr) = repr_attr else {
            panic!(
                "NetDecode with type_cast enabled requires a repr attribute. Example: #[repr(u8)]"
            );
        };

        // in netdecode, read a type of type_cast and then if type_cast_handler exists, use it to do `type_cast_handler(type_cast)`

        let type_cast = syn::parse_str::<syn::Type>(&type_cast).expect("Failed to parse type_cast");

        let StructInfo {
            struct_name: name,
            impl_generics,
            ty_generics,
            where_clause,
            lifetime: _lifetime,
            ..
        } = crate::helpers::extract_struct_info(&input, None);

        let type_cast_handler = match type_cast_handler {
            None => {
                quote! { value }
            }
            Some(handler) => {
                let handler = syn::parse_str::<syn::Expr>(&handler)
                    .expect("Failed to parse type_cast_handler");
                quote! { #handler }
            }
        };

        let enum_arms = if let syn::Data::Enum(data) = &input.data {
            let mut next_discriminant = 0;
            data.variants
                .iter()
                .map(|variant| {
                    let variant_name = &variant.ident;
                    let discriminant = if let Some((_, expr)) = &variant.discriminant {
                        // Use the explicit discriminant
                        quote! { #expr }
                    } else {
                        // Use the next implicit discriminant
                        let disc = quote! { #next_discriminant };
                        next_discriminant += 1;
                        disc
                    };
                    quote! {
                        #discriminant => Ok(#name::#variant_name),
                    }
                })
                .collect::<Vec<_>>()
        } else {
            panic!("NetDecode with type_cast enabled can only be derived for enums.");
        };

        let expanded = quote! {
            impl #impl_generics ferrumc_net_codec::decode::NetDecode for #name #ty_generics #where_clause {
                fn decode<R: std::io::Read>(reader: &mut R, opts: &ferrumc_net_codec::decode::NetDecodeOpts) -> ferrumc_net_codec::decode::NetDecodeResult<Self> {
                    let value = <#type_cast as ferrumc_net_codec::decode::NetDecode>::decode(reader, opts)?;
                    let value = #type_cast_handler;
                    let value = value as #repr_attr;
                    match (value as i32) {
                        #(#enum_arms)*
                        _ => Err(ferrumc_net_codec::decode::errors::NetDecodeError::InvalidEnumVariant),
                    }
                }
            }
        };

        return TokenStream::from(expanded);
    }

    let fields = if let syn::Data::Struct(data) = &input.data {
        &data.fields
    } else {
        panic!("NetDecode can only be derived for structs or enums with u8_cast enabled.");
    };

    let decode_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        quote! {
            #field_name: <#field_ty as ferrumc_net_codec::decode::NetDecode>::decode(reader, opts)?,
        }
    });

    let StructInfo {
        struct_name,
        impl_generics,
        ty_generics,
        where_clause,
        lifetime: _lifetime,
        ..
    } = crate::helpers::extract_struct_info(&input, None);

    let expanded = quote! {
        // impl ferrumc_net_codec::decode::NetDecode for #name {
        impl #impl_generics ferrumc_net_codec::decode::NetDecode for #struct_name #ty_generics #where_clause {
            fn decode<R: std::io::Read>(reader: &mut R, opts: &ferrumc_net_codec::decode::NetDecodeOpts) -> ferrumc_net_codec::decode::NetDecodeResult<Self> {
                Ok(Self {
                    #(#decode_fields)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
