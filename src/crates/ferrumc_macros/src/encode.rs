/*use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

struct FieldAttribs {
    field_name: syn::Ident,
    default_value: Option<syn::Expr>,
    raw_bytes: Option<RawBytes>,

}
struct RawBytes {
    prepend_length: bool,
}

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let mut is_packet_type = false;

    // get the attribute of the struct itself like :


    let mut field_attribs: Vec<FieldAttribs> = Vec::new();

    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => fields,
        _ => panic!("Only structs are supported"),
    };

    for field in fields.named {
        let field_name = field.ident.unwrap();

        if field_name.to_string() == "packet_id" {
            is_packet_type = true;
        }

        let mut field_attrib = FieldAttribs {
            field_name,
            default_value: None,
            raw_bytes: None,
        };

        let attribs = field.attrs;

        for attrib in attribs {
            // [encode(raw_bytes(prepend_length=true))]
            // [encode(raw_bytes)]

            let attrib_name = attrib.path().get_ident().unwrap();

            if attrib_name != "encode" {
                continue;
            }

            attrib
                .parse_nested_meta(|meta| {
                    if meta.path.is_ident("raw_bytes") {
                        let mut prepend = false;

                        meta.parse_nested_meta(|meta| {
                            if meta.path.is_ident("prepend_length") {
                                let value = meta.value().unwrap();
                                let value = value.parse::<syn::LitBool>().unwrap();
                                let value = value.value();
                                prepend = value;
                            }

                            Ok(())
                        })
                        .unwrap();

                        field_attrib.raw_bytes = Some(RawBytes {
                            prepend_length: prepend,
                        });
                    }

                    if meta.path.is_ident("default") {
                        let value = meta.value().unwrap();
                        let value = value.parse::<syn::Expr>().unwrap();
                        field_attrib.default_value = Some(value);
                    }

                    Ok(())
                })
                .unwrap();
        }

        field_attribs.push(field_attrib);
    }

    let mut field_statements = Vec::new();

    for field_attrib in field_attribs {
        let field_name = field_attrib.field_name;

        let mut statement: proc_macro2::TokenStream;

        // declare var names
        let cursor = format_ident!("__cursor_{}", field_name);
        let bytes = format_ident!("__bytes_{}", field_name);
        let len = format_ident!("__len_{}", field_name);

        if let Some(raw_bytes) = field_attrib.raw_bytes {
            statement = quote! {
                let mut #cursor = std::io::Cursor::new(Vec::new());
            };

            if raw_bytes.prepend_length {
                statement = quote! {
                    #statement

                    let #len = self.#field_name.len();
                    let #len = ferrumc_utils::encoding::varint::VarInt::new(#len as i32);

                    #len.encode(&mut #cursor).await?;
                }
            }

            statement = quote! {
                #statement
                tokio::io::AsyncWriteExt::write_all(&mut #cursor, &self.#field_name).await?;

                let mut #bytes = #cursor.into_inner();
                tokio::io::AsyncWriteExt::write_all(bytes, &#bytes).await?;
            };
        } else {
            statement = quote! {
                self.#field_name.encode(bytes).await?;
            };
        }

        field_statements.push(statement);
    }

    let expanded: proc_macro2::TokenStream;

    if is_packet_type {
        expanded = quote! {
            impl ferrumc_utils::type_impls::Encode for #name {
                async fn encode<T>(&self, bytes_out: &mut T) -> std::result::Result<(), ferrumc_utils::error::Error>
                    where
                            T: tokio::io::AsyncWrite + tokio::io::AsyncSeek + std::marker::Unpin
                {
                    use tokio::io::AsyncWriteExt;
                    let mut bytes_ = std::io::Cursor::new(Vec::new());
                    let mut bytes = &mut bytes_;

                    #(#field_statements)*

                    let __packet_data = bytes_.into_inner();

                    let __length = __packet_data.len() as i32;
                    let __length: ferrumc_utils::encoding::varint::VarInt = ferrumc_utils::encoding::varint::VarInt::new(__length);

                    let mut __cursor = std::io::Cursor::new(Vec::new());
                    __length.encode(&mut __cursor).await?;

                    __cursor.write_all(&__packet_data).await?;

                    let __encoded = __cursor.into_inner();
                    bytes_out.write_all(&__encoded).await?;

                    Ok(())
                }
            }
        }
    } else {
        expanded = quote! {
            impl ferrumc_utils::type_impls::Encode for #name {
                async fn encode<T>(&self, bytes: &mut T) -> std::result::Result<(), ferrumc_utils::error::Error>
                    where
                            T: tokio::io::AsyncWrite + tokio::io::AsyncSeek + std::marker::Unpin
                {
                    use tokio::io::AsyncWriteExt;
                    #(#field_statements)*
                    Ok(())
                }
            }
        }
    }


    TokenStream::from(expanded)
}
*/

use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{DeriveInput, Field, parse_macro_input};

struct FieldAttribs {
    field_name: syn::Ident,
    field_type: syn::Type,
    default_value: Option<syn::Expr>,
    raw_bytes: Option<RawBytes>,
}

struct RawBytes {
    prepend_length: bool,
}

fn parse_field_attributes(field: &Field) -> FieldAttribs {
    let field_name = field.ident.clone().unwrap();
    let field_type = field.ty.clone();
    let mut field_attrib = FieldAttribs {
        field_name,
        field_type,
        default_value: None,
        raw_bytes: None,
    };

    for attr in &field.attrs {
        if !attr.path().is_ident("encode") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("raw_bytes") {
                let mut prepend = false;
                meta.parse_nested_meta(|meta| {
                    if meta.path.is_ident("prepend_length") {
                        prepend = meta.value()?.parse::<syn::LitBool>()?.value;
                    }
                    Ok(())
                })?;
                field_attrib.raw_bytes = Some(RawBytes {
                    prepend_length: prepend,
                });
            } else if meta.path.is_ident("default") {
                field_attrib.default_value = Some(meta.value()?.parse()?);
            }
            Ok(())
        })
        .unwrap();
    }

    field_attrib
}

