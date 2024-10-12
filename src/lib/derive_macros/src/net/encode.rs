use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = if let syn::Data::Struct(data) = &input.data {
        &data.fields
    } else {
        unimplemented!("NetEncode can only be derived for structs");
    };

    let encode_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        quote! {
            // TODO: see if we need to pass options here
            <#field_ty as ferrumc_net_codec::encode::NetEncode>::encode(&self.#field_name, writer, &ferrumc_net_codec::encode::NetEncodeOpts::None)?;
        }
    });

    let length_prefixed_fields = encode_fields.clone();

    let expanded = quote! {
        impl ferrumc_net_codec::encode::NetEncode for #name {
            // TODO: see if we need to use options here.
            fn encode<W: std::io::Write>(&self, writer: &mut W, opts: &ferrumc_net_codec::encode::NetEncodeOpts) -> ferrumc_net_codec::encode::NetEncodeResult<()> {
                match opts {
                    ferrumc_net_codec::encode::NetEncodeOpts::None => {
                        #(#encode_fields)*
                    }
                    ferrumc_net_codec::encode::NetEncodeOpts::WithLength => {
                        // unimplemented!("NetEncodeOpts::WithLength is not yet implemented");
                        let actual_writer = writer;
                        let mut writer = Vec::new();
                        let mut writer = &mut writer;

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
                        let mut writer = Vec::new();
                        let mut writer = &mut writer;

                        // Get compression threshold from config
                        let compression_threshold = ferrumc_config::get_global_config().compression_threshold;

                        #(#length_prefixed_fields)*


                    },
                    _ => unimplemented!("Unsupported options for NetEncode"),
                }

                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
