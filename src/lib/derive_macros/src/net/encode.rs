use crate::helpers::{get_derive_attributes, StructInfo};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitInt};

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = if let syn::Data::Struct(data) = &input.data {
        &data.fields
    } else {
        unimplemented!("NetEncode can only be derived for structs");
    };

    let packet_attr = get_derive_attributes(&input, "packet");

    let (packet_id_snippet, async_packet_id_snippet) = {
        let mut packet_id = None;
        packet_attr.iter().for_each(|attr| {
            attr.parse_nested_meta(|meta| {
                let Some(ident) = meta.path.get_ident() else {
                    return Ok(());
                };

                match ident.to_string().as_str() {
                    "packet_id" => {
                        let value = meta.value().expect("value failed");
                        let value = value.parse::<LitInt>().expect("parse failed");
                        let n = value.base10_parse::<u8>().expect("base10_parse failed");
                        packet_id = Some(n);
                    }
                    &_ => {
                        return Ok(());
                    }
                }

                Ok(())
            }).unwrap();
        });

        let sync_impl = if let Some(packet_id) = packet_id {
            quote! {
                // encode a varint of packet_id
                // varint is at ferrumc_net_codec::net_types::var_int::VarInt
                <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&#packet_id.into(), writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
            }
        } else {
            quote! {  }
        };

        let async_impl = if let Some(packet_id) = packet_id {
            quote! {
                // encode a varint of packet_id
                // varint is at ferrumc_net_codec::net_types::var_int::VarInt
                <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&#packet_id.into(), writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
            }
        } else {
            quote! {  }
        };

        (sync_impl, async_impl)
    };

    let sync_encode_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        quote! {
            // TODO: see if we need to pass options here
            <#field_ty as ferrumc_net_codec::encode::NetEncode>::encode(&self.#field_name, writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
        }
    });


    let sync_impl = {
        // These exist only because we cannot move value LMAO, they're both the same
        let normal_fields = sync_encode_fields.clone();
        let length_prefixed_fields = sync_encode_fields.clone();
        let compressed_fields = sync_encode_fields.clone();

        quote! {
            fn encode<W: std::io::Write>(&self, writer: &mut W, opts: &ferrumc_net_codec::encode::NetEncodeOpts) -> ferrumc_net_codec::encode::NetEncodeResult<()> {
                match opts {
                    ferrumc_net_codec::encode::NetEncodeOpts::None => {
                        #packet_id_snippet
                        #(#normal_fields)*
                    }
                    ferrumc_net_codec::encode::NetEncodeOpts::WithLength => {
                        let actual_writer = writer;
                        let mut writer = Vec::new();
                        let mut writer = &mut writer;


                        #packet_id_snippet
                        #(#length_prefixed_fields)*


                        // let len = writer.len();
                        // len.encode(actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                        let len: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();
                        <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&len, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                        actual_writer.write_all(writer)?;
                    }
                    ferrumc_net_codec::encode::NetEncodeOpts::Compressed => {
                        // Check https://wiki.vg/Protocol#Packet_format for protocol info

                        // unimplemented!("NetEncodeOpts::Compressed is not yet implemented");

                        let actual_writer = writer;
                        let mut writer = Vec::new(); // Packet Data including Packet ID
                        let mut writer = &mut writer;

                        // Get compression threshold from config
                        let compression_threshold = ferrumc_config::get_global_config().network_compression_threshold;

                        #packet_id_snippet
                        #(#compressed_fields)*

                        // if size >= threshold, compress, otherwise, send uncompressed and set Data Length to 0
                        if writer.len() >= compression_threshold as usize {
                            // Packet Length - Uncompressed
                            // Data Length   - Uncompressed
                            // Packet ID     - Compressed
                            // Data          - Compressed

                            // Data length is set to uncompressed data length
                            let data_length: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();

                            // Compress Packet ID and Data
                            let mut compressed_data = Vec::new();
                            {
                                // Scope for encoder
                                let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                                e.write_all(writer)?;
                                compressed_data = e.finish()?;
                            }

                            let packet_length: ferrumc_net_codec::net_types::var_int::VarInt = (data_length.len + compressed_data.len()).into();

                            // Write
                            <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&packet_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                            <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&data_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                            actual_writer.write_all(&compressed_data)?;
                        } else {
                            // Everything is uncompressed

                            // Data Length always set to 0
                            let data_length: ferrumc_net_codec::net_types::var_int::VarInt = 0.into();

                            let packet_length: ferrumc_net_codec::net_types::var_int::VarInt = (data_length.len + writer.len()).into();

                            // Write
                            <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&packet_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                            <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&data_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                            actual_writer.write_all(writer)?;
                        }
                    },
                    e => unimplemented!("Unsupported option for NetEncode: {:?}", e),
                }

                Ok(())
            }
        }
    };

    let async_impl = {
        let encode_fields = fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_ty = &field.ty;
            quote! {
            // TODO: see if we need to pass options here
                <#field_ty as ferrumc_net_codec::encode::NetEncode>::encode_async(&self.#field_name, writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
            }
        });

        // These are the same. Only because we cannot move value!!
        let normal_fields = encode_fields.clone();
        // no await overhead here, since we're writing to a buffer(Vec) first.
        let length_prefixed_fields = sync_encode_fields.clone();
        let compressed_fields = encode_fields.clone();

        quote! {
            async fn encode_async<W: tokio::io::AsyncWrite + std::marker::Unpin>(&self, writer: &mut W, opts: &ferrumc_net_codec::encode::NetEncodeOpts) -> ferrumc_net_codec::encode::NetEncodeResult<()> {
                match opts {
                    ferrumc_net_codec::encode::NetEncodeOpts::None => {
                        #async_packet_id_snippet
                        #(#normal_fields)*
                    }
                    ferrumc_net_codec::encode::NetEncodeOpts::WithLength => {
                        // Write to a buffer first, then write the length and the buffer to the actual writer
                        let actual_writer = writer;
                        let mut writer = Vec::new();
                        let mut writer = &mut writer;

                        #async_packet_id_snippet
                        #(#length_prefixed_fields)*

                        let len: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();
                        <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&len, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                        // actual_writer.write_all(writer).await?;
                        <W as tokio::io::AsyncWriteExt>::write_all(actual_writer, writer).await?;
                    }
                    ferrumc_net_codec::encode::NetEncodeOpts::Compressed => {
                        // Check https://wiki.vg/Protocol#Packet_format for protocol info

                        // unimplemented!("NetEncodeOpts::Compressed is not yet implemented");

                        let actual_writer = writer;
                        let mut writer = Vec::new(); // Packet Data including Packet ID
                        let mut writer = &mut writer;

                        // Get compression threshold from config
                        let compression_threshold = ferrumc_config::get_global_config().network_compression_threshold;

                        #async_packet_id_snippet
                        #(#compressed_fields)*

                        // if size >= threshold, compress, otherwise, send uncompressed and set Data Length to 0
                        if writer.len() >= compression_threshold as usize {
                            // Packet Length - Uncompressed
                            // Data Length   - Uncompressed
                            // Packet ID     - Compressed
                            // Data          - Compressed

                            // Data length is set to uncompressed data length
                            let data_length: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();

                            // Compress Packet ID and Data
                            let mut compressed_data = Vec::new();
                            {
                                // Scope for encoder
                                let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                                e.write_all(writer)?;
                                compressed_data = e.finish()?;
                            }

                            let packet_length: ferrumc_net_codec::net_types::var_int::VarInt = (data_length.len + compressed_data.len()).into();

                            // Write
                            <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&packet_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                            <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&data_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                            actual_writer.write_all(&compressed_data).await?;
                        } else {
                            // Everything is uncompressed

                            // Data Length always set to 0
                            let data_length: ferrumc_net_codec::net_types::var_int::VarInt = 0.into();

                            let packet_length: ferrumc_net_codec::net_types::var_int::VarInt = (data_length.len + writer.len()).into();

                            // Write
                            <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&packet_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                            <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&data_length, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                            actual_writer.write_all(writer).await?;
                        }
                    },
                    _ => unimplemented!("Unsupported options for NetEncode"),
                }


                Ok(())
            }
        }
    };

    let StructInfo {
        struct_name,
        impl_generics,
        ty_generics,
        where_clause,
        lifetime: _lifetime,
        ..
    } = crate::helpers::extract_struct_info(&input, None);

    let expanded = quote! {
        // use std::io::Write;
        // use tokio::io::AsyncWriteExt;
        impl #impl_generics ferrumc_net_codec::encode::NetEncode for #struct_name #ty_generics #where_clause {
            #sync_impl
            #async_impl
        }
    };

    TokenStream::from(expanded)
}
