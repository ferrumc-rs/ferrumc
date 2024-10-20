use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use crate::helpers::StructInfo;

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = if let syn::Data::Struct(data) = &input.data {
        &data.fields
    } else {
        unimplemented!("NetEncode can only be derived for structs");
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

        quote! {
            fn encode<W: std::io::Write>(&self, writer: &mut W, opts: &ferrumc_net_codec::encode::NetEncodeOpts) -> ferrumc_net_codec::encode::NetEncodeResult<()> {
                match opts {
                    ferrumc_net_codec::encode::NetEncodeOpts::None => {
                        #(#normal_fields)*
                    }
                    ferrumc_net_codec::encode::NetEncodeOpts::WithLength => {
                        let actual_writer = writer;
                        let mut writer = Vec::new();
                        let mut writer = &mut writer;

                        #(#length_prefixed_fields)*

                        let len: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();
                        <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode(&len, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
                        actual_writer.write_all(writer)?;
                    }
                    _ => unimplemented!("Unsupported options for NetEncode"),
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

        quote! {
            async fn encode_async<W: tokio::io::AsyncWrite + std::marker::Unpin>(&self, writer: &mut W, opts: &ferrumc_net_codec::encode::NetEncodeOpts) -> ferrumc_net_codec::encode::NetEncodeResult<()> {
                match opts {
                    ferrumc_net_codec::encode::NetEncodeOpts::None => {
                        #(#normal_fields)*
                    }
                    ferrumc_net_codec::encode::NetEncodeOpts::WithLength => {
                        // Write to a buffer first, then write the length and the buffer to the actual writer
                        let actual_writer = writer;
                        let mut writer = Vec::new();
                        let mut writer = &mut writer;

                        #(#length_prefixed_fields)*

                        let len: ferrumc_net_codec::net_types::var_int::VarInt = writer.len().into();
                        <ferrumc_net_codec::net_types::var_int::VarInt as ferrumc_net_codec::encode::NetEncode>::encode_async(&len, actual_writer, &ferrumc_net_codec::encode::NetEncodeOpts::None).await?;
                        // actual_writer.write_all(writer).await?;
                        <W as tokio::io::AsyncWriteExt>::write_all(actual_writer, writer).await?;
                    }
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
        impl #impl_generics ferrumc_net_codec::encode::NetEncode for #struct_name #ty_generics #where_clause {
            #sync_impl
            #async_impl
        }
    };

    TokenStream::from(expanded)
}