use crate::helpers::StructInfo;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput, Expr, Fields, Token};

// --- Local Parser for #[packet(id = ...)] ---
struct PacketIdArg {
    id: Option<Expr>,
}

impl Parse for PacketIdArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut id = None;
        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            if key == "id" {
                id = Some(input.parse()?);
            } else {
                // Skip other arguments (like state = "play")
                // We parse as Expr because literals ("string") are valid Exprs
                let _skip: Expr = input.parse()?;
            }

            if !input.is_empty() {
                let _ = input.parse::<Token![,]>();
            }
        }
        Ok(PacketIdArg { id })
    }
}

// --- Snippet Generator ---

// Generate packet ID encoding snippets using the Expression
fn generate_packet_id_snippets(
    packet_id: Option<Expr>,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    if let Some(id_expr) = packet_id {
        // Sync version
        let sync_snippet = quote! {
            <ferrumc_protocol::codec::net_types::var_int::VarInt as ferrumc_protocol::codec::encode::NetEncode>::encode(
                &ferrumc_protocol::codec::net_types::var_int::VarInt::from(#id_expr),
                writer,
                &ferrumc_protocol::codec::encode::NetEncodeOpts::None
            )?;
        };

        // Async version
        let async_snippet = quote! {
            <ferrumc_protocol::codec::net_types::var_int::VarInt as ferrumc_protocol::codec::encode::NetEncode>::encode_async(
                &ferrumc_protocol::codec::net_types::var_int::VarInt::from(#id_expr),
                writer,
                &ferrumc_protocol::codec::encode::NetEncodeOpts::None
            ).await?;
        };

        (sync_snippet, async_snippet)
    } else {
        (quote! {}, quote! {})
    }
}

// --- Field Encoders ---

fn generate_field_encoders(fields: &syn::Fields) -> proc_macro2::TokenStream {
    let encode_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        quote! {
            <#field_ty as ferrumc_protocol::codec::encode::NetEncode>::encode(&self.#field_name, writer, &ferrumc_protocol::codec::encode::NetEncodeOpts::None)?;
        }
    });
    quote! { #(#encode_fields)* }
}

fn generate_async_field_encoders(fields: &syn::Fields) -> proc_macro2::TokenStream {
    let encode_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        quote! {
            <#field_ty as ferrumc_protocol::codec::encode::NetEncode>::encode_async(&self.#field_name, writer, &ferrumc_protocol::codec::encode::NetEncodeOpts::None).await?;
        }
    });
    quote! { #(#encode_fields)* }
}

