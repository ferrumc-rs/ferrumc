use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lifetime};
use crate::nbt::helpers::NbtFieldAttribute;

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = crate::helpers::get_fields(&input);
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Determine the lifetime to use in FromNbt
    let nbt_lifetime = if let Some(lt) = input.generics.lifetimes().next() {
        lt.lifetime.clone()
    } else {
        Lifetime::new("'_", Span::call_site())
    };

    let fields_init = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        let mut deserialize_name = field_name.to_string();

        let attributes = NbtFieldAttribute::from_field(field);
        
        let mut optional = false;
        let mut skip = false;

        for attr in attributes {
            match attr {
                NbtFieldAttribute::Rename { new_name } => {
                    deserialize_name = new_name;
                }
                NbtFieldAttribute::Optional => {
                    optional = true;
                }
                NbtFieldAttribute::Skip => {
                    skip = true;
                }
            }
        }

        let elem_name = format!("{} (field: {})", deserialize_name, field_name);
        
        if skip {
            return quote! {
                #field_name: Default::default(),
            };
        }
        
        if optional {
            // Basically checks if the field is present in the element, if not, returns None,
            // else, tries to deserialize the field.
            // The Option<T> deserializer just deserializes the field normally. 
            // Since it expects this code to handle it. :) I love programming.
            return quote! {
                #field_name: element.get(#deserialize_name).map_or(Ok(None), |e| {
                    <#field_ty as ::ferrumc_nbt::FromNbt<#nbt_lifetime>>::from_nbt(tapes, e)
                })?,
            };
        }
        
        quote! {
            #field_name: <#field_ty as ::ferrumc_nbt::FromNbt<#nbt_lifetime>>::from_nbt(
                tapes,
                element.get(#deserialize_name).ok_or({
                    ::ferrumc_nbt::NBTError::ElementNotFound(#elem_name)
                })?
            )?,
        }
    });

    let expanded = quote! {
        impl #impl_generics ::ferrumc_nbt::FromNbt<#nbt_lifetime> for #struct_name #ty_generics #where_clause {
            fn from_nbt(
                tapes: &#nbt_lifetime ::ferrumc_nbt::NbtTape,
                element: &#nbt_lifetime ::ferrumc_nbt::NbtTapeElement
            ) -> ::ferrumc_nbt::Result<Self> {
                Ok(#struct_name {
                    #(#fields_init)*
                })
            }
        }

        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn from_bytes(bytes: &#nbt_lifetime[u8]) -> ::ferrumc_nbt::Result<Self> {
                let mut tape = ::ferrumc_nbt::NbtTape::new(bytes);
                tape.parse();
                let root = tape.root.as_ref()
                    .map(|(_, b)| b)
                    .ok_or(::ferrumc_nbt::NBTError::NoRootTag)?;
                <#struct_name #ty_generics as ::ferrumc_nbt::FromNbt<#nbt_lifetime>>::from_nbt(&tape, root)
            }
        }
    };

    TokenStream::from(expanded)
}
