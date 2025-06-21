use crate::helpers::{extract_struct_info, get_derive_attributes, StructInfo};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Collect attributes relevant to our `net(...)` usage:
    let net_attributes = get_derive_attributes(&input, "net");
    let repr_attr = get_derive_attributes(&input, "repr");

    // Attempt to parse the `#[repr(...)]` attribute if it exists.
    let repr_type = {
        let mut repr_t = None;
        for attr in &repr_attr {
            attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    repr_t = Some(ident.to_string());
                }
                Ok(())
            })
            .unwrap();
        }
        repr_t.map(|val| syn::parse_str::<syn::Ident>(&val).expect("Failed to parse repr type"))
    };

    // Look for `#[net(type_cast = "X", type_cast_handler = "Y")]` usage for enum casting.
    let (type_cast, type_cast_handler) = {
        let mut cast = None;
        let mut cast_handler = None;
        for attr in &net_attributes {
            attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    match ident.to_string().as_str() {
                        "type_cast" => {
                            let value = meta.value().expect("Missing type_cast value");
                            let value = value.parse::<LitStr>().expect("Failed to parse type_cast");
                            cast = Some(value.value());
                        }
                        "type_cast_handler" => {
                            let value = meta.value().expect("Missing type_cast_handler value");
                            let value = value
                                .parse::<LitStr>()
                                .expect("Failed to parse type_cast_handler");
                            cast_handler = Some(value.value());
                        }
                        _ => {}
                    }
                }
                Ok(())
            })
            .unwrap();
        }
        (cast, cast_handler)
    };

    // If `type_cast` is present, we assume this is an enum. We'll decode by reading
    // the specified type, then casting into the enum.
    if let Some(type_cast) = type_cast {
        let Some(repr_ident) = repr_type else {
            panic!("NetDecode with type_cast requires a repr attribute. Example: #[repr(u8)]");
        };

        let type_cast_ty =
            syn::parse_str::<syn::Type>(&type_cast).expect("Failed to parse type_cast as a type");

        let StructInfo {
            struct_name: enum_name,
            impl_generics,
            ty_generics,
            where_clause,
            ..
        } = extract_struct_info(&input, None);

        let cast_handler_expr = match type_cast_handler {
            None => quote!(value),
            Some(handler_str) => {
                let handler_expr = syn::parse_str::<syn::Expr>(&handler_str)
                    .expect("Failed to parse type_cast_handler");
                quote!(#handler_expr)
            }
        };

        // Build match arms for each variant's discriminant (explicit or implicit).
        let enum_arms = if let syn::Data::Enum(data) = &input.data {
            let mut next_disc = 0;
            data.variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    // If the variant has a discriminant (e.g., `Variant = 5`), use that.
                    // Otherwise, use the running `next_disc`.
                    let disc_expr = if let Some((_, disc)) = &variant.discriminant {
                        quote! { #disc }
                    } else {
                        let disc_token = quote! { #next_disc };
                        next_disc += 1;
                        disc_token
                    };
                    quote! {
                        #disc_expr => Ok(#enum_name::#variant_ident),
                    }
                })
                .collect::<Vec<_>>()
        } else {
            panic!("`#[net(type_cast = ...)]` is only valid on enums.");
        };

        let expanded = quote! {
            impl #impl_generics ferrumc_net_codec::decode::NetDecode
                for #enum_name #ty_generics
                #where_clause
            {
                fn decode<R: std::io::Read>(
                    reader: &mut R,
                    opts: &ferrumc_net_codec::decode::NetDecodeOpts
                ) -> ferrumc_net_codec::decode::NetDecodeResult<Self> {
                    // Decode the initial numeric value
                    let value = <#type_cast_ty as ferrumc_net_codec::decode::NetDecode>::decode(reader, opts)?;
                    // Possibly transform via the handler
                    let value = #cast_handler_expr;
                    // Cast to the repr type
                    let value = value as #repr_ident;

                    // Match against the known variant discriminants
                    match (value as i32) {
                        #(#enum_arms)*
                        _ => Err(ferrumc_net_codec::decode::errors::NetDecodeError::InvalidEnumVariant),
                    }
                }

                async fn decode_async<R: tokio::io::AsyncRead + Unpin>(
                    reader: &mut R,
                    opts: &ferrumc_net_codec::decode::NetDecodeOpts
                ) -> ferrumc_net_codec::decode::NetDecodeResult<Self> {
                    // Decode the initial numeric value
                    let value = <#type_cast_ty as ferrumc_net_codec::decode::NetDecode>::decode_async(reader, opts).await?;
                    // Possibly transform via the handler
                    let value = #cast_handler_expr;
                    // Cast to the repr type
                    let value = value as #repr_ident;

                    // Match against the known variant discriminants
                    match (value as i32) {
                        #(#enum_arms)*
                        _ => Err(ferrumc_net_codec::decode::errors::NetDecodeError::InvalidEnumVariant),
                    }
                }
            }
        };
        return TokenStream::from(expanded);
    }

    // Otherwise, handle struct decoding. We'll check if each field has an optional trigger.
    let StructInfo {
        struct_name,
        impl_generics,
        ty_generics,
        where_clause,
        ..
    } = extract_struct_info(&input, None);

    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("NetDecode can only be derived for structs or for enums with `u8_cast`."),
    };

    // Generate per-field decode statements. We'll build them in order, storing
    // them in local variables named the same as the field, so the subsequent fields
    // can use them in the optional triggers if needed.
    let mut decode_statements = Vec::new();
    let mut async_decode_statements = Vec::new();
    let mut field_names = Vec::new();

    for field in fields {
        let field_name = field
            .ident
            .clone()
            .expect("Unnamed fields are not currently supported");
        let field_ty = &field.ty;

        // Check for optional trigger attribute: `#[net(optional_trigger = "...expr...")]`
        // or something like `#[net(optional_trigger = { some_field == true })]`.
        let mut optional_trigger_expr: Option<syn::Expr> = None;

        // Check the `net(...)` attributes on this field
        for attr in &field.attrs {
            if attr.path().is_ident("net") {
                // e.g., #[net(optional_trigger = { some_field == true })]

                attr.parse_nested_meta(|meta| {
                    if let Some(ident) = meta.path.get_ident() {
                        if ident.to_string().as_str() == "optional_trigger" {
                            meta.parse_nested_meta(|meta| {
                                if let Some(expr) = meta.path.get_ident() {
                                    let val = syn::parse_str::<syn::Expr>(&expr.to_string())
                                        .expect("Failed to parse optional_trigger expression");

                                    optional_trigger_expr = Some(val);
                                } else {
                                    panic!("Expected an expression for optional_trigger");
                                }

                                Ok(())
                            })
                            .expect("Failed to parse optional_trigger expression");
                        }
                    }
                    Ok(())
                })
                .unwrap();
            }
        }

        // Generate decoding code depending on whether there's an optional trigger
        if let Some(expr) = optional_trigger_expr {
            // For an optional field, we decode it only if `expr` is true at runtime.
            // We'll store the result in a local variable `field_name` which will be an Option<T>.
            // Then at the end, we can build the struct using those local variables.
            decode_statements.push(quote! {
                let #field_name = {
                    if #expr {
                        Some(<#field_ty as ferrumc_net_codec::decode::NetDecode>::decode(reader, opts)?)
                    } else {
                        None
                    }
                };
            });

            // For async decoding, we need to handle the async case as well.
            async_decode_statements.push(quote! {
                let #field_name = {
                    if #expr {
                        Some(<#field_ty as ferrumc_net_codec::decode::NetDecode>::decode_async(reader, opts).await?)
                    } else {
                        None
                    }
                };
            });
        } else {
            // Check if the field is an Option<T> and handle it accordingly.
            let is_optional = {
                let ty_str = quote! { #field_ty }.to_string();
                ty_str.contains("Option<")
            };

            if is_optional {
                decode_statements.push(quote! {
                    compile_error!("Optional fields must have an `optional_trigger` attribute\n\
                        Example: #[net(optional_trigger = { some_field == true })]");
                });
                async_decode_statements.push(quote! {
                    compile_error!("Optional fields must have an `optional_trigger` attribute\n\
                        Example: #[net(optional_trigger = { some_field == true })]");
                });
            }

            // Normal (non-optional) field decode:
            decode_statements.push(quote! {
                let #field_name = <#field_ty as ferrumc_net_codec::decode::NetDecode>::decode(reader, opts)?;
            });

            async_decode_statements.push(quote! {
                let #field_name = <#field_ty as ferrumc_net_codec::decode::NetDecode>::decode_async(reader, opts).await?;
            });
        }

        field_names.push(field_name);
    }

    // After decoding everything into local variables, construct the struct.
    let build_struct = quote! {
        Ok(Self {
            #(#field_names),*
        })
    };

    let expanded = quote! {
        impl #impl_generics ferrumc_net_codec::decode::NetDecode
            for #struct_name #ty_generics
            #where_clause
        {
            fn decode<R: std::io::Read>(
                reader: &mut R,
                opts: &ferrumc_net_codec::decode::NetDecodeOpts
            ) -> ferrumc_net_codec::decode::NetDecodeResult<Self> {
                #(#decode_statements)*

                #build_struct
            }

            async fn decode_async<R: tokio::io::AsyncRead + Unpin>(
                reader: &mut R,
                opts: &ferrumc_net_codec::decode::NetDecodeOpts
            ) -> ferrumc_net_codec::decode::NetDecodeResult<Self> {
                #(#async_decode_statements)*

                #build_struct
            }
        }
    };

    TokenStream::from(expanded)
}
