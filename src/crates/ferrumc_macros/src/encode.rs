use quote::{format_ident, quote};
use syn::DeriveInput;

pub(crate) fn generate_encode_func(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

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

            let meta = attrib.parse_nested_meta(|meta| {
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

                    let #len = #bytes.len();
                    let #len = ferrumc_utils::encoding::varint::VarInt::new(#len as i32);

                    #len.encode(&mut #cursor).await?;
                }
            }

            statement = quote! {
                #statement
                #cursor.write_all(&self.#field_name).await?;

                let #bytes = #cursor.into_inner();
                bytes.write_all(&#bytes).await?;
            };
        } else {
            statement = quote! {
                // <#type_name as Encode>::encode(&self.#ident, &mut bytes).await?;
                <#field_name as Encode>::encode(&self.#field_name, &mut bytes).await?;
            };
        }

        field_statements.push(statement);
    }
    let expanded = quote! {
        impl #name {
            pub async fn encode(&self) -> ferrumc_utils::prelude::Result<Vec<u8>> {
                let mut bytes = std::io::Cursor::new(Vec::new());
                #(#field_statements)*
                Ok(Vec::new())
            }
        }
    };
    expanded
}