use quote::{format_ident, quote};
use syn::{DeriveInput, Field, Generics, Lifetime, parse_macro_input, Data};

use proc_macro::TokenStream;

struct FieldAttribs {
    field_name: syn::Ident,
    field_type: syn::Type,
    default_value: Option<syn::Expr>,
    raw_bytes: Option<RawBytes>,
    prepend_length: bool,
    lifetime: Option<Lifetime>,
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
        prepend_length: false,
        lifetime: None,
    };

    if let syn::Type::Reference(ref_type) = &field.ty {
        field_attrib.lifetime = ref_type.lifetime.clone();
    }

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
            } else if meta.path.is_ident("prepend_length") {
                field_attrib.prepend_length = meta.value()?.parse::<syn::LitBool>()?.value;
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

    let prepend_length = field_attrib.prepend_length
        || field_attrib
            .raw_bytes
            .as_ref()
            .map_or(false, |rb| rb.prepend_length);

    let mut statement = quote! {
        let mut #cursor = std::io::Cursor::new(Vec::new());
    };

    if prepend_length {
        statement = quote! {
            #statement
            let #len = self.#field_name.len();
            let #len = ferrumc_codec::network_types::varint::VarInt::new(#len as i32);
            #len.net_encode(&mut #cursor).await?;
        };
    }

    if field_attrib.raw_bytes.is_some() {
        quote! {
            #statement
            tokio::io::AsyncWriteExt::write_all(&mut #cursor, &self.#field_name).await?;
            let mut #bytes = #cursor.into_inner();
            tokio::io::AsyncWriteExt::write_all(bytes, &#bytes).await?;
        }
    } else {
        if prepend_length {
            quote! {
                #statement
                self.#field_name.net_encode(&mut #cursor).await?;
                let mut #bytes = #cursor.into_inner();
                tokio::io::AsyncWriteExt::write_all(bytes, &#bytes).await?;
            }
        } else {
            quote! {
                self.#field_name.net_encode(bytes).await?;
            }
        }
    }
}

fn generate_encode_impl(
    name: &syn::Ident,
    generics: Generics,
    field_statements: &[proc_macro2::TokenStream],
    is_packet_type: bool,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    if is_packet_type {
        quote! {
            impl #impl_generics ferrumc_codec::enc::NetEncode for #name #ty_generics #where_clause {
                async fn net_encode<T>(&self, bytes_out: &mut T) -> std::result::Result<(), ferrumc_codec::error::CodecError>
                    where T: tokio::io::AsyncWrite + std::marker::Unpin
                {
                    use tokio::io::AsyncWriteExt;

                    let mut bytes_ = std::io::Cursor::new(Vec::new());
                    let mut bytes = &mut bytes_;

                    #(#field_statements)*

                    let __packet_data = bytes_.into_inner();
                    let __length = ferrumc_codec::network_types::varint::VarInt::new(__packet_data.len() as i32);
                    let mut __cursor = std::io::Cursor::new(Vec::new());
                    __length.net_encode(&mut __cursor).await?;
                    __cursor.write_all(&__packet_data).await?;
                    let __encoded = __cursor.into_inner();
                    bytes_out.write_all(&__encoded).await?;

                    Ok(())
                }
            }
        }
    } else {
        quote! {
            impl #impl_generics ferrumc_codec::enc::NetEncode for #name #ty_generics #where_clause {
                async fn net_encode<T>(&self, bytes: &mut T) -> std::result::Result<(), ferrumc_codec::error::CodecError>
                    where T: tokio::io::AsyncWrite + std::marker::Unpin
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
    generics: &syn::Generics,
    field_attribs: &[FieldAttribs],
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let non_default_fields: Vec<&FieldAttribs> = field_attribs
        .iter()
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
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn new_auto(#(#non_default_fields_params)*) -> Self {
                Self {
                    #(#non_default_fields_names)*
                    #(#default_field_statements)*
                }
            }
        }
    }
}
/*pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = input.generics;

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
        let modified_constructor = generate_modified_constructor(name, &generics, &field_attribs);
        constructor = modified_constructor;
    }

    let is_packet_type = field_attribs
        .iter()
        .any(|attr| attr.field_name == "packet_id");

    let field_statements: Vec<proc_macro2::TokenStream> = field_attribs
        .iter()
        .map(generate_field_encode_statement)
        .collect();

    let expanded = generate_encode_impl(name, generics, &field_statements, is_packet_type);

    let final_output = quote! {
        #constructor
        #expanded
    };

    TokenStream::from(final_output)
}
*/

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = input.generics;

    let expanded = match &input.data {
        Data::Struct(data_struct) => {
            derive_for_struct(name, generics, data_struct)
        }
        Data::Enum(data_enum) => {
            derive_for_enum(name, generics, data_enum)
        }
        _ => panic!("Only structs and enums are supported"),
    };

    TokenStream::from(expanded)
}

fn derive_for_enum(
    name: &syn::Ident,
    generics: Generics,
    data_enum: &syn::DataEnum,
) -> proc_macro2::TokenStream {
    let variants = &data_enum.variants;

    let match_arms: Vec<proc_macro2::TokenStream> = variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            match &variant.fields {
                syn::Fields::Unnamed(fields_unnamed) => {
                    let field_patterns: Vec<_> = fields_unnamed
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(i, _)| format_ident!("field{}", i))
                        .collect();

                    let encode_calls: Vec<_> = field_patterns
                        .iter()
                        .map(|fp| quote! { #fp.net_encode(bytes).await?; })
                        .collect();

                    quote! {
                        Self::#variant_name( #(ref #field_patterns),* ) => {
                            #(#encode_calls)*
                            Ok(())
                        }
                    }
                }
                syn::Fields::Named(fields_named) => {
                    let field_names: Vec<_> = fields_named
                        .named
                        .iter()
                        .map(|f| f.ident.as_ref().unwrap())
                        .collect();

                    let encode_calls: Vec<_> = field_names
                        .iter()
                        .map(|fnm| quote! { #fnm.net_encode(bytes).await?; })
                        .collect();

                    quote! {
                        Self::#variant_name { #(ref #field_names),* } => {
                            #(#encode_calls)*
                            Ok(())
                        }
                    }
                }
                syn::Fields::Unit => {
                    quote! {
                        Self::#variant_name => {
                            Ok(())
                        }
                    }
                }
            }
        })
        .collect();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ferrumc_codec::enc::NetEncode for #name #ty_generics #where_clause {
            async fn net_encode<T>(&self, bytes: &mut T) -> std::result::Result<(), ferrumc_codec::error::CodecError>
                where T: tokio::io::AsyncWrite + std::marker::Unpin
            {
                use tokio::io::AsyncWriteExt;
                match self {
                    #(#match_arms),*
                }
            }
        }
    }
}
fn derive_for_struct(
    name: &syn::Ident,
    generics: Generics,
    data_struct: &syn::DataStruct,
) -> proc_macro2::TokenStream {
    let fields = match &data_struct.fields {
        syn::Fields::Named(fields) => fields,
        _ => panic!("Only structs with named fields are supported"),
    };

    let field_attribs: Vec<FieldAttribs> =
        fields.named.iter().map(parse_field_attributes).collect();

    let should_generate_modified_constructor = field_attribs
        .iter()
        .any(|attr| attr.default_value.is_some());
    let constructor = if should_generate_modified_constructor {
        generate_modified_constructor(name, &generics, &field_attribs)
    } else {
        quote! {}
    };

    let is_packet_type = field_attribs
        .iter()
        .any(|attr| attr.field_name == "packet_id");

    let field_statements: Vec<proc_macro2::TokenStream> = field_attribs
        .iter()
        .map(generate_field_encode_statement)
        .collect();

    let expanded = generate_encode_impl(name, generics, &field_statements, is_packet_type);

    quote! {
        #constructor
        #expanded
    }
}
