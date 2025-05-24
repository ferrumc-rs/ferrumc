use crate::helpers::{get_derive_attributes, StructInfo};
use crate::net::packets::get_packet_details_from_attributes;
use crate::static_loading::packets::PacketBoundiness;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

// Generate packet ID encoding snippets
fn generate_packet_id_snippets(
    packet_id: Option<u8>,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let sync_snippet = if let Some(id) = packet_id {
        quote! {
            <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&#id.into(), writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
        }
    } else {
        quote! {}
    };

    let async_snippet = if let Some(id) = packet_id {
        quote! {
            <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&#id.into(), writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
        }
    } else {
        quote! {}
    };

    (sync_snippet, async_snippet)
}

// Generate field encoding expressions for structs
fn generate_field_encoders(fields: &syn::Fields) -> proc_macro2::TokenStream {
    let encode_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        quote! {
            <#field_ty as ferrumc_net_codec::encode::NetEncode>::encode(&self.#field_name, writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
        }
    });
    quote! { #(#encode_fields)* }
}

fn generate_async_field_encoders(fields: &syn::Fields) -> proc_macro2::TokenStream {
    let encode_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        quote! {
            <#field_ty as ferrumc_net_codec::encode::NetEncode>::encode_async(&self.#field_name, writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
        }
    });
    quote! { #(#encode_fields)* }
}

