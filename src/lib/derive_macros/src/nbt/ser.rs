use proc_macro::TokenStream;
use quote::quote;
use crate::nbt::helpers::NbtFieldAttribute;

pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let fields = crate::helpers::get_fields(&input);

    let fields = fields.into_iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let field_name = ident.to_string();

        let mut serialize_name = field_name.clone();

        let attributes = NbtFieldAttribute::from_field(&field);
        
        let mut skip = false;
        
        for attr in attributes {
            match attr {
                NbtFieldAttribute::Rename { new_name } => {
                    serialize_name = new_name;
                }
                NbtFieldAttribute::Skip => {
                    skip = true;
                }
                // We don't care about optionals while serialization
                _ => {}
            }
        }

        if skip {
            return quote! {
                // Skip field
            };
        }
        
        quote! {
            <#ty as ::ferrumc_nbt::NBTSerializable>::serialize(&self.#ident, writer, &::ferrumc_nbt::NBTSerializeOptions::WithHeader(#serialize_name));
        }
    });

    let name = &input.ident;

    let expanded = quote! {
        impl ::ferrumc_nbt::NBTSerializable for #name {
            fn serialize(&self, writer: &mut Vec<u8>, options: &::ferrumc_nbt::NBTSerializeOptions) {
                match options {
                    ::ferrumc_nbt::NBTSerializeOptions::WithHeader(name) => {
                        <u8 as ferrumc_nbt::NBTSerializable>::serialize(&Self::id(), writer, &::ferrumc_nbt::NBTSerializeOptions::None);
                        <&'_ str as ferrumc_nbt::NBTSerializable>::serialize(name, writer, &::ferrumc_nbt::NBTSerializeOptions::None);
                    }
                    ::ferrumc_nbt::NBTSerializeOptions::None => {}
                }

                #(#fields)*

                <u8 as ferrumc_nbt::NBTSerializable>::serialize(&0u8, writer, &::ferrumc_nbt::NBTSerializeOptions::None);
            }

            fn id() -> u8 {
                10
            }
        }

        impl #name {
            pub fn serialize_with_header(&self, writer: &mut Vec<u8>) {
                <#name as ::ferrumc_nbt::NBTSerializable>::serialize(self, writer, &::ferrumc_nbt::NBTSerializeOptions::WithHeader(stringify!(#name)));
            }
        }
    };

    TokenStream::from(expanded)
}