fn generate_field_encode_statement(field_attrib: &FieldAttribs) -> proc_macro2::TokenStream {
    let field_name = &field_attrib.field_name;
    let cursor = format_ident!("__cursor_{}", field_name);
    let bytes = format_ident!("__bytes_{}", field_name);
    let len = format_ident!("__len_{}", field_name);

    if let Some(raw_bytes) = &field_attrib.raw_bytes {
        let mut statement = quote! {
            let mut #cursor = std::io::Cursor::new(Vec::new());
        };

        if raw_bytes.prepend_length {
            statement = quote! {
                #statement
                let #len = self.#field_name.len();
                let #len = crate::utils::encoding::varint::VarInt::new(#len as i32);
                #len.encode(&mut #cursor).await?;
            };
        }

        quote! {
            #statement
            tokio::io::AsyncWriteExt::write_all(&mut #cursor, &self.#field_name).await?;
            let mut #bytes = #cursor.into_inner();
            tokio::io::AsyncWriteExt::write_all(bytes, &#bytes).await?;
        }
    } else {
        quote! {
            self.#field_name.encode(bytes).await?;
        }
    }
}

fn generate_encode_impl(
    name: &syn::Ident,
    field_statements: &[proc_macro2::TokenStream],
    is_packet_type: bool,
) -> proc_macro2::TokenStream {
    if is_packet_type {
        quote! {
            impl crate::utils::type_impls::Encode for #name {
                async fn encode<T>(&self, bytes_out: &mut T) -> std::result::Result<(), crate::utils::error::Error>
                    where T: tokio::io::AsyncWrite + tokio::io::AsyncSeek + std::marker::Unpin
                {
                    use tokio::io::AsyncWriteExt;
                    let mut bytes_ = std::io::Cursor::new(Vec::new());
                    let mut bytes = &mut bytes_;

                    #(#field_statements)*

                    let __packet_data = bytes_.into_inner();
                    let __length = crate::utils::encoding::varint::VarInt::new(__packet_data.len() as i32);
                    let mut __cursor = std::io::Cursor::new(Vec::new());
                    __length.encode(&mut __cursor).await?;
                    __cursor.write_all(&__packet_data).await?;
                    let __encoded = __cursor.into_inner();
                    bytes_out.write_all(&__encoded).await?;

                    Ok(())
                }
            }
        }
    } else {
        quote! {
            impl crate::utils::type_impls::Encode for #name {
                async fn encode<T>(&self, bytes: &mut T) -> std::result::Result<(), crate::utils::error::Error>
                    where T: tokio::io::AsyncWrite + tokio::io::AsyncSeek + std::marker::Unpin
                {
                    use tokio::io::AsyncWriteExt;
                    #(#field_statements)*
                    Ok(())
                }
            }
        }
    }
}

fn generate_modified_constructor(
    name: &syn::Ident,
    field_attribs: &[FieldAttribs],
) -> proc_macro2::TokenStream {
    // example of this:
    // struct Test {
    //     a: i32,
    //     b: i32,
    //     #[encode(default = 5)]
    //     c: i32,
    // }

    // impl Test {
    //     fn new(a: i32, b: i32) -> Self {
    //         Self {
    //             a,
    //             b,
    //             c: 5,
    //         }

    let non_default_fields: Vec<&FieldAttribs> = field_attribs
        .into_iter()
        .filter(|attr| attr.default_value.is_none())
        .collect();
    let non_default_fields_params: Vec<proc_macro2::TokenStream> = non_default_fields
        .iter()
        .map(|attr| {
            let field_name = &attr.field_name;
            let field_type = &attr.field_type;
            quote! {
                #field_name: #field_type,
            }
        })
        .collect();
    let non_default_fields_names: Vec<proc_macro2::TokenStream> = non_default_fields
        .iter()
        .map(|attr| {
            let field_name = &attr.field_name;
            quote! {
                #field_name,
            }
        })
        .collect();

    let default_fields: Vec<&FieldAttribs> = field_attribs
        .iter()
        .filter(|attr| attr.default_value.is_some())
        .collect();

    let default_field_statements = default_fields.iter().map(|attr| {
        let field_name = &attr.field_name;
        let default_value = attr.default_value.as_ref().unwrap();
        quote! {
            #field_name: #default_value,
        }
    });

    quote! {
        impl #name {
            pub fn new_auto(#(#non_default_fields_params)*) -> Self {
                Self {
                    #(#non_default_fields_names)*
                    #(#default_field_statements)*
                }
            }
        }
    }
}

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => fields,
        _ => panic!("Only structs with named fields are supported"),
    };

    let field_attribs: Vec<FieldAttribs> =
        fields.named.iter().map(parse_field_attributes).collect();

    let should_generate_modified_constructor = field_attribs
        .iter()
        .any(|attr| attr.default_value.is_some());
    let mut constructor = quote! {};
    if should_generate_modified_constructor {
        let modified_constructor = generate_modified_constructor(name, &field_attribs);
        constructor = modified_constructor;
    }

    let is_packet_type = field_attribs
        .iter()
        .any(|attr| attr.field_name == "packet_id");

    let field_statements: Vec<proc_macro2::TokenStream> = field_attribs
        .iter()
        .map(generate_field_encode_statement)
        .collect();

    let expanded = generate_encode_impl(name, &field_statements, is_packet_type);

    let final_output = quote! {
        #constructor
        #expanded
    };

    TokenStream::from(final_output)
}