// --- Enum Encoders ---

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
                            <#field_tys as ferrumc_protocol::codec::encode::NetEncode>::encode(#field_idents, writer, &ferrumc_protocol::codec::encode::NetEncodeOpts::None)?;
                        )*
                    }
                },
                 quote! {
                    Self::#variant_ident { #(#field_idents),* } => {
                        #(
                            <#field_tys as ferrumc_protocol::codec::encode::NetEncode>::encode_async(#field_idents, writer, &ferrumc_protocol::codec::encode::NetEncodeOpts::None).await?;
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
                            <#field_tys as ferrumc_protocol::codec::encode::NetEncode>::encode(#field_names, writer, &ferrumc_protocol::codec::encode::NetEncodeOpts::None)?;
                        )*
                    }
                },
                 quote! {
                    Self::#variant_ident(#(#field_names),*) => {
                        #(
                            <#field_tys as ferrumc_protocol::codec::encode::NetEncode>::encode_async(#field_names, writer, &ferrumc_protocol::codec::encode::NetEncodeOpts::None).await?;
                        )*
                    }
                })
            }
            Fields::Unit => (
                quote! { Self::#variant_ident => {} },
                quote! { Self::#variant_ident => {} }
            ),
        }
    }).unzip::<_, _, Vec<_>, Vec<_>>();

    let (sync_variants, async_variants) = variants;

    (
        quote! { match self { #(#sync_variants)* } },
        quote! { match self { #(#async_variants)* } },
    )
}

// --- 3. Main Derive Function ---

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // 1. Parse the packet ID from attributes
    let mut packet_id_expr = None;
    for attr in &input.attrs {
        if attr.path().is_ident("packet") {
            if let Ok(args) = attr.parse_args::<PacketIdArg>() {
                if let Some(id) = args.id {
                    packet_id_expr = Some(id);
                }
            }
        }
    }

    let (packet_id_snippet, async_packet_id_snippet) = generate_packet_id_snippets(packet_id_expr);

    let (sync_impl, async_impl) = match &input.data {
        syn::Data::Struct(data) => {
            let field_encoders = generate_field_encoders(&data.fields);
            let async_field_encoders = generate_async_field_encoders(&data.fields);

            (
                quote! {
                    fn encode<W: std::io::Write>(&self, writer: &mut W, opts: &ferrumc_protocol::codec::encode::NetEncodeOpts) -> Result<(),  ferrumc_protocol::codec::encode::errors::NetEncodeError> {
                        match opts {
                            ferrumc_protocol::codec::encode::NetEncodeOpts::None => {
                                #packet_id_snippet
                                #field_encoders
                            }
                            ferrumc_protocol::codec::encode::NetEncodeOpts::WithLength => {
                                let actual_writer = writer;
                                let mut writer = Vec::new();
                                let mut writer = &mut writer;

                                #packet_id_snippet
                                #field_encoders

                                let len: ferrumc_protocol::codec::net_types::var_int::VarInt = writer.len().into();
                                <ferrumc_protocol::codec::net_types::var_int::VarInt as ferrumc_protocol::codec::encode::NetEncode>::encode(&len, actual_writer, &ferrumc_protocol::codec::encode::NetEncodeOpts::None)?;
                                actual_writer.write_all(writer)?;
                            }
                            e => unimplemented!("Unsupported option for NetEncode: {:?}", e),
                        }
                        Ok(())
                    }
                },
                quote! {
                    async fn encode_async<W: tokio::io::AsyncWrite + std::marker::Unpin>(&self, writer: &mut W, opts: &ferrumc_protocol::codec::encode::NetEncodeOpts) -> Result<(),  ferrumc_protocol::codec::encode::errors::NetEncodeError> {
                        match opts {
                            ferrumc_protocol::codec::encode::NetEncodeOpts::None => {
                                #async_packet_id_snippet
                                #async_field_encoders
                            }
                            ferrumc_protocol::codec::encode::NetEncodeOpts::WithLength => {
                                let actual_writer = writer;
                                let mut writer = Vec::new();
                                let mut writer = &mut writer;

                                #async_packet_id_snippet
                                #async_field_encoders // FIX: Used correct encoder

                                let len: ferrumc_protocol::codec::net_types::var_int::VarInt = writer.len().into();
                                <ferrumc_protocol::codec::net_types::var_int::VarInt as ferrumc_protocol::codec::encode::NetEncode>::encode_async(&len, actual_writer, &ferrumc_protocol::codec::encode::NetEncodeOpts::None).await?;
                                <W as tokio::io::AsyncWriteExt>::write_all(actual_writer, writer).await?;
                            }
                            e => unimplemented!("Unsupported option for NetEncode: {:?}", e),
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
                    fn encode<W: std::io::Write>(&self, writer: &mut W, opts: &ferrumc_protocol::codec::encode::NetEncodeOpts) -> Result<(),  ferrumc_protocol::codec::encode::errors::NetEncodeError> {
                        match opts {
                            ferrumc_protocol::codec::encode::NetEncodeOpts::None => {
                                #packet_id_snippet
                                #sync_enum_encoder
                            }
                            ferrumc_protocol::codec::encode::NetEncodeOpts::WithLength => {
                                let actual_writer = writer;
                                let mut writer = Vec::new();
                                let mut writer = &mut writer;

                                #packet_id_snippet
                                #sync_enum_encoder

                                let len: ferrumc_protocol::codec::net_types::var_int::VarInt = writer.len().into();
                                <ferrumc_protocol::codec::net_types::var_int::VarInt as ferrumc_protocol::codec::encode::NetEncode>::encode(&len, actual_writer, &ferrumc_protocol::codec::encode::NetEncodeOpts::None)?;
                                actual_writer.write_all(writer)?;
                            }
                            e => unimplemented!("Unsupported option for NetEncode: {:?}", e),
                        }
                        Ok(())
                    }
                },
                quote! {
                    async fn encode_async<W: tokio::io::AsyncWrite + std::marker::Unpin>(&self, writer: &mut W, opts: &ferrumc_protocol::codec::encode::NetEncodeOpts) -> Result<(),  ferrumc_protocol::codec::encode::errors::NetEncodeError> {
                        match opts {
                            ferrumc_protocol::codec::encode::NetEncodeOpts::None => {
                                #async_packet_id_snippet
                                #async_enum_encoder
                            }
                            ferrumc_protocol::codec::encode::NetEncodeOpts::WithLength => {
                                let actual_writer = writer;
                                let mut writer = Vec::new();
                                let mut writer = &mut writer;

                                #async_packet_id_snippet
                                #async_enum_encoder

                                let len: ferrumc_protocol::codec::net_types::var_int::VarInt = writer.len().into();
                                <ferrumc_protocol::codec::net_types::var_int::VarInt as ferrumc_protocol::codec::encode::NetEncode>::encode_async(&len, actual_writer, &ferrumc_protocol::codec::encode::NetEncodeOpts::None).await?;
                                <W as tokio::io::AsyncWriteExt>::write_all(actual_writer, writer).await?;
                            }
                            e => unimplemented!("Unsupported option for NetEncode: {:?}", e),
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
        ..
    } = crate::helpers::extract_struct_info(&input, None);

    TokenStream::from(quote! {
        impl #impl_generics ferrumc_protocol::codec::encode::NetEncode for #struct_name #ty_generics #where_clause {
            #sync_impl
            #async_impl
        }
    })
}
