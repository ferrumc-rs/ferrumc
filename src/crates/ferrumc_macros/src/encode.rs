use quote::{format_ident, quote};
use syn::DeriveInput;

pub(crate) fn generate_encode_func(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let mut is_packet_type = false;

    struct FieldAttribs {
        field_name: syn::Ident,
        raw_bytes: Option<RawBytes>,
    }
    struct RawBytes {
        prepend_length: bool,
    }

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

            attrib.parse_nested_meta(|meta| {
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
                    }).unwrap();

                    field_attrib.raw_bytes = Some(RawBytes {
                        prepend_length: prepend,
                    });
                }

                Ok(())
            }).unwrap();
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
    }else {
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
    expanded
}