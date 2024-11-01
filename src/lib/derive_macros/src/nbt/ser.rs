use crate::nbt::helpers::{NbtFieldAttribute, Cases};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, Fields, Expr, LitStr};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let input_attributes = NbtFieldAttribute::from_input(&input);

    let mut tag_type = 10u8;

    let serialize_impl = match &input.data {
        Data::Struct(data_struct) => {
            let fields = match &data_struct.fields {
                Fields::Named(fields_named) => &fields_named.named,
                Fields::Unnamed(fields_unnamed) => &fields_unnamed.unnamed,
                Fields::Unit => panic!("Unit structs are not supported!"),
            };

            let mut variant_case: Cases = Cases::Normal;

            for attr in &input_attributes {
                match attr {
                    NbtFieldAttribute::RenameAll { case } => {
                        variant_case = case.clone();
                    },
                    NbtFieldAttribute::TagType { tag } => { tag_type = *tag; },
                    _ => {}
                }
            }


            let fields = fields.iter().enumerate().map(|(i, field)| {
                let ident = format!("_{}", i);
                let ident = syn::Ident::new(&ident, field.span());
                let ident = field.ident.as_ref().unwrap_or(&ident);
                let ty = &field.ty;
                let field_name = ident.to_string();

                let mut serialize_name = field_name.clone();

                let attributes = NbtFieldAttribute::from_field(field);

                let mut skip = false;
                let mut skip_if: Option<Expr> = None;
                let mut flatten = false;

                for attr in attributes {
                    match attr {
                        NbtFieldAttribute::Rename { new_name } => {
                            serialize_name = new_name;
                        }
                        NbtFieldAttribute::Skip => {
                            skip = true;
                        }
                        NbtFieldAttribute::Flatten => {
                            flatten = true;
                        }
                        NbtFieldAttribute::SkipIf { condition } => {
                            if skip {
                                return quote! {};
                            }

                            let condition = syn::parse_str::<Expr>(&condition).unwrap();
                            skip_if = Some(condition);
                        }
                        _ => {}
                    }
                }

                serialize_name = variant_case.transform(serialize_name);

                if skip {
                    return quote! {};
                }

                if flatten {
                    return quote! {
                        <#ty as ::ferrumc_nbt::NBTSerializable>::serialize(&self.#ident, writer, &::ferrumc_nbt::NBTSerializeOptions::Flatten);
                    };
                }

                if let Some(condition) = skip_if {
                    quote! {
                        if !#condition (&self.#ident) {
                            <#ty as ::ferrumc_nbt::NBTSerializable>::serialize(&self.#ident, writer, &::ferrumc_nbt::NBTSerializeOptions::WithHeader(#serialize_name));
                        }
                    }
                } else {
                    quote! {
                        <#ty as ::ferrumc_nbt::NBTSerializable>::serialize(&self.#ident, writer, &::ferrumc_nbt::NBTSerializeOptions::WithHeader(#serialize_name));
                    }
                }
            });

            quote! {
                #(#fields)*
            }
        }
        Data::Enum(data_enum) => {
            let variants = data_enum.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let mut variant_name = variant_ident.to_string();

                let mut variant_case: Cases = Cases::Normal; // will only be used if tagged
                let mut tagged: Option<LitStr> = None;
                let mut untagged = false;
                let mut variant_content = LitStr::new("content", Span::call_site()); // will only be used if tagged

                for attr in &input_attributes {
                    match attr {
                        NbtFieldAttribute::RenameAll { case } => {
                            variant_case = case.clone();
                        },
                        NbtFieldAttribute::Tag { tag } => {
                            tagged = Some(LitStr::new(tag.as_str(), Span::call_site()));
                            if tag.as_str() == "untagged" {
                                untagged = true;
                            }
                        }
                        NbtFieldAttribute::Content { content } => {
                            variant_content = LitStr::new(content.as_str(), Span::call_site());
                        }
                        NbtFieldAttribute::TagType { tag } => { tag_type = *tag; },
                        _ => {}
                    }
                }

                for attr in NbtFieldAttribute::from_variant(&variant) {
                    match attr {
                        NbtFieldAttribute::Rename { new_name } => {
                            variant_name = new_name.clone();
                        },
                        _ => {},
                    }
                }

                let tag_name = variant_case.transform(variant_name);

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

                        let fields = quote! { #(#fields)* };
                        let tagged = if let Some(tag) = tagged {
                            if untagged { fields }
                            else {
                                quote! {
                                    <&'_ str as ::ferrumc_nbt::NBTSerializable>::serialize(&#tag_name, writer, &::ferrumc_nbt::NBTSerializeOptions::WithHeader(#tag));
                                    <u8 as ::ferrumc_nbt::NBTSerializable>::serialize(&10, writer, &::ferrumc_nbt::NBTSerializeOptions::None);
                                    <&'_ str as ::ferrumc_nbt::NBTSerializable>::serialize(&#variant_content, writer, &::ferrumc_nbt::NBTSerializeOptions::None);
                                    #fields
                                    <u8 as ::ferrumc_nbt::NBTSerializable>::serialize(&0u8, writer, &::ferrumc_nbt::NBTSerializeOptions::None);
                                }
                            }
                        } else { fields };

                        quote! {
                            { #(#field_idents),* } => {
                                #tagged
                            }
                        }
                    }
                    Fields::Unnamed(fields_unnamed) => {
                        let fields = fields_unnamed.unnamed.iter().enumerate().map(|(i, field)| {
                            let ident = syn::Ident::new(&format!("_{}", i), field.span());
                            let ty = &field.ty;

                            if !untagged && tagged.is_some() {
                                if fields_unnamed.unnamed.len() == 1 {
                                    quote! {
                                        <#ty as ::ferrumc_nbt::NBTSerializable>::serialize(#ident, writer, &::ferrumc_nbt::NBTSerializeOptions::WithHeader(#variant_content));
                                    }
                                } else {
                                    quote! { unimplemented!(); }
                                }
                            } else {
                                quote! {
                                    <#ty as ::ferrumc_nbt::NBTSerializable>::serialize(#ident, writer, &::ferrumc_nbt::NBTSerializeOptions::None);
                                }
                            }
                        });

                        let idents = (0..fields_unnamed.unnamed.len()).map(|i| syn::Ident::new(&format!("_{}", i), Span::call_site()));

                        let fields = quote! { #(#fields)* };
                        let tagged = if let Some(tag) = tagged {
                            if untagged { fields }
                            else {
                                quote! {
                                    <&'_ str as ::ferrumc_nbt::NBTSerializable>::serialize(&#tag_name, writer, &::ferrumc_nbt::NBTSerializeOptions::WithHeader(#tag));
                                    #fields
                                }
                            }
                        } else { fields };

                        quote! {
                            (#(#idents),*) => {
                                #tagged
                            }
                        }
                    }
                    Fields::Unit => match tagged {
                        Some(tag) => {
                            if untagged {
                                quote! {
                                    => {
                                        <&'_ str as ::ferrumc_nbt::NBTSerializable>::serialize(&#tag_name, writer, &::ferrumc_nbt::NBTSerializeOptions::None);
                                    }
                                }
                            } else {
                                quote! {
                                    => {
                                        <&'_ str as ::ferrumc_nbt::NBTSerializable>::serialize(&#tag_name, writer, &::ferrumc_nbt::NBTSerializeOptions::WithHeader(#tag));
                                    }
                                }
                            }
                        },
                        None => quote! {
                             => {}
                        },
                    }
                };

                quote! {
                    Self::#variant_ident #serialize_fields
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
                    ::ferrumc_nbt::NBTSerializeOptions::Network => {
                        <u8 as ::ferrumc_nbt::NBTSerializable>::serialize(&Self::id(), writer, &::ferrumc_nbt::NBTSerializeOptions::None);
                    }
                    ::ferrumc_nbt::NBTSerializeOptions::None => {}
                    ::ferrumc_nbt::NBTSerializeOptions::Flatten => {}
                }

                #serialize_impl

                if options != &::ferrumc_nbt::NBTSerializeOptions::Flatten && Self::id() == 10 {
                    <u8 as ::ferrumc_nbt::NBTSerializable>::serialize(&0u8, writer, &::ferrumc_nbt::NBTSerializeOptions::None);
                }
            }

            fn id() -> u8 {
                #tag_type
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            pub fn serialize_with_header(&self) -> Vec<u8> {
                let mut writer = Vec::new();
                <#name #ty_generics as ::ferrumc_nbt::NBTSerializable>::serialize(self, &mut writer, &::ferrumc_nbt::NBTSerializeOptions::WithHeader(stringify!(#name)));
                writer
            }
        }
        
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn serialize_as_network(&self) -> Vec<u8> {
                let mut writer = Vec::new();
                
                <#name #ty_generics as ::ferrumc_nbt::NBTSerializable>::serialize(self, &mut writer, &::ferrumc_nbt::NBTSerializeOptions::Network);
                /*<u8 as ::ferrumc_nbt::NBTSerializable>::serialize(&<Self as ::ferrumc_nbt::NBTSerializable>::id(), &mut writer, &::ferrumc_nbt::NBTSerializeOptions::None);
                <#name #ty_generics as ::ferrumc_nbt::NBTSerializable>::serialize(self, &mut writer, &::ferrumc_nbt::NBTSerializeOptions::None);*/
                
                writer
            }
        }
    };

    TokenStream::from(expanded)
}
