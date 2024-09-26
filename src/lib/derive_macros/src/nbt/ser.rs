use crate::nbt::helpers::NbtFieldAttribute;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, Fields};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let serialize_impl = match &input.data {
        Data::Struct(data_struct) => {
            let fields = match &data_struct.fields {
                Fields::Named(fields_named) => &fields_named.named,
                Fields::Unnamed(fields_unnamed) => &fields_unnamed.unnamed,
                Fields::Unit => panic!("Unit structs are not supported!"),
            };

            let fields = fields.iter().enumerate().map(|(i, field)| {
                let ident = format!("_{}", i);
                let ident = syn::Ident::new(&ident, field.span());
                let ident = field.ident.as_ref().unwrap_or(&ident);
                let ty = &field.ty;
                let field_name = ident.to_string();

                let mut serialize_name = field_name.clone();

                let attributes = NbtFieldAttribute::from_field(field);

                let mut skip = false;

                for attr in attributes {
                    match attr {
                        NbtFieldAttribute::Rename { new_name } => {
                            serialize_name = new_name;
                        }
                        NbtFieldAttribute::Skip => {
                            skip = true;
                        }
                        _ => {}
                    }
                }

                if skip {
                    return quote! {};
                }

                quote! {
                    <#ty as ::ferrumc_nbt::NBTSerializable>::serialize(&self.#ident, writer, &::ferrumc_nbt::NBTSerializeOptions::WithHeader(#serialize_name));
                }
            });

            quote! {
                #(#fields)*
            }
        }
        Data::Enum(data_enum) => {
            let variants = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;

                let serialize_fields = match &variant.fields {
                    Fields::Named(fields_named) => {
                        let fields = fields_named.named.iter().map(|field| {
                            let ident = field.ident.as_ref().unwrap();
                            let ty = &field.ty;
                            let field_name = ident.to_string();

                            let mut serialize_name = field_name.clone();

                            let attributes = NbtFieldAttribute::from_field(field);

                            let mut skip = false;

                            for attr in attributes {
                                match attr {
                                    NbtFieldAttribute::Rename { new_name } => {
                                        serialize_name = new_name;
                                    }
                                    NbtFieldAttribute::Skip => {
                                        skip = true;
                                    }
                                    _ => {}
                                }
                            }

                            if skip {
                                return quote! {};
                            }

                            quote! {
                                <#ty as ::ferrumc_nbt::NBTSerializable>::serialize(#ident, writer, &::ferrumc_nbt::NBTSerializeOptions::WithHeader(#serialize_name));
                            }
                        });

                        let field_idents = fields_named.named.iter().map(|f| &f.ident);

                        quote! {
                            { #(#field_idents),* } => {
                                #(#fields)*
                            }
                        }
                    }
                    Fields::Unnamed(fields_unnamed) => {
                        let fields = fields_unnamed.unnamed.iter().enumerate().map(|(i, field)| {
                            let ident = syn::Ident::new(&format!("_{}", i), field.span());
                            let ty = &field.ty;

                            quote! {
                                <#ty as ::ferrumc_nbt::NBTSerializable>::serialize(#ident, writer, &::ferrumc_nbt::NBTSerializeOptions::None);
                            }
                        });

                        let idents = (0..fields_unnamed.unnamed.len()).map(|i| syn::Ident::new(&format!("_{}", i), Span::call_site()));

                        quote! {
                            (#(#idents),*) => {
                                #(#fields)*
                            }
                        }
                    }
                    Fields::Unit => quote! {
                        => {}
                    },
                };

                quote! {
                    Self::#variant_name #serialize_fields
                }
            });

            quote! {
                match self {
                    #(#variants),*
                }
            }
        }
        Data::Union(_) => panic!("Unions are not supported!"),
    };

    let expanded = quote! {
        impl #impl_generics ::ferrumc_nbt::NBTSerializable for #name #ty_generics #where_clause {
            fn serialize(&self, writer: &mut Vec<u8>, options: &::ferrumc_nbt::NBTSerializeOptions) {
                match options {
                    ::ferrumc_nbt::NBTSerializeOptions::WithHeader(name) => {
                        <u8 as ::ferrumc_nbt::NBTSerializable>::serialize(&Self::id(), writer, &::ferrumc_nbt::NBTSerializeOptions::None);
                        <&'_ str as ::ferrumc_nbt::NBTSerializable>::serialize(name, writer, &::ferrumc_nbt::NBTSerializeOptions::None);
                    }
                    ::ferrumc_nbt::NBTSerializeOptions::None => {}
                }

                #serialize_impl

                <u8 as ::ferrumc_nbt::NBTSerializable>::serialize(&0u8, writer, &::ferrumc_nbt::NBTSerializeOptions::None);
            }

            fn id() -> u8 {
                10
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            pub fn serialize_with_header(&self) -> Vec<u8> {
                let mut writer = Vec::new();
                <#name #ty_generics as ::ferrumc_nbt::NBTSerializable>::serialize(self, &mut writer, &::ferrumc_nbt::NBTSerializeOptions::WithHeader(stringify!(#name)));
                writer
            }
        }
    };

    TokenStream::from(expanded)
}