// Generate enum variant encoding using static dispatch
fn generate_enum_encoders(
    data: &syn::DataEnum,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let variants = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        match &variant.fields {
            Fields::Named(fields) => {
                let field_idents: Vec<_> = fields.named.iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect();
                let field_tys: Vec<_> = fields.named.iter()
                    .map(|f| &f.ty)
                    .collect();

                (quote! {
                    Self::#variant_ident { #(#field_idents),* } => {
                        #(
                            <#field_tys as ferrumc_net_codec::encode::NetEncode>::encode(#field_idents, writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                        )*
                    }
                },
                 quote! {
                    Self::#variant_ident { #(#field_idents),* } => {
                        #(
                            <#field_tys as ferrumc_net_codec::encode::NetEncode>::encode_async(#field_idents, writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                        )*
                    }
                })
            }
            Fields::Unnamed(fields) => {
                let field_names: Vec<_> = (0..fields.unnamed.len())
                    .map(|i| syn::Ident::new(&format!("field{i}"), proc_macro2::Span::call_site()))
                    .collect();
                let field_tys: Vec<_> = fields.unnamed.iter()
                    .map(|f| &f.ty)
                    .collect();

                (quote! {
                    Self::#variant_ident(#(#field_names),*) => {
                        #(
                            <#field_tys as ferrumc_net_codec::encode::NetEncode>::encode(#field_names, writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                        )*
                    }
                },
                 quote! {
                    Self::#variant_ident(#(#field_names),*) => {
                        #(
                            <#field_tys as ferrumc_net_codec::encode::NetEncode>::encode_async(#field_names, writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                        )*
                    }
                })
            }
            Fields::Unit => (
                quote! {
                    Self::#variant_ident => {}
                },
                quote! {
                    Self::#variant_ident => {}
                }
            ),
        }
    }).unzip::<_, _, Vec<_>, Vec<_>>();

    let (sync_variants, async_variants) = variants;

    (
        quote! {
            match self {
                #(#sync_variants)*
            }
        },
        quote! {
            match self {
                #(#async_variants)*
            }
        },
    )
}

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let packet_attr = get_derive_attributes(&input, "packet");
    let (packet_id_snippet, async_packet_id_snippet) = generate_packet_id_snippets(
        get_packet_details_from_attributes(packet_attr.as_slice(), PacketBoundiness::Clientbound)
            .unzip()
            .1,
    );

    let (sync_impl, async_impl) = match &input.data {
        syn::Data::Struct(data) => {
            let field_encoders = generate_field_encoders(&data.fields);
            let async_field_encoders = generate_async_field_encoders(&data.fields);

            (
                quote! {
                    fn encode<W: std::io::Write>(&self, writer: &mut W, opts: &ferrumc_net_codec::encode::NetEncodeOpts) -> ferrumc_net_codec::encode::NetEncodeResult<()> {
                        match opts {
                            ferrumc_net_codec::encode::NetEncodeOpts::None => {
                                #packet_id_snippet
                                #field_encoders
                            }
                            ferrumc_net_codec::encode::NetEncodeOpts::WithLength => {
                                let actual_writer = writer;
                                let mut writer = Vec::new();
                                let mut writer = &mut writer;

                                #packet_id_snippet
                                #field_encoders

                                let len: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();
                                <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&len, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                                actual_writer.write_all(writer)?;
                            }
                            ferrumc_net_codec::encode::NetEncodeOpts::Compressed => {
                                let actual_writer = writer;
                                let mut writer = Vec::new();
                                let mut writer = &mut writer;

                                let compression_threshold = ferrumc_config::statics::get_global_config().network_compression_threshold;

                                #packet_id_snippet
                                #field_encoders

                                if writer.len() >= compression_threshold as usize {
                                    let data_length: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();

                                    let mut compressed_data = Vec::new();
                                    {
                                        let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                                        e.write_all(writer)?;
                                        compressed_data = e.finish()?;
                                    }

                                    let packet_length: ferrumc_net_codec::net_types::var_int::VarInt = (data_length.len() + compressed_data.len()).into();

                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&packet_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&data_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                                    actual_writer.write_all(&compressed_data)?;
                                } else {
                                    let data_length: ferrumc_net_codec::net_types::var_int::VarInt = 0.into();
                                    let packet_length: ferrumc_net_codec::net_types::var_int::VarInt = (data_length.len() + writer.len()).into();

                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&packet_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&data_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                                    actual_writer.write_all(writer)?;
                                }
                            },
                            e => unimplemented!("Unsupported option for NetEncode: {:?}", e),
                        }
                        Ok(())
                    }
                },
                quote! {
                    async fn encode_async<W: tokio::io::AsyncWrite + std::marker::Unpin>(&self, writer: &mut W, opts: &ferrumc_net_codec::encode::NetEncodeOpts) -> ferrumc_net_codec::encode::NetEncodeResult<()> {
                        match opts {
                            ferrumc_net_codec::encode::NetEncodeOpts::None => {
                                #async_packet_id_snippet
                                #async_field_encoders
                            }
                            ferrumc_net_codec::encode::NetEncodeOpts::WithLength => {
                                let actual_writer = writer;
                                let mut writer = Vec::new();
                                let mut writer = &mut writer;

                                #async_packet_id_snippet
                                #field_encoders

                                let len: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();
                                <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&len, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                                <W as tokio::io::AsyncWriteExt>::write_all(actual_writer, writer).await?;
                            }
                            ferrumc_net_codec::encode::NetEncodeOpts::Compressed => {
                                let actual_writer = writer;
                                let mut writer = Vec::new();
                                let mut writer = &mut writer;

                                let compression_threshold = ferrumc_config::statics::get_global_config().network_compression_threshold;

                                #async_packet_id_snippet
                                #async_field_encoders

                                if writer.len() >= compression_threshold as usize {
                                    let data_length: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();

                                    let mut compressed_data = Vec::new();
                                    {
                                        let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                                        e.write_all(writer)?;
                                        compressed_data = e.finish()?;
                                    }

                                    let packet_length: ferrumc_net_codec::net_types::var_int::VarInt = (data_length.len() + compressed_data.len()).into();

                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&packet_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                                     <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&data_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                                    // actual_writer.write_all(&compressed_data).await?;
                                     <W as tokio::io::AsyncWriteExt>::write_all(actual_writer, &compressed_data).await?;
                                } else {
                                    let data_length: ferrumc_net_codec::net_types::var_int::VarInt = 0.into();
                                    let packet_length: ferrumc_net_codec::net_types::var_int::VarInt = (data_length.len() + writer.len()).into();

                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&packet_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&data_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                                    // actual_writer.write_all(writer).await?;
                                    <W as tokio::io::AsyncWriteExt>::write_all(actual_writer, writer).await?;
                                }
                            },
                            _ => unimplemented!("Unsupported options for NetEncode"),
                        }
                        Ok(())
                    }
                },
            )
        }
        syn::Data::Enum(data) => {
            let (sync_enum_encoder, async_enum_encoder) = generate_enum_encoders(data);

            (
                quote! {
                    fn encode<W: std::io::Write>(&self, writer: &mut W, opts: &ferrumc_net_codec::encode::NetEncodeOpts) -> ferrumc_net_codec::encode::NetEncodeResult<()> {
                        match opts {
                            ferrumc_net_codec::encode::NetEncodeOpts::None => {
                                #packet_id_snippet
                                #sync_enum_encoder
                            }
                            ferrumc_net_codec::encode::NetEncodeOpts::WithLength => {
                                let actual_writer = writer;
                                let mut writer = Vec::new();
                                let mut writer = &mut writer;

                                #packet_id_snippet
                                #sync_enum_encoder

                                let len: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();
                                <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&len, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                                actual_writer.write_all(writer)?;
                            }
                            ferrumc_net_codec::encode::NetEncodeOpts::Compressed => {
                                let actual_writer = writer;
                                let mut writer = Vec::new();
                                let mut writer = &mut writer;

                                let compression_threshold = ferrumc_config::statics::get_global_config().network_compression_threshold;

                                #packet_id_snippet
                                #sync_enum_encoder

                                if writer.len() >= compression_threshold as usize {
                                    let data_length: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();

                                    let mut compressed_data = Vec::new();
                                    {
                                        let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                                        <flate2::write::ZlibEncoder<Vec<u8>> as std::io::Write>::write_all(&mut e, writer)?;
                                        compressed_data = e.finish()?;
                                    }

                                    let packet_length: ferrumc_net_codec::net_types::var_int::VarInt = (data_length.len() + compressed_data.len()).into();

                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&packet_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&data_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                                    actual_writer.write_all(&compressed_data)?;
                                } else {
                                    let data_length: ferrumc_net_codec::net_types::var_int::VarInt = 0.into();
                                    let packet_length: ferrumc_net_codec::net_types::var_int::VarInt = (data_length.len() + writer.len()).into();

                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&packet_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&data_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                                    actual_writer.write_all(writer)?;
                                }
                            },
                            e => unimplemented!("Unsupported option for NetEncode: {:?}", e),
                        }
                        Ok(())
                    }
                },
                quote! {
                    async fn encode_async<W: tokio::io::AsyncWrite + std::marker::Unpin>(&self, writer: &mut W, opts: &ferrumc_net_codec::encode::NetEncodeOpts) -> ferrumc_net_codec::encode::NetEncodeResult<()> {
                        match opts {
                            ferrumc_net_codec::encode::NetEncodeOpts::None => {
                                #async_packet_id_snippet
                                #async_enum_encoder
                            }
                            ferrumc_net_codec::encode::NetEncodeOpts::WithLength => {
                                let actual_writer = writer;
                                let mut writer = Vec::new();
                                let mut writer = &mut writer;

                                #async_packet_id_snippet
                                #sync_enum_encoder

                                let len: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();
                                <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&len, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                                <W as tokio::io::AsyncWriteExt>::write_all(actual_writer, writer).await?;
                            }
                            ferrumc_net_codec::encode::NetEncodeOpts::Compressed => {
                                let actual_writer = writer;
                                let mut writer = Vec::new();
                                let mut writer = &mut writer;

                                let compression_threshold = ferrumc_config::statics::get_global_config().network_compression_threshold;

                                #async_packet_id_snippet
                                #async_enum_encoder

                                if writer.len() >= compression_threshold as usize {
                                    let data_length: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();

                                    let mut compressed_data = Vec::new();
                                    {
                                        let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                                        <flate2::write::ZlibEncoder<Vec<u8>> as std::io::Write>::write_all(&mut e, writer)?;
                                        compressed_data = e.finish()?;
                                    }

                                    let packet_length: ferrumc_net_codec::net_types::var_int::VarInt = (data_length.len() + compressed_data.len()).into();

                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&packet_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&data_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                                     <W as tokio::io::AsyncWriteExt>::write_all(actual_writer, &compressed_data).await?;
                                } else {
                                    let data_length: ferrumc_net_codec::net_types::var_int::VarInt = 0.into();
                                    let packet_length: ferrumc_net_codec::net_types::var_int::VarInt = (data_length.len() + writer.len()).into();

                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&packet_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                                    <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&data_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                                    <W as tokio::io::AsyncWriteExt>::write_all(actual_writer, writer).await?;
                                }
                            },
                            _ => unimplemented!("Unsupported options for NetEncode"),
                        }
                        Ok(())
                    }
                },
            )
        }
        _ => unimplemented!("NetEncode can only be derived for structs and enums"),
    };

    let StructInfo {
        struct_name,
        impl_generics,
        ty_generics,
        where_clause,
        lifetime: _lifetime,
        ..
    } = crate::helpers::extract_struct_info(&input, None);

    TokenStream::from(quote! {
        impl #impl_generics ferrumc_net_codec::encode::NetEncode for #struct_name #ty_generics #where_clause {
            #sync_impl
            #async_impl
        }
    })
}
